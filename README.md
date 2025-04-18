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
lean2md <lean_src_dir> <md_tgt_dir>     # Convert Lean files to Markdown
lean2md --version                       # Display version information
```

Example:

```bash
# Convert all .lean files in the Geometry directory to .md files in the docs directory
lean2md Geometry docs
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
- `--!` at the end of a line: Forces the line to be included in the output even if it would normally be filtered out

### Comment handling:
- Regular comments `/- ... -/`: Delimiters are removed in the output
- Docstrings `/-- ... -/`: Delimiters are preserved in the output and included in code blocks (unless the docstring is followed by `--+`)

Note: Marker precedence matters. Content within `--#--` ignore blocks will always be ignored, regardless of other markers like `--!`.

For concrete examples of how these features and markers work in practice, see the test fixtures in the `tests/fixtures/` directory. Each fixture contains input Lean files and their expected Markdown output, demonstrating how different markers and features behave.

## Project Structure

- `src/lean2md_core.rs`: Core functionality for converting Lean to Markdown
- `src/lib.rs`: Library interface that exports public functions
- `src/main.rs`: Command-line interface
- `tests/integration_tests.rs`: End-to-end tests
- `tests/fixtures/`: Test fixtures for various features

## Testing

The project includes both unit tests and integration tests:

- **Unit tests**: Located in `src/lean2md_core.rs`, they verify individual components like block parsing and marker handling.

- **Integration tests**: Located in `tests/integration_tests.rs`, they test end-to-end conversion using fixture files.

### Running Tests

```bash
# Run all tests
cargo test

# Run a specific test
cargo test test_markers
```

### Test Fixtures

Test fixtures are organized in `fixtures` with the following structure:

```
tests/fixtures/
  ├── admonish/
  │   ├── test_admonish.lean      # Input fixture
  │   └── expected_admonish.md    # Expected output
  ├── docstrings/
  ├── ignore_blocks/
  ├── markers/
  └── nested_code/
```

To add a new test case, create a new fixture directory with both input `.lean` and expected `.md` files, then add a test function in `integration_tests.rs` that calls `run_fixture_test()` with your fixture name.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgments

This project was inspired by and builds upon the work of:

- [Seasawher/mdgen](https://github.com/Seasawher/mdgen) - An excellent tool for generating Markdown from Lean files in Lean itself
- [arthurpaulino/lean2md](https://github.com/arthurpaulino/lean2md) - The original lean2md that provided the blueprint for this implementation

Thanks to the authors of these projects for their work in Lean documentation tooling.
