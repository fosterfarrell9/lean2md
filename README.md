# lean2md

A command-line tool that converts Lean files (`.lean`) to Markdown (`.md`) documents, preserving code structure and comments.

## Description

`lean2md` processes Lean files and creates Markdown documentation that alternates between text from Lean comments and code blocks. It's designed to support literate programming with Lean by making it easy to generate readable documentation from annotated source files.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.56.0 or later
- Cargo (comes with Rust)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/lean2md.git
cd lean2md
# Build the project
cargo build --release
# The executable will be in target/release/lean2md
```

You might consider adding the `target/relase` folder to your PATH to run `lean2md` from any directory.

## Usage

Basic usage:

```bash
lean2md <lean_src_dir> <md_tgt_dir>
```

Example:

```bash
# Convert all .lean files in the PlaneGeometry directory to .md files in the docs directory
lean2md PlaneGeometry docs
```

When running with cargo:

```bash
cargo run -- <lean_src_dir> <md_tgt_dir>
```

## Features

- Converts Lean comments `/- ... -/` to Markdown text
- Maintains Lean code blocks inside Markdown code fences
- Preserves directory structure from source to target
- Special handling for docstrings and custom markers
- Recursive processing of nested folders

## Special Markers

- `--#` at the end of a line: Ignores the entire line
- `--#--`: Lines between two `--#--` markers are completely ignored
- `--+`  at the end of a docstring: The docstring is formatted as an admonish block

## Project Structure

- `src/lean2md.rs`: Main entry point and processing logic
- `Cargo.toml`: Project configuration and dependencies

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
