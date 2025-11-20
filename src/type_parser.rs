/// Parses a C++ type definition to extract the namespace/class/type hierarchy.
///
/// This parser handles pseudo-C++ type definitions (struct, enum, typedef, union, etc.)
/// and extracts the qualified type name, splitting it by `::` while preserving template parameters.
///
/// # Examples
///
/// ```
/// use ida_c_splitter::type_parser::parse_type;
///
/// let result = parse_type("struct __cppobj TAdfStructPtr<SDeformPoints> : CAdfStructPtrBase");
/// assert_eq!(result, vec!["TAdfStructPtr<SDeformPoints>"]);
///
/// let result = parse_type("struct __cppobj dynamo::vm::machine");
/// assert_eq!(result, vec!["dynamo", "vm", "machine"]);
/// ```
pub fn parse_type(type_def: &str) -> Vec<String> {
    let type_def = type_def.trim();

    // Strip comments like /*VFT*/
    let type_def = strip_comments(type_def);

    // Determine what kind of type definition this is
    if type_def.starts_with("typedef ") {
        parse_typedef(&type_def)
    } else if type_def.starts_with("enum ") || type_def.starts_with("const enum ") {
        parse_enum(&type_def)
    } else if type_def.starts_with("struct ") || type_def.starts_with("const struct ") {
        parse_struct(&type_def)
    } else if type_def.starts_with("union ") {
        parse_union(&type_def)
    } else {
        // Unknown type, try to extract what we can
        Vec::new()
    }
}

/// Strips C-style comments from the input string
fn strip_comments(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '/' {
            if chars.peek() == Some(&'*') {
                chars.next(); // consume *
                // Skip until we find */
                let mut found_end = false;
                while let Some(c) = chars.next() {
                    if c == '*' && chars.peek() == Some(&'/') {
                        chars.next(); // consume /
                        found_end = true;
                        break;
                    }
                }
                if found_end {
                    result.push(' '); // Replace comment with space
                }
            } else {
                result.push(ch);
            }
        } else {
            result.push(ch);
        }
    }

    result
}

/// Parse a typedef declaration
/// Format: typedef [old_type] [new_type];
/// We want to extract the new_type (the rightmost identifier/path)
fn parse_typedef(type_def: &str) -> Vec<String> {
    // Remove "typedef " prefix
    let rest = type_def.strip_prefix("typedef ").unwrap_or(type_def).trim();

    // Remove trailing semicolon if present
    let rest = rest.trim_end_matches(';').trim();

    // Find the last space outside of template brackets and parentheses
    // This separates the old type from the new type
    let mut type_name = find_last_type_in_typedef(rest);

    // Handle function pointers: typedef RET (*NAME)(ARGS)
    // or typedef RET (__convention *NAME)(ARGS)
    // The name is after the * inside the first parentheses
    if type_name.starts_with('(') && type_name.contains('*') {
        // Find the * inside the first set of parentheses
        if let Some(star_pos) = type_name.find('*') {
            let after_star = &type_name[star_pos + 1..];
            // Find the closing ) after the name
            if let Some(end) = after_star.find(')') {
                type_name = after_star[..end].trim();
            }
        }
    } else {
        // For simple pointers, strip leading * and & characters
        type_name = type_name
            .trim_start_matches('*')
            .trim_start_matches('&')
            .trim();
    }

    // Now split by :: while preserving templates
    split_by_scope_resolution(type_name)
}

/// Parse an enum declaration
/// Format: [const] enum [__bitmask] [namespace::]Name [: underlying_type]
fn parse_enum(type_def: &str) -> Vec<String> {
    // Remove "const " prefix if present
    let rest = type_def.strip_prefix("const ").unwrap_or(type_def);

    // Remove "enum " prefix
    let rest = rest.strip_prefix("enum ").unwrap_or(rest).trim();

    // Skip enum qualifiers like __bitmask
    let rest = skip_enum_qualifiers(rest);

    // Split by : to separate the enum name from the underlying type
    // But only if : is not inside template brackets
    let enum_name = if let Some(colon_pos) = find_type_separator(rest, ':') {
        rest[..colon_pos].trim()
    } else {
        rest
    };

    split_by_scope_resolution(enum_name)
}

/// Parse a struct declaration
/// Format: [const] struct [__cppobj] [__declspec(...)] [namespace::]Name [: base_classes]
fn parse_struct(type_def: &str) -> Vec<String> {
    // Remove "const " prefix if present
    let rest = type_def.strip_prefix("const ").unwrap_or(type_def);

    // Remove "struct " prefix
    let rest = rest.strip_prefix("struct ").unwrap_or(rest).trim();

    // Skip qualifiers like __cppobj, __declspec(...), __unaligned
    let rest = skip_struct_qualifiers(rest);

    // Find the struct name (before : or end of string)
    let struct_name = if let Some(colon_pos) = find_type_separator(rest, ':') {
        rest[..colon_pos].trim()
    } else {
        rest
    };

    split_by_scope_resolution(struct_name)
}

/// Parse a union declaration
/// Format: union [__declspec(...)] [namespace::]Name
fn parse_union(type_def: &str) -> Vec<String> {
    // Remove "union " prefix
    let rest = type_def.strip_prefix("union ").unwrap_or(type_def).trim();

    // Skip qualifiers like __declspec(...)
    let rest = skip_struct_qualifiers(rest);

    split_by_scope_resolution(rest)
}

/// Skip struct qualifiers like __cppobj, __declspec(align(8)), __unaligned
fn skip_struct_qualifiers(s: &str) -> &str {
    let mut rest = s;

    loop {
        let original = rest;
        rest = rest.trim_start();

        // Skip __cppobj
        if rest.starts_with("__cppobj ") {
            rest = &rest[9..];
            continue;
        }

        // Skip __unaligned
        if rest.starts_with("__unaligned ") {
            rest = &rest[12..];
            continue;
        }

        // Skip __declspec(...)
        if rest.starts_with("__declspec(")
            && let Some(close_pos) = find_matching_paren(rest, 10)
        {
            rest = rest[close_pos + 1..].trim_start();
            continue;
        }

        // If nothing changed, we're done
        if rest == original || rest == original.trim_start() {
            break;
        }
    }

    rest
}

/// Skip enum qualifiers like __bitmask
fn skip_enum_qualifiers(s: &str) -> &str {
    let mut rest = s;

    loop {
        let original = rest;
        rest = rest.trim_start();

        // Skip __bitmask
        if rest.starts_with("__bitmask ") {
            rest = &rest[10..];
            continue;
        }

        // If nothing changed, we're done
        if rest == original || rest == original.trim_start() {
            break;
        }
    }

    rest
}

/// Find the position of a matching closing parenthesis
fn find_matching_paren(s: &str, open_pos: usize) -> Option<usize> {
    let mut depth = 1;
    for (i, ch) in s[open_pos + 1..].char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(open_pos + 1 + i);
                }
            }
            _ => {}
        }
    }
    None
}

/// Find the last type name in a typedef (after the last space outside brackets/templates)
fn find_last_type_in_typedef(s: &str) -> &str {
    let mut bracket_depth: i32 = 0;
    let mut angle_depth: i32 = 0;
    let mut paren_depth: i32 = 0;
    let mut last_space_outside = None;

    for (i, ch) in s.char_indices() {
        match ch {
            '<' => angle_depth += 1,
            '>' => angle_depth = angle_depth.saturating_sub(1),
            '(' => paren_depth += 1,
            ')' => paren_depth = paren_depth.saturating_sub(1),
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            ' ' if angle_depth == 0 && paren_depth == 0 && bracket_depth == 0 => {
                last_space_outside = Some(i);
            }
            _ => {}
        }
    }

    if let Some(pos) = last_space_outside {
        s[pos + 1..].trim()
    } else {
        s
    }
}

/// Find the position of a separator character outside of template brackets
/// For ':' separator, only matches single ':' not followed by another ':'
fn find_type_separator(s: &str, separator: char) -> Option<usize> {
    let mut angle_depth: i32 = 0;
    let chars: Vec<char> = s.chars().collect();

    for (i, &ch) in chars.iter().enumerate() {
        match ch {
            '<' => angle_depth += 1,
            '>' => angle_depth = angle_depth.saturating_sub(1),
            c if c == separator && angle_depth == 0 => {
                // If separator is ':', make sure it's not part of '::'
                if separator == ':' {
                    // Check if the previous char is ':'
                    let prev_is_colon = i > 0 && chars[i - 1] == ':';
                    // Check if the next char is ':'
                    let next_is_colon = i + 1 < chars.len() && chars[i + 1] == ':';

                    // Only match if it's a standalone ':'
                    if !prev_is_colon && !next_is_colon {
                        return s.char_indices().nth(i).map(|(pos, _)| pos);
                    }
                } else {
                    return s.char_indices().nth(i).map(|(pos, _)| pos);
                }
            }
            _ => {}
        }
    }

    None
}

/// Splits a C++ qualified name by `::` while preserving content within template brackets.
///
/// This function correctly handles nested templates and ensures that `::` within
/// template parameters are not used as split points.
fn split_by_scope_resolution(path: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut bracket_depth: i32 = 0;
    let mut chars = path.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '<' => {
                bracket_depth += 1;
                current.push(ch);
            }
            '>' => {
                bracket_depth = bracket_depth.saturating_sub(1);
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
#[path = "type_parser_generated_tests.rs"]
mod type_parser_generated_tests;
