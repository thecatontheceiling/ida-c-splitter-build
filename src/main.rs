use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;

mod signature_parser;

#[derive(Debug, Serialize)]
struct Intermediate {
    function_declarations: usize,
    data_declarations: usize,
    function_definitions: Vec<IntermediateFunction>,
}

#[derive(Debug, Serialize, Clone)]
struct IntermediateFunction {
    pub offset: usize,
    pub address: usize,
    pub signature: String,
    pub segments: Vec<String>,
}

fn main() {
    let file_path = std::env::args().nth(1).expect("No file path provided");

    let file = std::fs::File::open(file_path).expect("Failed to open file");
    let mmap = unsafe { memmap2::Mmap::map(&file).expect("Failed to mmap file") };

    let function_declarations = memchr::memmem::find(&mmap, b"// Function declarations")
        .expect("Failed to find function declarations");
    println!("Function declarations: {function_declarations}");
    let data_declarations =
        memchr::memmem::find(&mmap[function_declarations..], b"// Data declarations")
            .expect("Failed to find data declarations")
            + function_declarations;
    println!("Data declarations: {data_declarations}");
    let mut function_definitions: Vec<IntermediateFunction> = vec![];

    for offset_from_dd in memchr::memmem::find_iter(&mmap[data_declarations..], b"//----- (") {
        let offset = offset_from_dd + data_declarations;
        let address_start = offset + "//----- (".len();
        let address_length = "0000000140AFDCB0".len();
        let address_slice = &mmap[address_start..address_start + address_length];
        let address_str = unsafe { std::str::from_utf8_unchecked(address_slice) };
        let address = usize::from_str_radix(address_str, 16).expect("Failed to parse address");

        let remaining_file = unsafe { std::str::from_utf8_unchecked(&mmap[offset..]) };
        let signature = remaining_file
            .lines()
            .find(|line| !line.starts_with("//"))
            .expect("Failed to find signature");
        let signature = &signature[..signature
            .rfind("(")
            .expect("Failed to find opening parenthesis")];

        let segments = signature_parser::parse_signature(signature);
        function_definitions.push(IntermediateFunction {
            offset,
            address,
            signature: signature.to_string(),
            segments,
        });
    }
    println!("Function definitions: {}", function_definitions.len());

    let intermediate = Intermediate {
        function_declarations,
        data_declarations,
        function_definitions,
    };

    let json =
        serde_json::to_string_pretty(&intermediate).expect("Failed to serialize intermediate");
    std::fs::write("intermediate.json", json).expect("Failed to write intermediate.json");

    // Create file tree hierarchy
    create_file_tree(&intermediate.function_definitions, &mmap, "output");
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

/// Strips template parameters from a segment name
/// e.g., "unique_ptr<CResourceLoader>" -> "unique_ptr"
fn strip_template_params(segment: &str) -> &str {
    segment
        .find('<')
        .map(|pos| &segment[..pos])
        .unwrap_or(segment)
}

/// Creates the file tree hierarchy from function definitions
fn create_file_tree(functions: &[IntermediateFunction], mmap: &[u8], output_dir: &str) {
    // Group functions by their target file path
    let mut file_groups: HashMap<PathBuf, Vec<usize>> = HashMap::new();

    for (idx, func) in functions.iter().enumerate() {
        let path = if func.segments.len() < 2 {
            // Functions with < 2 segments go into global.cpp
            PathBuf::from(output_dir).join("global.cpp")
        } else {
            let mut path = PathBuf::from(output_dir);

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

    // Create output directory
    std::fs::create_dir_all(output_dir).expect("Failed to create output directory");

    // Write each file
    for (path, indices) in file_groups {
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
        std::fs::write(&path, file_content)
            .unwrap_or_else(|e| panic!("Failed to write file {:?}: {}", path, e));
    }

    println!("File tree created in {} directory", output_dir);
}
