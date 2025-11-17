use serde::Serialize;

#[derive(Debug, Serialize)]
struct Intermediate {
    function_declarations: usize,
    data_declarations: usize,
    function_definitions: Vec<IntermediateFunction>,
}

#[derive(Debug, Serialize)]
struct IntermediateFunction {
    pub offset: usize,
    pub address: usize,
    pub signature: String,
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

        function_definitions.push(IntermediateFunction {
            offset,
            address,
            signature: signature.to_string(),
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
}
