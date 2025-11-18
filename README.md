# IDA C Splitter

A tool that takes an IDA Hex-Rays decompiler full-executable C export and splits it into a navigable file tree.

## What It Does

When you export a full executable as C code from IDA's Hex-Rays decompiler, you get a single massive file. This tool:

1. Parses the IDA C export file
2. Identifies individual functions and their namespaces
3. Creates a directory structure based on namespace/class hierarchy
4. Splits functions into separate `.cpp` files for easy navigation

## Installation

```bash
cargo install --path .
```

## Usage

Basic usage:

```bash
ida-c-splitter --input path/to/ida_export.c --output path/to/output_dir
```

Short form:

```bash
ida-c-splitter -i ida_export.c -o output
```

If you don't specify an output directory, it defaults to `./output`.

### Examples

```bash
# Split an IDA export into the default 'output' directory
ida-c-splitter -i decompiled.c

# Specify a custom output directory
ida-c-splitter -i decompiled.c -o split_code

# Enable debug logging
RUST_LOG=debug ida-c-splitter -i decompiled.c
```

## Output Structure

The tool creates a file tree based on the function signatures:

- Functions with full namespaces like `Namespace::Class::Function` create directories for each namespace level
- Template instantiations are grouped together (e.g., `unique_ptr<Foo>` and `unique_ptr<Bar>` go in the same file)
- Functions with fewer than 2 namespace segments go into `global.cpp`
- Data declarations are written to `__data_declarations.cpp`

Example output structure:

```
output/
├── __data_declarations.cpp
├── global.cpp
├── MyNamespace/
│   ├── MyClass.cpp
│   └── Utilities/
│       └── Helper.cpp
└── std/
    ├── unique_ptr.cpp
    └── vector.cpp
```

## Logging

The tool uses `tracing` for logging. Control verbosity with the `RUST_LOG` environment variable:

```bash
# Info level (default)
RUST_LOG=info ida-c-splitter -i file.c

# Debug level (shows individual file writes)
RUST_LOG=debug ida-c-splitter -i file.c

# Trace level (very verbose)
RUST_LOG=trace ida-c-splitter -i file.c
```

## Performance

The tool uses memory mapping for fast file reading and parallel processing (via `rayon`) for writing output files, making it efficient even for large IDA exports.

## License

See LICENSE file for details.
