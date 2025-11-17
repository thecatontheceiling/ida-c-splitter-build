use ida_c_splitter::signature_parser::parse_signature;
use std::fs;
use std::io::Write;

fn main() {
    let signatures = fs::read_to_string("/home/user/ida-c-splitter/signatures_sample.txt")
        .expect("Failed to read signatures_sample.txt");

    let mut output = String::new();
    output.push_str("use crate::signature_parser::parse_signature;\n\n");

    let mut test_number = 1;
    for line in signatures.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Parse the signature to get expected output
        let expected = parse_signature(trimmed);

        // Generate the test case
        output.push_str(&format!("#[test]\n"));
        output.push_str(&format!("fn test_signature_{:03}() {{\n", test_number));
        output.push_str(&format!("    let result = parse_signature(\"{}\");\n",
            trimmed.replace("\\", "\\\\").replace("\"", "\\\"")));
        output.push_str("    assert_eq!(result, vec![\n");

        for (i, part) in expected.iter().enumerate() {
            let escaped = part.replace("\\", "\\\\").replace("\"", "\\\"");
            if i == expected.len() - 1 {
                output.push_str(&format!("        \"{}\"\n", escaped));
            } else {
                output.push_str(&format!("        \"{}\",\n", escaped));
            }
        }

        output.push_str("    ]);\n");
        output.push_str("}\n\n");

        test_number += 1;
    }

    // Write the output file
    let output_path = "/home/user/ida-c-splitter/src/signature_parser_generated_tests.rs";
    let mut file = fs::File::create(output_path)
        .expect("Failed to create output file");
    file.write_all(output.as_bytes())
        .expect("Failed to write to output file");

    println!("Generated {} test cases in {}", test_number - 1, output_path);
}
