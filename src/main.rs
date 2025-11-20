use clap::Parser;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::path::{Path, PathBuf};

mod signature_parser;
mod type_parser;

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

    // Build type hierarchy from header file (if available)
    let type_hierarchy = if let Some(header) = &args.header {
        tracing::info!("Opening header file: {}", header.display());
        let header_file = std::fs::File::open(header).expect("Failed to open file");
        let header_mmap = unsafe { memmap2::Mmap::map(&header_file).expect("Failed to mmap file") };

        // Parse header file
        let header_file = parse_header_file(&header_mmap);

        // Build type hierarchy
        let type_hierarchy = build_type_hierarchy(&header_file);

        // Create file tree hierarchy for types and empty_defs
        create_header_file_tree(&header_file, &header_mmap, output, &type_hierarchy);

        type_hierarchy
    } else {
        HashMap::new()
    };

    // Handle cpp file
    {
        tracing::info!("Opening cpp file: {}", args.input.display());
        let cpp_file = std::fs::File::open(&args.input).expect("Failed to open file");
        let cpp_mmap = unsafe { memmap2::Mmap::map(&cpp_file).expect("Failed to mmap file") };

        // Parse cpp file
        let cpp_file = parse_cpp_file(&cpp_mmap);

        // Create file tree hierarchy
        create_impl_file_tree(&cpp_file.functions, &cpp_mmap, output, &type_hierarchy);

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

#[derive(Default, Debug)]
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
            type_start..type_start + type_slice.len(),
            type_def.to_string(),
        ));
    }

    output
}

/// Builds a map of type names to their root type segments in the hierarchy.
/// Types that are nested within other types will map to their outermost parent.
/// For example, if we have:
///   - CFile
///   - CFile::SFileState
///   - CFile::SFileState::EState
///
/// Then all three will map to ["CFile"] as their root segments.
fn build_type_hierarchy(header_file: &HeaderFile) -> HashMap<String, Vec<String>> {
    // Phase 1: Collect all known type names and their segments
    let all_types: HashMap<String, Vec<String>> = header_file
        .types
        .iter()
        .map(|(_, type_def)| {
            let segments = type_parser::parse_type(type_def);
            let type_name = segments.join("::");
            (type_name, segments)
        })
        .collect();

    // Phase 2: For each type, find its root by walking up the hierarchy
    let mut type_roots: HashMap<String, Vec<String>> = HashMap::new();

    for (type_name, segments) in &all_types {
        // Start with the type itself as the root
        let mut root_segments = segments.clone();

        // Walk up the hierarchy from parent to grandparent, etc.
        for i in (1..segments.len()).rev() {
            let parent = segments[..i].join("::");
            if all_types.contains_key(&parent) {
                // Parent exists as a type, so this might be nested
                // Keep the parent as our current root candidate
                root_segments = segments[..i].to_vec();
            } else {
                // Parent is not a known type (it's a namespace or doesn't exist)
                // We've found our root
                break;
            }
        }

        type_roots.insert(type_name.clone(), root_segments);
    }

    type_roots
}

/// Creates the file tree hierarchy from type definitions
fn create_header_file_tree(
    header_file: &HeaderFile,
    mmap: &[u8],
    output_dir: &Path,
    type_hierarchy: &HashMap<String, Vec<String>>,
) {
    // First, collect all type segments to identify which empty_defs to skip
    let type_segments: HashSet<Vec<String>> = header_file
        .types
        .iter()
        .map(|(_, type_def)| type_parser::parse_type(type_def))
        .collect();

    // Keep original order: empty_defs first (filtered), then types
    let items = header_file
        .empty_defs
        .iter()
        .filter_map(|def| {
            let def = def.as_str();
            let segments = type_parser::parse_type(def);
            // Skip if this segment already exists in types
            if type_segments.contains(&segments) {
                None
            } else {
                Some((segments, def))
            }
        })
        .chain(header_file.types.iter().map(|(range, type_def)| {
            let segments = type_parser::parse_type(type_def);
            let body = unsafe { std::str::from_utf8_unchecked(&mmap[range.start..range.end]) };
            (segments, body)
        }));

    // Group items by their target file path
    let mut file_groups: HashMap<PathBuf, Vec<String>> = HashMap::new();

    for (segments, body) in items {
        let path = {
            // Determine which segments to use for file path
            let type_name = segments.join("::");
            let effective_segments = if let Some(root_segments) = type_hierarchy.get(&type_name) {
                // Use root type segments if part of a hierarchy
                root_segments
            } else {
                // Use original segments as fallback
                &segments
            };

            if effective_segments.is_empty() {
                panic!("Empty segments for type: {:?} {:?}", segments, body);
            }

            let mut path = output_dir.to_path_buf();

            // Create directory structure from parent namespaces (all but last segment)
            if effective_segments.len() > 1 {
                for segment in &effective_segments[..effective_segments.len() - 1] {
                    let stripped = strip_template_params(segment);
                    path.push(sanitize_filename(stripped));
                }
            }

            // Last segment becomes the filename
            let stripped = strip_template_params(&effective_segments[effective_segments.len() - 1]);
            let target_filename = sanitize_filename(stripped);
            let target_filename = target_filename
                .strip_suffix("_vtbl")
                .unwrap_or(&target_filename);
            let filename = format!("{target_filename}.h");
            path.push(filename);
            path
        };
        file_groups.entry(path).or_default().push(body.to_string());
    }

    tracing::info!("Writing {} header files", file_groups.len());

    // Write each file in parallel using rayon
    file_groups.par_iter().for_each(|(path, bodies)| {
        // Create parent directories
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("Failed to create directory structure");
        }

        // Combine all bodies for this file
        let file_content = bodies.join("\n");

        // Write the file
        std::fs::write(path, file_content)
            .unwrap_or_else(|e| panic!("Failed to write file {:?}: {}", path, e));

        tracing::debug!("Wrote header file: {}", path.display());
    });
}

#[derive(Default, Debug)]
struct CppFile {
    data_declarations: Range<usize>,
    functions: Vec<Function>,
}
#[derive(Debug, Clone)]
struct Function {
    offset: usize,
    segments: Vec<String>,
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

/// Creates the file tree hierarchy from function definitions
fn create_impl_file_tree(
    functions: &[Function],
    mmap: &[u8],
    output_dir: &Path,
    type_hierarchy: &HashMap<String, Vec<String>>,
) {
    // Group functions by their target file path
    let mut file_groups: HashMap<PathBuf, Vec<usize>> = HashMap::new();

    for (idx, func) in functions.iter().enumerate() {
        let path = {
            let segments = &func.segments;
            if segments.len() < 2 {
                // Functions with < 2 segments go into global.cpp
                output_dir.join("global.cpp")
            } else {
                // For functions, segments are [namespace/class, ..., class, method]
                // The containing scope is everything except the last element (the method name)
                let scope = segments[..segments.len() - 1].join("::");

                // Determine which segments to use for file path
                let effective_segments: &[String] =
                    if let Some(root_segments) = type_hierarchy.get(&scope) {
                        // Use root type segments if part of a hierarchy
                        root_segments
                    } else {
                        // Use segments[0..n-2] for namespace behavior (class name, not method)
                        &segments[..segments.len() - 1]
                    };

                let mut path = output_dir.to_path_buf();

                // Create directory structure from parent namespaces (all but last segment)
                if effective_segments.len() > 1 {
                    for segment in &effective_segments[..effective_segments.len() - 1] {
                        let stripped = strip_template_params(segment);
                        path.push(sanitize_filename(stripped));
                    }
                }

                // Last segment becomes the filename
                let stripped =
                    strip_template_params(&effective_segments[effective_segments.len() - 1]);
                let filename = format!("{}.cpp", sanitize_filename(stripped));
                path.push(filename);
                path
            }
        };
        file_groups.entry(path).or_default().push(idx);
    }

    tracing::info!("Writing {} cpp files", file_groups.len());

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

        tracing::debug!("Wrote cpp file: {}", path.display());
    });
}

/// Strips template parameters from a segment name
/// e.g., "unique_ptr<CResourceLoader>" -> "unique_ptr"
fn strip_template_params(segment: &str) -> &str {
    segment
        .find('<')
        .map(|pos| &segment[..pos])
        .unwrap_or(segment)
}
