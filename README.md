# IDA C Splitter

Splits IDA Hex-Rays decompiler C/C++ exports into navigable file trees organized by namespace hierarchy.

## What It Does

IDA's Hex-Rays decompiler exports executables as monolithic C/C++ files. This tool:

1. Parses C++ implementation files and optional header files from IDA
2. Extracts functions and type definitions with their namespace/class hierarchies
3. Creates directory structures matching C++ namespace organization
4. Splits content into separate `.cpp` and `.h` files for easy navigation
5. Groups nested types with their parent types for better organization

## Installation

```bash
cargo install --path .
```

## Usage

Basic usage:

```bash
ida-c-splitter implementation.cpp
```

With header file:

```bash
ida-c-splitter implementation.cpp --header types.h
```

Custom output directory:

```bash
ida-c-splitter implementation.cpp --header types.h --output split_code
```

Default output directory is `./output`.

### Examples

```bash
# Process implementation file only
ida-c-splitter game.cpp

# Process both header and implementation
ida-c-splitter game.cpp --header game.h

# Custom output with debug logging
RUST_LOG=debug ida-c-splitter game.cpp --header game.h -o output_dir
```

**Note:** For best results, include the header file with `--header`. The header contains type definitions that enable proper nested type grouping and hierarchy resolution. Without it, nested types (like `Class::NestedClass`) may be split into separate files instead of being grouped with their parent types.

## Output Structure

The tool creates organized file trees:

- **Namespace hierarchy**: `Namespace::Class::Function` creates `Namespace/Class.cpp`
- **Header files**: Type definitions from `--header` generate matching `.h` files
- **Template grouping**: `unique_ptr<Foo>` and `unique_ptr<Bar>` both go in `unique_ptr.cpp`
- **Nested type fusion**: `Class::NestedClass` groups with parent `Class` in one file
- **Global functions**: Functions without namespace go to `global.cpp`
- **Data declarations**: Static data goes to `_data.cpp`

Example structure:

```
output/
├── _data.cpp
├── global.cpp
├── Engine/
│   ├── Renderer.cpp
│   ├── Renderer.h
│   └── Audio/
│       ├── SoundSystem.cpp
│       └── SoundSystem.h
└── std/
    ├── unique_ptr.cpp
    ├── vector.cpp
    └── string.cpp
```

## Logging

The tool uses `tracing` for logging. Control verbosity with the `RUST_LOG` environment variable:

```bash
# Info level (default)
RUST_LOG=info ida-c-splitter file.c

# Debug level (shows individual file writes)
RUST_LOG=debug ida-c-splitter file.c

# Trace level (very verbose)
RUST_LOG=trace ida-c-splitter file.c
```

## Features

- **Fast**: Memory-mapped I/O and parallel processing via `rayon`
- **Smart grouping**: Nested types automatically fuse with parent types
- **Template handling**: Strips template parameters for clean filenames
- **Cross-platform**: Windows-safe filename sanitization
- **Comprehensive**: Handles structs, unions, enums, typedefs, and function pointers

## Performance

Uses memory-mapped file I/O and parallel processing for both parsing and writing, efficiently handling even multi-gigabyte IDA exports.
