use ida_c_splitter::type_parser::parse_type;
use std::fs;
use std::io::Write;
use std::process::Command;

fn main() {
    let types = fs::read_to_string("types_sample.txt").expect("Failed to read types_sample.txt");

    let mut output = String::new();
    output.push_str("use crate::type_parser::parse_type;\n\n");

    let mut test_number = 1;
    for line in types.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Parse the type to get expected output
        let expected = parse_type(trimmed);

        // Skip types that produce empty results (these are unsupported or malformed)
        if expected.is_empty() {
            eprintln!(
                "Warning: Skipping type that produces empty result: {}",
                trimmed
            );
            continue;
        }

        // Generate the test case
        output.push_str("#[test]\n");
        output.push_str(&format!("fn test_type_{:03}() {{\n", test_number));
        output.push_str(&format!(
            "    let result = parse_type(\"{}\");\n",
            trimmed.replace("\\", "\\\\").replace("\"", "\\\"")
        ));
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
    let output_path = "src/type_parser_generated_tests.rs";
    let mut file = fs::File::create(output_path).expect("Failed to create output file");
    file.write_all(output.as_bytes())
        .expect("Failed to write to output file");

    // Format the generated file with cargo fmt
    let fmt_status = Command::new("cargo")
        .arg("fmt")
        .status()
        .expect("Failed to execute cargo fmt");

    if !fmt_status.success() {
        eprintln!("Warning: cargo fmt exited with non-zero status code");
    }

    println!(
        "Generated {} test cases in {}",
        test_number - 1,
        output_path
    );
}
