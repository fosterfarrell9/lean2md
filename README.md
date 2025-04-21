# lean2md

A command-line tool that converts Lean files (`.lean`) to Markdown (`.md`) documents, preserving code structure and comments.

## Description

`lean2md` processes Lean files and creates Markdown documentation that alternates between text from Lean comments and code blocks. It's designed to support literate programming with Lean by making it easy to generate readable documentation from annotated source files, particularly for use with mdbook to create polished documentation websites.

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
- `--@quiz:name` and `--@quiz-end`: Creates a quiz within a comment block that will be extracted to a TOML file in the `quizzes` directory and referenced in the Markdown with `{{#quiz ../quizzes/name.toml}}`

### Comment handling
- Regular comments `/- ... -/`: Delimiters are removed in the output
- Docstrings `/-- ... -/`: Delimiters are preserved in the output and included in code blocks (unless the docstring is followed by `--+`)

### Quiz Support

lean2md supports creating quizzes for the [mdbook-quiz](https://github.com/cognitive-engineering-lab/mdbook-quiz) preprocessor directly within Lean files:

- Quizzes are defined inside comment blocks with `--@quiz:name` and `--@quiz-end` markers.
- The content between markers is extracted verbatim (markers inside are preserved).
- A TOML file is generated at `<parent_of_md_tgt_dir>/quizzes/name.toml`.
- A reference `{{#quiz ../quizzes/name.toml}}` is added to the markdown output.
- Inside the quiz block you should use the syntax required by `mdbook-quiz`. There are three types of questions provided by `mdbook-quiz`:
  - **ShortAnswer**: For questions where the user inputs a text answer
  - **MultipleChoice**: For questions with several options and one correct answer
  - **Tracing**: Evaluates if code will compile using the Rust compiler. Note that this question type is not (yet) suitable for Lean code, as the quiz system will attempt to compile it with the Rust compiler.
- In order for the quizzes to work, you need to have `mdbook-quiz` installed and added `[preprocessor.quiz]` to your `book.toml` file

Example:
```lean
/-
--@quiz:lean_basics
[[questions]]
type = "ShortAnswer"
prompt.prompt = "What is the keyword for definitions in Lean?"
answer.answer = "def"
context = "For example, you can write: `def x := 5`."

[[questions]]
type = "MultipleChoice"
prompt.prompt = "What symbol is used for type annotations in Lean?"
prompt.distractors = [
  "=>",
  "->",
  "=="
]
answer.answer = ":"
context = """
In Lean, we use the colon symbol to annotate types. For example: `def x : Nat := 5`
"""
--@quiz-end
-/
```

### Notes and further examples

Marker precedence matters. Content within `--#--` ignore blocks will always be ignored, regardless of other markers like `--!`. Markers inside quiz blocks will also be ignored.

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
  ├── nested_code/
  └── quizzes/
```

To add a new test case, create a new fixture directory with both input `.lean` and expected `.md` files, then add a test function in `integration_tests.rs` that calls `run_fixture_test()` with your fixture name.

## Integration with mdbook

The markdown files generated by lean2md can be easily used with [mdbook](https://rust-lang.github.io/mdBook/) to create documentation websites or e-books. Combined with the quiz support (via [mdbook-quiz](https://github.com/cognitive-engineering-lab/mdbook-quiz)), this provides a complete solution for creating interactive learning materials directly from your Lean code.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgments

This project was inspired by and builds upon the work of:

- [Seasawher/mdgen](https://github.com/Seasawher/mdgen) - An excellent tool for generating Markdown from Lean files in Lean itself
- [arthurpaulino/lean2md](https://github.com/arthurpaulino/lean2md) - The original lean2md that provided the blueprint for this implementation

Thanks to the authors of these projects for their work in Lean documentation tooling.
