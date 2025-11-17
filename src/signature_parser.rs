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
        .unwrap_or(signature.trim());

    // Split by :: while respecting template bracket depth
    split_by_scope_resolution(function_path)
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
mod tests {
    use super::*;

    #[test]
    fn test_simple_function() {
        let result = parse_signature("void __fastcall CDiveState::CDiveState");
        assert_eq!(result, vec!["CDiveState", "CDiveState"]);
    }

    #[test]
    fn test_complex_templated_function() {
        let signature = "__int64 __fastcall hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfFilterNode<hknpCompressedMeshShapeInternals::RayCastQuery<1>,hkcdStaticTree::Tree<hkcdStaticTree::DynamicStorage5>::NodeContext>::call<hkcdStaticTree::Tree<hkcdStaticTree::DynamicStorage5>::NodeContext>";
        let result = parse_signature(signature);
        assert_eq!(
            result,
            vec![
                "hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>",
                "IfFilterNode<hknpCompressedMeshShapeInternals::RayCastQuery<1>,hkcdStaticTree::Tree<hkcdStaticTree::DynamicStorage5>::NodeContext>",
                "call<hkcdStaticTree::Tree<hkcdStaticTree::DynamicStorage5>::NodeContext>"
            ]
        );
    }

    #[test]
    fn test_very_long_templated_function() {
        let signature = "std::tr1::_Callable_obj<std::tr1::_Bind<void,void,std::tr1::_Bind1<std::tr1::_Callable_pmf<void (__cdecl NSaveLoad::CManagedObject<NSaveLoad::CLocalDataRef<CPlayerActionObserver::CSystemicButtonHintData> >::*const)(void),NSaveLoad::CManagedObject<NSaveLoad::CLocalDataRef<CPlayerActionObserver::CSystemicButtonHintData> >,0>,NSaveLoad::CManagedObject<NSaveLoad::CLocalDataRef<CPlayerActionObserver::CSystemicButtonHintData> > *> >,0> *__fastcall std::tr1::_Impl_no_alloc0<std::tr1::_Callable_obj<std::tr1::_Bind<void,void,std::tr1::_Bind1<std::tr1::_Callable_pmf<void (NSaveLoad::CManagedObject<NSaveLoad::CLocalDataRef<CPlayerActionObserver::CSystemicButtonHintData>>::*const)(void),NSaveLoad::CManagedObject<NSaveLoad::CLocalDataRef<CPlayerActionObserver::CSystemicButtonHintData>>,0>,NSaveLoad::CManagedObject<NSaveLoad::CLocalDataRef<CPlayerActionObserver::CSystemicButtonHintData>> *>>,0>,void>::_Get";
        let result = parse_signature(signature);
        assert_eq!(
            result,
            vec![
                "std",
                "tr1",
                "_Impl_no_alloc0<std::tr1::_Callable_obj<std::tr1::_Bind<void,void,std::tr1::_Bind1<std::tr1::_Callable_pmf<void (NSaveLoad::CManagedObject<NSaveLoad::CLocalDataRef<CPlayerActionObserver::CSystemicButtonHintData>>::*const)(void),NSaveLoad::CManagedObject<NSaveLoad::CLocalDataRef<CPlayerActionObserver::CSystemicButtonHintData>>,0>,NSaveLoad::CManagedObject<NSaveLoad::CLocalDataRef<CPlayerActionObserver::CSystemicButtonHintData>> *>>,0>,void>",
                "_Get"
            ]
        );
    }

    #[test]
    fn test_namespace_function() {
        let result = parse_signature("void __fastcall std::bad_alloc::bad_alloc");
        assert_eq!(result, vec!["std", "bad_alloc", "bad_alloc"]);
    }

    #[test]
    fn test_templated_namespace() {
        let result = parse_signature("int *__fastcall std::_Uninitialized_copy<int *,int *,std::allocator<int>>");
        assert_eq!(
            result,
            vec!["std", "_Uninitialized_copy<int *,int *,std::allocator<int>>"]
        );
    }
}

#[cfg(test)]
#[path = "signature_parser_generated_tests.rs"]
mod signature_parser_generated_tests;
