use clap::Parser;
use ida_c_splitter::{signature_parser, type_parser};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::path::{Path, PathBuf};

/// Split IDA Hex-Rays decompiler C exports into a navigable file tree
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the C++ implementation file from IDA
    input: PathBuf,

    /// Path to the C header file from IDA (optional)
    #[arg(long)]
    header: Option<PathBuf>,

    /// Output directory for split files
    #[arg(short, long, default_value = "output")]
    output: PathBuf,
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let args = Args::parse();
    let output = &args.output;

    std::fs::remove_dir_all(output).ok();
    std::fs::create_dir_all(output).expect("Failed to create output directory");
    tracing::info!("Output directory prepared: {}", output.display());

    let type_hierarchy = if let Some(header_path) = &args.header {
        tracing::info!("Processing header: {}", header_path.display());
        let file = std::fs::File::open(header_path).expect("Failed to open header");
        let mmap = unsafe { memmap2::Mmap::map(&file).expect("Failed to mmap header") };
        let header = parse_header(&mmap);
        let hierarchy = build_type_hierarchy(&header);
        write_headers(&header, &mmap, output, &hierarchy);
        hierarchy
    } else {
        HashMap::new()
    };

    {
        tracing::info!("Processing implementation: {}", args.input.display());
        let file = std::fs::File::open(&args.input).expect("Failed to open implementation");
        let mmap = unsafe { memmap2::Mmap::map(&file).expect("Failed to mmap implementation") };
        let cpp = parse_cpp(&mmap);
        write_implementations(&cpp.functions, &mmap, output, &type_hierarchy);

        tracing::info!("Writing data declarations");
        std::fs::write(output.join("_data.cpp"), &mmap[cpp.data_declarations])
            .expect("Failed to write data declarations");
    }

    tracing::info!("Complete: {}", output.display());
}

fn sanitize_filename(name: &str) -> String {
    let mut sanitized = name
        .replace('<', "(")
        .replace('>', ")")
        .replace(':', "_")
        .replace('"', "'")
        .replace(['/', '\\', '|', '?', '*'], "_");

    let reserved = [
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
        "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];

    if reserved.contains(&sanitized.to_uppercase().as_str()) {
        sanitized = format!("_{}", sanitized);
    }

    sanitized = sanitized
        .trim_end_matches('.')
        .trim_end_matches(' ')
        .to_string();

    if sanitized.is_empty() {
        sanitized = "_".to_string();
    }

    sanitized
}

#[derive(Default, Debug)]
struct Header {
    empty_defs: Vec<String>,
    types: Vec<(Range<usize>, String)>,
}

fn parse_header(mmap: &[u8]) -> Header {
    let empty_defs = unsafe {
        std::str::from_utf8_unchecked(mmap)
            .lines()
            .skip_while(|l| {
                !(l.starts_with("union ") || l.starts_with("struct ") || l.starts_with("enum "))
            })
            .take_while(|l| !(l.trim().is_empty() || l.starts_with("/")))
            .map(|l| l.trim().to_string())
            .collect()
    };

    let type_starts: Vec<usize> = memchr::memmem::find_iter(mmap, b"/* ").collect();
    let types = type_starts
        .iter()
        .enumerate()
        .map(|(idx, &start)| {
            let end = type_starts.get(idx + 1).copied().unwrap_or(mmap.len());
            let slice = &mmap[start..end];
            let def = unsafe {
                std::str::from_utf8_unchecked(slice)
                    .lines()
                    .find(|l| {
                        !(l.starts_with("//") || l.starts_with("/*") || l.starts_with("#pragma"))
                    })
                    .unwrap()
            };
            (start..start + slice.len(), def.to_string())
        })
        .collect();

    Header { empty_defs, types }
}

/// Builds a map of type names to their root type segments.
/// Nested types map to their outermost parent (e.g., A::B::C all map to A).
fn build_type_hierarchy(header: &Header) -> HashMap<String, Vec<String>> {
    let all_types: HashMap<String, Vec<String>> = header
        .types
        .iter()
        .map(|(_, def)| {
            let segments = type_parser::parse_type(def);
            (segments.join("::"), segments)
        })
        .collect();

    let mut roots = HashMap::new();
    for (name, segments) in &all_types {
        let mut root = segments.clone();
        for i in (1..segments.len()).rev() {
            let parent = segments[..i].join("::");
            if all_types.contains_key(&parent) {
                root = segments[..i].to_vec();
            } else {
                break;
            }
        }
        roots.insert(name.clone(), root);
    }
    roots
}

fn write_headers(
    header: &Header,
    mmap: &[u8],
    output: &Path,
    hierarchy: &HashMap<String, Vec<String>>,
) {
    let type_segments: HashSet<Vec<String>> = header
        .types
        .iter()
        .map(|(_, def)| type_parser::parse_type(def))
        .collect();

    let items = header
        .empty_defs
        .iter()
        .filter_map(|def| {
            let segments = type_parser::parse_type(def);
            (!type_segments.contains(&segments)).then_some((segments, def.as_str()))
        })
        .chain(header.types.iter().map(|(range, def)| {
            let segments = type_parser::parse_type(def);
            let body = unsafe { std::str::from_utf8_unchecked(&mmap[range.clone()]) };
            (segments, body)
        }));

    let mut groups: HashMap<PathBuf, Vec<String>> = HashMap::new();

    for (segments, body) in items {
        let name = segments.join("::");
        let effective = hierarchy.get(&name).unwrap_or(&segments);

        if effective.is_empty() {
            panic!("Empty segments for: {:?}", body);
        }

        let mut path = output.to_path_buf();
        if effective.len() > 1 {
            for seg in &effective[..effective.len() - 1] {
                path.push(sanitize_filename(strip_template_params(seg)));
            }
        }

        let filename = sanitize_filename(strip_template_params(&effective[effective.len() - 1]));
        let filename = filename.strip_suffix("_vtbl").unwrap_or(&filename);
        path.push(format!("{}.h", filename));

        groups.entry(path).or_default().push(body.to_string());
    }

    tracing::info!("Writing {} header files", groups.len());

    groups.par_iter().for_each(|(path, bodies)| {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("Failed to create directories");
        }
        std::fs::write(path, bodies.join("\n"))
            .unwrap_or_else(|e| panic!("Failed to write {:?}: {}", path, e));
        tracing::debug!("Wrote: {}", path.display());
    });
}

#[derive(Default, Debug)]
struct Cpp {
    data_declarations: Range<usize>,
    functions: Vec<Function>,
}

#[derive(Debug, Clone)]
struct Function {
    offset: usize,
    segments: Vec<String>,
}

fn parse_cpp(mmap: &[u8]) -> Cpp {
    let func_decls = memchr::memmem::find(mmap, b"// Function declarations")
        .expect("Missing function declarations marker");
    let data_decls = memchr::memmem::find(&mmap[func_decls..], b"// Data declarations")
        .expect("Missing data declarations marker")
        + func_decls;

    tracing::debug!("Function declarations at: {}", func_decls);
    tracing::debug!("Data declarations at: {}", data_decls);

    let raw: Vec<(usize, String)> = memchr::memmem::find_iter(&mmap[data_decls..], b"//----- (")
        .map(|rel_offset| {
            let offset = rel_offset + data_decls;
            let rest = unsafe { std::str::from_utf8_unchecked(&mmap[offset..]) };
            let sig = rest
                .lines()
                .find(|line| !line.starts_with("//"))
                .expect("Missing signature");
            let sig = &sig[..sig.rfind('(').expect("Missing opening paren")];
            (offset, sig.to_string())
        })
        .collect();

    tracing::info!("Found {} functions", raw.len());

    let functions: Vec<Function> = raw
        .par_iter()
        .map(|(offset, sig)| Function {
            offset: *offset,
            segments: signature_parser::parse_signature(sig),
        })
        .collect();

    Cpp {
        data_declarations: data_decls..functions[0].offset,
        functions,
    }
}

fn write_implementations(
    functions: &[Function],
    mmap: &[u8],
    output: &Path,
    hierarchy: &HashMap<String, Vec<String>>,
) {
    let mut groups: HashMap<PathBuf, Vec<usize>> = HashMap::new();

    for (idx, func) in functions.iter().enumerate() {
        let path = if func.segments.len() < 2 {
            output.join("global.cpp")
        } else {
            let scope = func.segments[..func.segments.len() - 1].join("::");
            let effective = hierarchy
                .get(&scope)
                .map(|v| v.as_slice())
                .unwrap_or(&func.segments[..func.segments.len() - 1]);

            let mut path = output.to_path_buf();
            if effective.len() > 1 {
                for seg in &effective[..effective.len() - 1] {
                    path.push(sanitize_filename(strip_template_params(seg)));
                }
            }

            let filename =
                sanitize_filename(strip_template_params(&effective[effective.len() - 1]));
            path.push(format!("{}.cpp", filename));
            path
        };
        groups.entry(path).or_default().push(idx);
    }

    tracing::info!("Writing {} implementation files", groups.len());

    groups.par_iter().for_each(|(path, indices)| {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("Failed to create directories");
        }

        let content: Vec<u8> = indices
            .iter()
            .flat_map(|&idx| {
                let func = &functions[idx];
                let end = functions
                    .get(idx + 1)
                    .map(|f| f.offset)
                    .unwrap_or(mmap.len());
                &mmap[func.offset..end]
            })
            .copied()
            .collect();

        std::fs::write(path, content)
            .unwrap_or_else(|e| panic!("Failed to write {:?}: {}", path, e));
        tracing::debug!("Wrote: {}", path.display());
    });
}

fn strip_template_params(s: &str) -> &str {
    s.find('<').map(|pos| &s[..pos]).unwrap_or(s)
}
