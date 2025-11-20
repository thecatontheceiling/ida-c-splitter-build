use crate::split_scope;

/// Parses a C++ type definition to extract namespace/class/type hierarchy.
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
pub fn parse_type(def: &str) -> Vec<String> {
    let def = def.trim().strip_suffix(';').unwrap_or(def.trim());
    let def = strip_comments(def);

    if def.starts_with("typedef ") {
        parse_typedef(&def)
    } else if def.starts_with("enum ") || def.starts_with("const enum ") {
        parse_enum(&def)
    } else if def.starts_with("struct ") || def.starts_with("const struct ") {
        parse_struct(&def)
    } else if def.starts_with("union ") || def.starts_with("const union ") {
        parse_union(&def)
    } else {
        Vec::new()
    }
}

fn strip_comments(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '/' && chars.peek() == Some(&'*') {
            chars.next();
            while let Some(c) = chars.next() {
                if c == '*' && chars.peek() == Some(&'/') {
                    chars.next();
                    result.push(' ');
                    break;
                }
            }
        } else {
            result.push(ch);
        }
    }
    result
}

fn parse_typedef(def: &str) -> Vec<String> {
    let rest = def
        .strip_prefix("typedef ")
        .unwrap_or(def)
        .trim()
        .trim_end_matches(';')
        .trim();
    let mut name = find_last_type_in_typedef(rest)
        .trim_start_matches('*')
        .trim_start_matches('&')
        .trim();

    if name.starts_with('(') && name.contains('*')
        && let Some(star) = name.find('*')
            && let Some(end) = name[star + 1..].find(')') {
                name = name[star + 1..star + 1 + end].trim();
            }

    let mut depth = 0;
    for (i, ch) in name.char_indices() {
        match ch {
            '<' => depth += 1,
            '>' => depth -= 1,
            '(' if depth == 0 => {
                if i > 0 {
                    name = name[..i].trim();
                }
                break;
            }
            _ => {}
        }
    }

    split_scope(name)
}

fn parse_enum(def: &str) -> Vec<String> {
    let rest = def.strip_prefix("const ").unwrap_or(def);
    let rest = rest.strip_prefix("enum ").unwrap_or(rest).trim();
    let rest = skip_enum_qualifiers(rest);
    let name = find_type_separator(rest, ':')
        .map(|pos| rest[..pos].trim())
        .unwrap_or(rest);
    split_scope(name)
}

fn parse_struct(def: &str) -> Vec<String> {
    let rest = def.strip_prefix("const ").unwrap_or(def);
    let rest = rest.strip_prefix("struct ").unwrap_or(rest).trim();
    let rest = skip_struct_qualifiers(rest);
    let name = find_type_separator(rest, ':')
        .map(|pos| rest[..pos].trim())
        .unwrap_or(rest);
    split_scope(name)
}

fn parse_union(def: &str) -> Vec<String> {
    let rest = def.strip_prefix("const ").unwrap_or(def);
    let rest = rest.strip_prefix("union ").unwrap_or(rest).trim();
    let rest = skip_struct_qualifiers(rest);
    split_scope(rest)
}

fn skip_struct_qualifiers(s: &str) -> &str {
    let mut rest = s;
    loop {
        let before = rest;
        rest = rest.trim_start();

        if rest.starts_with("__cppobj ") {
            rest = &rest[9..];
        } else if rest.starts_with("__unaligned ") {
            rest = &rest[12..];
        } else if rest.starts_with("__declspec(")
            && let Some(close) = find_matching_paren(rest, 10) {
                rest = rest[close + 1..].trim_start();
            }

        if rest == before || rest == before.trim_start() {
            break;
        }
    }
    rest
}

fn skip_enum_qualifiers(s: &str) -> &str {
    let mut rest = s;
    loop {
        let before = rest.trim_start();
        if let Some(stripped) = before.strip_prefix("__bitmask ") {
            rest = stripped;
        } else {
            return before;
        }
    }
}

fn find_matching_paren(s: &str, open: usize) -> Option<usize> {
    let mut depth = 1;
    for (i, ch) in s[open + 1..].char_indices() {
        match ch {
            '(' => depth += 1,
            ')' if {
                depth -= 1;
                depth == 0
            } =>
            {
                return Some(open + 1 + i);
            }
            _ => {}
        }
    }
    None
}

fn find_last_type_in_typedef(s: &str) -> &str {
    let mut depths = (0i32, 0i32, 0i32);
    let mut last_space = None;
    for (i, ch) in s.char_indices() {
        match ch {
            '<' => depths.0 += 1,
            '>' => depths.0 = depths.0.saturating_sub(1),
            '(' => depths.1 += 1,
            ')' => depths.1 = depths.1.saturating_sub(1),
            '[' => depths.2 += 1,
            ']' => depths.2 = depths.2.saturating_sub(1),
            ' ' if depths == (0, 0, 0) => last_space = Some(i),
            _ => {}
        }
    }
    last_space.map(|p| s[p + 1..].trim()).unwrap_or(s)
}

fn find_type_separator(s: &str, sep: char) -> Option<usize> {
    let mut depth = 0;
    let chars: Vec<char> = s.chars().collect();
    for (i, &ch) in chars.iter().enumerate() {
        match ch {
            '<' => depth += 1,
            '>' => depth -= 1,
            c if c == sep && depth == 0 => {
                if sep == ':' {
                    let prev = i > 0 && chars[i - 1] == ':';
                    let next = i + 1 < chars.len() && chars[i + 1] == ':';
                    if !prev && !next {
                        return s.char_indices().nth(i).map(|(p, _)| p);
                    }
                } else {
                    return s.char_indices().nth(i).map(|(p, _)| p);
                }
            }
            _ => {}
        }
    }
    None
}

#[cfg(test)]
#[path = "type_parser_generated_tests.rs"]
mod type_parser_generated_tests;
