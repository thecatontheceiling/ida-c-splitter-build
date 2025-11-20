use crate::split_scope;

/// Parses a C++ function signature to extract namespace/class/function hierarchy.
///
/// # Examples
///
/// ```
/// use ida_c_splitter::signature_parser::parse_signature;
///
/// let result = parse_signature("void __fastcall CDiveState::CDiveState");
/// assert_eq!(result, vec!["CDiveState", "CDiveState"]);
/// ```
pub fn parse_signature(sig: &str) -> Vec<String> {
    let conventions = [
        "__fastcall",
        "__cdecl",
        "__thiscall",
        "__stdcall",
        "__vectorcall",
    ];

    let function_path = conventions
        .iter()
        .flat_map(|&conv| {
            let mut positions = Vec::new();
            let mut start = 0;
            while let Some(pos) = sig[start..].find(conv) {
                let abs = start + pos;
                let valid = abs == 0
                    || sig[..abs]
                        .chars()
                        .last()
                        .map(|c| c.is_whitespace() || matches!(c, '*' | '&' | '('))
                        .unwrap_or(false);
                if valid {
                    positions.push(abs);
                }
                start = abs + conv.len();
            }
            positions.into_iter().map(move |pos| (pos, conv))
        })
        .max_by_key(|(pos, _)| *pos)
        .map(|(pos, conv)| {
            let mut path = sig[pos + conv.len()..].trim_start();
            if path.starts_with("__noreturn") {
                path = path[10..].trim_start();
            }
            path.trim_start_matches('*')
                .trim_start_matches('&')
                .trim_start()
        })
        .unwrap_or_else(|| skip_return_type(sig.trim()));

    split_scope(function_path)
}

fn skip_return_type(sig: &str) -> &str {
    fn trim_ptr(s: &str) -> &str {
        s.trim_start_matches('*')
            .trim_start_matches('&')
            .trim_start()
    }

    if sig.contains('<') {
        let mut depth = 0;
        let mut last_space = None;
        for (i, ch) in sig.char_indices() {
            match ch {
                '<' => depth += 1,
                '>' => depth -= 1,
                ' ' if depth == 0 => last_space = Some(i),
                _ => {}
            }
        }
        if let Some(pos) = last_space {
            return trim_ptr(sig[pos..].trim_start());
        }
    } else if let Some(scope_pos) = find_first_scope_resolution(sig) {
        let last_space = sig[..scope_pos].rfind(' ');
        if let Some(pos) = last_space {
            return trim_ptr(sig[pos..].trim_start());
        }
    } else if let Some(pos) = sig.find(' ') {
        return trim_ptr(sig[pos..].trim_start());
    }

    sig
}

fn find_first_scope_resolution(s: &str) -> Option<usize> {
    let mut depth = 0;
    let mut chars = s.char_indices().peekable();
    while let Some((i, ch)) = chars.next() {
        match ch {
            '<' => depth += 1,
            '>' => depth -= 1,
            ':' if depth == 0 && chars.peek().map(|(_, c)| *c) == Some(':') => return Some(i),
            _ => {}
        }
    }
    None
}

#[cfg(test)]
#[path = "signature_parser_generated_tests.rs"]
mod signature_parser_generated_tests;
