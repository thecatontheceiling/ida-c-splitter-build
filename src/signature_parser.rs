/// Parses a C++ function signature to extract the namespace/class/function hierarchy.
///
/// This parser handles pseudo-C++ function signatures (without parameters) and extracts
/// the qualified function name, splitting it by `::` while preserving template parameters.
///
/// # Examples
///
/// ```
/// use ida_c_splitter::signature_parser::parse_signature;
///
/// let result = parse_signature("void __fastcall CDiveState::CDiveState");
/// assert_eq!(result, vec!["CDiveState", "CDiveState"]);
/// ```
pub fn parse_signature(signature: &str) -> Vec<String> {
    // Find the calling convention keyword to determine where the function path starts
    let calling_conventions = [
        "__fastcall",
        "__cdecl",
        "__thiscall",
        "__stdcall",
        "__vectorcall",
    ];

    // For function pointer return types, there may be multiple calling conventions
    // (one in the return type, one for the function itself). We want the LAST one.
    let function_path = calling_conventions
        .iter()
        .flat_map(|&conv| {
            // Find all occurrences of this calling convention
            let mut positions = Vec::new();
            let mut start = 0;
            while let Some(pos) = signature[start..].find(conv) {
                positions.push(start + pos);
                start = start + pos + conv.len();
            }
            positions.into_iter().map(move |pos| (pos, conv))
        })
        .max_by_key(|(pos, _)| *pos)
        .map(|(pos, conv)| {
            let mut path = signature[pos + conv.len()..].trim_start();
            // Skip function attributes that may appear after calling convention
            let attributes = ["__noreturn"];
            for &attr in &attributes {
                if path.starts_with(attr) {
                    path = path[attr.len()..].trim_start();
                }
            }
            path
        })
        .unwrap_or_else(|| skip_return_type(signature.trim()));

    // Split by :: while respecting template bracket depth
    split_by_scope_resolution(function_path)
}

/// Skips the return type in a signature that doesn't have a calling convention.
///
/// For signatures like "void SomeFunction" or "void *Namespace::Function",
/// this returns everything after the return type.
fn skip_return_type(signature: &str) -> &str {
    // Find the first occurrence of ::
    if let Some(scope_pos) = signature.find("::") {
        // Backtrack to find the last space before ::
        let before_scope = &signature[..scope_pos];
        if let Some(space_pos) = before_scope.rfind(' ') {
            // Skip any pointer/reference markers after the space
            let after_space = &signature[space_pos..].trim_start();
            return after_space
                .trim_start_matches('*')
                .trim_start_matches('&')
                .trim_start();
        }
    }

    // No :: found, so this is a simple function name
    // Find the first space and return everything after it
    if let Some(space_pos) = signature.find(' ') {
        let after_space = &signature[space_pos..].trim_start();
        return after_space
            .trim_start_matches('*')
            .trim_start_matches('&')
            .trim_start();
    }

    // No space found, return the whole thing (shouldn't happen in practice)
    signature
}

/// Splits a C++ qualified name by `::` while preserving content within template brackets.
///
/// This function correctly handles nested templates and ensures that `::` within
/// template parameters are not used as split points.
fn split_by_scope_resolution(path: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut bracket_depth = 0;
    let mut chars = path.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '<' => {
                bracket_depth += 1;
                current.push(ch);
            }
            '>' => {
                bracket_depth -= 1;
                current.push(ch);
            }
            ':' if bracket_depth == 0 => {
                // Check if this is part of ::
                if chars.peek() == Some(&':') {
                    chars.next(); // consume the second :
                    if !current.is_empty() {
                        parts.push(current.trim().to_string());
                        current = String::new();
                    }
                } else {
                    current.push(ch);
                }
            }
            _ => {
                current.push(ch);
            }
        }
    }

    if !current.is_empty() {
        parts.push(current.trim().to_string());
    }

    parts
}

#[cfg(test)]
#[path = "signature_parser_generated_tests.rs"]
mod signature_parser_generated_tests;
