use clap::Parser;
use rayon::prelude::*;
use std::collections::HashMap;
use std::ops::Range;
use std::path::{Path, PathBuf};

mod signature_parser;

/// Split IDA Hex-Rays decompiler C exports into a navigable file tree
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the IDA C export file
    input: PathBuf,

    /// Path to the IDA C header file
    #[arg(long)]
    header: Option<PathBuf>,

    /// Output directory for the split files
    #[arg(short, long, default_value = "output")]
    output: PathBuf,
}

#[derive(Debug, Clone)]
struct Function {
    offset: usize,
    segments: Vec<String>,
}

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let args = Args::parse();
    let output = &args.output;

    // Prepare output directory
    std::fs::remove_dir_all(output).ok();
    std::fs::create_dir_all(output).expect("Failed to create output directory");
    tracing::info!("Output directory prepared: {}", output.display());

    // Handle header file
    if let Some(header) = &args.header {
        tracing::info!("Opening header file: {}", header.display());
        let header_file = std::fs::File::open(header).expect("Failed to open file");
        let header_mmap = unsafe { memmap2::Mmap::map(&header_file).expect("Failed to mmap file") };

        // Parse header file
        let header_file = parse_header_file(&header_mmap);
        std::fs::write(
            output.join("__header.h"),
            header_file
                .empty_defs
                .into_iter()
                .map(|s| s.strip_suffix(";").map(|s| s.to_string()).unwrap_or(s))
                .chain(header_file.types.into_iter().map(|p| p.1))
                .collect::<Vec<String>>()
                .join("\n"),
        )
        .unwrap();
    }

    // Handle cpp file
    {
        tracing::info!("Opening cpp file: {}", args.input.display());
        let cpp_file = std::fs::File::open(&args.input).expect("Failed to open file");
        let cpp_mmap = unsafe { memmap2::Mmap::map(&cpp_file).expect("Failed to mmap file") };

        // Parse cpp file
        let cpp_file = parse_cpp_file(&cpp_mmap);

        // Create file tree hierarchy
        create_file_tree(&cpp_file.functions, &cpp_mmap, output);

        // Write data declarations
        tracing::info!("Writing data declarations");
        std::fs::write(
            output.join("__data_declarations.cpp"),
            &cpp_mmap[cpp_file.data_declarations],
        )
        .expect("Failed to write __data_declarations.cpp");
    }

    tracing::info!("Output produced: {}", output.display());
}

/// Sanitizes a filename for Windows compatibility
fn sanitize_filename(name: &str) -> String {
    // Invalid Windows characters: < > : " / \ | ? *
    let mut sanitized = name
        .replace('<', "(")
        .replace('>', ")")
        .replace(':', "_")
        .replace('"', "'")
        .replace(['/', '\\', '|', '?', '*'], "_");

    // Handle reserved Windows names
    let reserved_names = [
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
        "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];

    if reserved_names.contains(&sanitized.to_uppercase().as_str()) {
        sanitized = format!("_{}", sanitized);
    }

    // Remove trailing dots and spaces (not allowed on Windows)
    sanitized = sanitized
        .trim_end_matches('.')
        .trim_end_matches(' ')
        .to_string();

    // Ensure not empty
    if sanitized.is_empty() {
        sanitized = "_".to_string();
    }

    sanitized
}

#[derive(Default)]
struct HeaderFile {
    empty_defs: Vec<String>,
    types: Vec<(Range<usize>, String)>,
}
fn parse_header_file(mmap: &[u8]) -> HeaderFile {
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
    let mut output = HeaderFile {
        empty_defs,
        types: vec![],
    };

    let raw_type_starts: Vec<usize> = memchr::memmem::find_iter(mmap, b"/* ").collect();
    for (idx, &type_start) in raw_type_starts.iter().enumerate() {
        let type_slice =
            &mmap[type_start..raw_type_starts.get(idx + 1).copied().unwrap_or(mmap.len())];
        let type_def = unsafe {
            std::str::from_utf8_unchecked(type_slice)
                .lines()
                .find(|l| !(l.starts_with("//") || l.starts_with("/*") || l.starts_with("#pragma")))
                .unwrap()
        };

        output.types.push((
            type_start..type_start + type_def.len(),
            type_def.to_string(),
        ));
    }

    output
}

struct CppFile {
    data_declarations: Range<usize>,
    functions: Vec<Function>,
}
fn parse_cpp_file(mmap: &[u8]) -> CppFile {
    // Find section markers
    let function_declarations = memchr::memmem::find(mmap, b"// Function declarations")
        .expect("Failed to find function declarations");
    tracing::debug!(
        "Found function declarations at offset: {}",
        function_declarations
    );

    let data_declarations =
        memchr::memmem::find(&mmap[function_declarations..], b"// Data declarations")
            .expect("Failed to find data declarations")
            + function_declarations;
    tracing::debug!("Found data declarations at offset: {}", data_declarations);

    // Collect offsets and signatures first
    let raw_functions: Vec<(usize, String)> =
        memchr::memmem::find_iter(&mmap[data_declarations..], b"//----- (")
            .map(|offset_from_dd| {
                let offset = offset_from_dd + data_declarations;

                let remaining_file = unsafe { std::str::from_utf8_unchecked(&mmap[offset..]) };
                let signature = remaining_file
                    .lines()
                    .find(|line| !line.starts_with("//"))
                    .expect("Failed to find signature");
                let signature = &signature[..signature
                    .rfind('(')
                    .expect("Failed to find opening parenthesis")];

                (offset, signature.to_string())
            })
            .collect();

    tracing::info!("Found {} function definitions", raw_functions.len());

    // Parse segments in parallel
    let functions: Vec<Function> = raw_functions
        .par_iter()
        .map(|(offset, signature)| {
            let segments = signature_parser::parse_signature(signature);
            Function {
                offset: *offset,
                segments,
            }
        })
        .collect();

    CppFile {
        data_declarations: data_declarations..functions[0].offset,
        functions,
    }
}

/// Strips template parameters from a segment name
/// e.g., "unique_ptr<CResourceLoader>" -> "unique_ptr"
fn strip_template_params(segment: &str) -> &str {
    segment
        .find('<')
        .map(|pos| &segment[..pos])
        .unwrap_or(segment)
}

/// Creates the file tree hierarchy from function definitions
fn create_file_tree(functions: &[Function], mmap: &[u8], output_dir: &Path) {
    // Group functions by their target file path
    let mut file_groups: HashMap<PathBuf, Vec<usize>> = HashMap::new();

    for (idx, func) in functions.iter().enumerate() {
        let path = if func.segments.len() < 2 {
            // Functions with < 2 segments go into global.cpp
            output_dir.join("global.cpp")
        } else {
            let mut path = output_dir.to_path_buf();
            // Everything up to n-2 segments become folders (all but the last two)
            // Strip template parameters to group template instantiations together
            if func.segments.len() > 2 {
                for segment in &func.segments[..func.segments.len() - 2] {
                    let stripped = strip_template_params(segment);
                    path.push(sanitize_filename(stripped));
                }
            }

            // n-1 segment becomes the .cpp filename (second to last)
            // Strip template parameters so all instantiations go into the same file
            let stripped = strip_template_params(&func.segments[func.segments.len() - 2]);
            let filename = format!("{}.cpp", sanitize_filename(stripped));
            path.push(filename);
            path
        };

        file_groups.entry(path).or_default().push(idx);
    }

    tracing::info!("Writing {} files", file_groups.len());

    // Write each file in parallel using rayon
    file_groups.par_iter().for_each(|(path, indices)| {
        // Create parent directories
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("Failed to create directory structure");
        }

        // Collect all function bodies for this file
        let mut file_content = Vec::new();

        for &idx in indices.iter() {
            let current_func = &functions[idx];

            // Find the next function's offset in the overall list (functions are sorted by offset)
            let end_offset = functions
                .get(idx + 1)
                .map(|f| f.offset)
                .unwrap_or(mmap.len());

            // Extract function body
            let body = &mmap[current_func.offset..end_offset];
            file_content.extend_from_slice(body);
        }

        // Write the file
        std::fs::write(path, file_content)
            .unwrap_or_else(|e| panic!("Failed to write file {:?}: {}", path, e));

        tracing::debug!("Wrote file: {}", path.display());
    });
}
