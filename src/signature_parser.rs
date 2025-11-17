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

    let function_path = calling_conventions
        .iter()
        .filter_map(|&conv| {
            signature
                .find(conv)
                .map(|pos| signature[pos + conv.len()..].trim_start())
        })
        .next()
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
            return after_space.trim_start_matches('*').trim_start_matches('&').trim_start();
        }
    }

    // No :: found, so this is a simple function name
    // Find the first space and return everything after it
    if let Some(space_pos) = signature.find(' ') {
        let after_space = &signature[space_pos..].trim_start();
        return after_space.trim_start_matches('*').trim_start_matches('&').trim_start();
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
