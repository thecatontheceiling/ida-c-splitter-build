# IDA C Splitter

Splits IDA Hex-Rays decompiler C/C++ exports into navigable file trees organized by namespace hierarchy.

Developed by yours truly and Claude. I promise I have actually tested this, and that it does actually work.

## What It Does

IDA's Hex-Rays decompiler exports executables as monolithic C/C++ files. This tool:

1. Parses C++ implementation files and optional header files from IDA
2. Extracts functions and type definitions with their namespace/class hierarchies
3. Creates directory structures matching C++ namespace organization
4. Splits content into separate `.cpp` and `.h` files for easy navigation
5. Groups nested types with their parent types for better organization

## Installation

```bash
cargo install --git https://github.com/ferrobrew/ida-c-splitter.git
```

## Usage

Pass an IDA export to the tool; this will output a file tree in `./output`.

```bash
ida-c-splitter implementation.c
```

For best results, combine this with the IDA header export. The header contains type definitions that enable proper nested type grouping and hierarchy resolution. Without it, nested types (like `Class::NestedClass`) may be split into separate files instead of being grouped with their parent types.

```bash
ida-c-splitter implementation.c --header types.h
```

Set a custom output directory:

```bash
ida-c-splitter implementation.c --header types.h --output project
```

### Examples

```bash
# Process implementation file only
ida-c-splitter game.cpp

# Process both header and implementation
ida-c-splitter game.cpp --header game.h

# Custom output with debug logging
RUST_LOG=debug ida-c-splitter game.cpp --header game.h -o output_dir
```

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
