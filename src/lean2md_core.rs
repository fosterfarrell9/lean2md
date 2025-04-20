use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

#[derive(Debug)]
/// Represents a block of content extracted from a Lean file
pub struct Block {
    /// The textual content of the block
    pub content: String,
    /// Whether this block represents code (true) or text (false)
    pub is_code: bool,
    /// Whether this block should be formatted as an admonish block
    pub is_admonish: bool,
    /// Optional reference to a quiz file
    pub quiz_reference: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_blocks_basic() {
        let input = "/- Simple comment -/\ndef foo := 1";
        let (blocks, _) = build_blocks(input).unwrap();

        // Print blocks for debugging
        println!("Number of blocks: {}", blocks.len());
        for (i, block) in blocks.iter().enumerate() {
            println!("Block {}: is_code={}, content='{}'", i, block.is_code, block.content);
        }

        assert_eq!(blocks.len(), 2);
    }

    #[test]
    fn test_ignore_marker() {
        let input = "line 1\nline 2 --#\nline 3";
        let (blocks, _) = build_blocks(input).unwrap();
        assert!(!blocks[0].content.contains("line 2"));
    }

    #[test]
    fn test_force_include_marker() {
        let input = "line 1\nline 2 --#--!\nline 3";
        let (blocks, _) = build_blocks(input).unwrap();
        assert!(blocks[0].content.contains("line 2 --#"));
    }
}

/// Parses a Lean file content into blocks and quizzes
///
/// # Arguments
///
/// * `content` - String content of a Lean file
///
/// # Returns
///
/// Result containing a tuple of (blocks, quizzes) or an error message
pub fn build_blocks(content: &str) -> Result<(Vec<Block>, Vec<(String, String)>), String> {
    let mut blocks = Vec::new();
    let mut quizzes = Vec::new();
    let mut current_content = String::new();
    let mut in_comment_block = false;
    let mut in_ignore_block = false;
    let mut in_code_example = false;
    let mut in_quiz = false;
    let mut current_quiz_name = String::new();
    let mut current_quiz_content = String::new();

    for (_i, line) in content.lines().enumerate() {
        let line = line.trim_end();

        // Check for entering/exiting ignore blocks with --#--
        if line == "--#--" {
            in_ignore_block = !in_ignore_block;
            continue;
        }

        // Skip processing while in ignore block
        if in_ignore_block {
            continue;
        }

        // Inside comment blocks, check for quiz markers
        if in_comment_block {
            // Start of quiz
            if line.starts_with("--@quiz:") && !in_quiz {
                // Extract quiz name
                current_quiz_name = line[8..].trim().to_string();
                in_quiz = true;
                current_quiz_content.clear();
                continue;
            }

            // End of quiz
            if line == "--@quiz-end" && in_quiz {
                in_quiz = false;
                // Store the quiz
                quizzes.push((current_quiz_name.clone(), current_quiz_content.clone()));
                // Add a block with the quiz reference
                if !current_content.trim().is_empty() {
                    blocks.push(Block {
                        content: current_content.trim().to_string(),
                        is_code: false,
                        is_admonish: false,
                        quiz_reference: None,
                    });
                    current_content = String::new();
                }
                blocks.push(Block {
                    content: String::new(),
                    is_code: false,
                    is_admonish: false,
                    quiz_reference: Some(current_quiz_name.clone()),
                });
                continue;
            }

            // Collect quiz content if in quiz mode
            if in_quiz {
                current_quiz_content.push_str(line);
                current_quiz_content.push('\n');
                continue;
            }
        }

        // Skip lines ending with --# regardless of context
        if line.ends_with("--#") {
            continue;
        }

        // Special handling for lines ending with --!
        if line.ends_with("--!") {
            // Extract content without the --! suffix and trim trailing whitespace
            let content = line[..line.len()-3].trim_end().to_string();

            // Add this line directly to the current content block
            current_content.push_str(&content);
            current_content.push('\n');

            continue; // Skip further processing for this line
        }

        // Check for code block markers inside comments
        if in_comment_block && line.trim() == "```lean" {
            in_code_example = true;
            current_content.push_str(line);
            current_content.push('\n');
            continue;
        }

        if in_comment_block && line.trim() == "```" && in_code_example {
            in_code_example = false;
            current_content.push_str(line);
            current_content.push('\n');
            continue;
        }

        // When in a code example inside a comment, preserve everything as-is
        if in_comment_block && in_code_example {
            current_content.push_str(line);
            current_content.push('\n');
            continue;
        }

        // Inside comment blocks, don't interpret docstring markers that appear in the middle of text
        if in_comment_block {
            // Skip lines that end with --# even inside comment blocks
            if line.ends_with("--#") {
                continue;
            }

        // Check if this line contains the end of a comment block
        if line.trim_end().ends_with("-/") && !line.contains("```") {
            let is_admonish = line.ends_with("--+");

            // Add the content up to the -/ marker (removing the -/ itself)
            if let Some(end_idx) = line.rfind("-/") {
                // Only add the part before the -/ marker
                current_content.push_str(&line[0..end_idx]);
                current_content.push('\n');
            } else {
                // Fallback: add the whole line
                current_content.push_str(line);
                current_content.push('\n');
            }

            // Add the comment block
            blocks.push(Block {
                content: current_content.trim().to_string(),
                is_code: false,
                is_admonish,
                quiz_reference: None,
            });

            in_comment_block = false;
            current_content = String::new();
        } else {
            // Just add the line as-is to the current comment block
            current_content.push_str(line);
            current_content.push('\n');
        }
        continue;
        }

        // Skip lines that end with --#
        if line.ends_with("--#") {
            continue;
        }

        // Check for docstring with --+ at the end (special admonish block)
        if line.starts_with("/--") &&
           line.contains("-/") &&
           line.ends_with("--+") {
            // Handle as a special admonish block
            if !current_content.trim().is_empty() {
                blocks.push(Block {
                    content: current_content.trim().to_string(),
                    is_code: true,
                    is_admonish: false,
                    quiz_reference: None,
                });
            }

            // Extract content between markers
            let start_idx = 3; // Skip the "/--"
            let end_idx = line.rfind("-/").unwrap();

            if start_idx <= end_idx {
                let comment_text = &line[start_idx..end_idx];
                blocks.push(Block {
                    content: comment_text.trim().to_string(),
                    is_code: false,
                    is_admonish: true, // Mark as admonish block
                    quiz_reference: None,
                });
            }

            current_content = String::new();
            continue;
        }

        // Handle single-line docstrings (/--...-/) - treat as code
        if line.starts_with("/--") && line.contains("-/") && !in_comment_block {
            // Add the whole line as code
            current_content.push_str(line);
            current_content.push('\n');
            continue;
        }

        // Handle single-line comments that start and end on the same line
        if line.starts_with("/-") && !line.starts_with("/--") && line.contains("-/") && !in_comment_block {
            // If we have accumulated code content, add it as a code block
            if !current_content.trim().is_empty() {
                blocks.push(Block {
                    content: current_content.trim().to_string(),
                    is_code: true,
                    is_admonish: false,
                    quiz_reference: None,
                });
            }

            // Extract the comment text between /- and -/
            let start_idx = line.find("/-").unwrap() + 2;
            let end_idx = line.rfind("-/").unwrap();

            if start_idx <= end_idx {
                let comment_text = &line[start_idx..end_idx];
                blocks.push(Block {
                    content: comment_text.trim().to_string(),
                    is_code: false,
                    is_admonish: false,
                    quiz_reference: None,
                });
            }

            // If there's any content after the comment, start collecting it
            if let Some(rest) = line.split("-/").nth(1) {
                if !rest.trim().is_empty() && !rest.trim_end().ends_with("--+") {
                    current_content = rest.to_string();
                    current_content.push('\n');
                } else {
                    current_content = String::new();
                }
            } else {
                current_content = String::new();
            }

            continue;
        }

        // Handle opening of a comment block
        if line.starts_with("/-") && !line.starts_with("/--") && !in_comment_block {
            // If we have accumulated code content, add it as a code block
            if !current_content.trim().is_empty() {
                blocks.push(Block {
                    content: current_content.trim().to_string(),
                    is_code: true,
                    is_admonish: false,
                    quiz_reference: None,
                });
            }

            in_comment_block = true;
            // Skip the /- marker
            current_content = line[2..].to_string();
            current_content.push('\n');
            continue;
        }

        // Add the line to the current content
        current_content.push_str(line);
        current_content.push('\n');
    }

    // Add any remaining content
    if !current_content.trim().is_empty() {
        blocks.push(Block {
            content: current_content.trim().to_string(),
            is_code: !in_comment_block,
            is_admonish: false,
            quiz_reference: None,
        });
    }

    // If we're still in a comment block at the end, that's an error
    if in_comment_block {
        return Err("Unclosed comment block at the end of file".to_string());
    }

    Ok((blocks, quizzes))
}

fn merge_blocks(blocks: &[Block]) -> String {
    let mut result = String::new();

    for block in blocks {
        if block.content.is_empty() && block.quiz_reference.is_none() {
            continue;
        }

        // Handle quiz references
        if let Some(quiz_ref) = &block.quiz_reference {
            result.push_str(&format!("{{{{#quiz ../quizzes/{}.toml}}}}\n\n", quiz_ref));
            continue;
        }

        if block.is_code {
            result.push_str("```lean\n");
            result.push_str(&block.content);
            result.push_str("\n```\n\n");
        } else if block.is_admonish {
            // Format as admonish block
            result.push_str("```admonish abstract collapsible = false, title = \"Docstring\"\n");
            result.push_str(&block.content);
            result.push_str("\n```\n\n");
        } else {
            result.push_str(&block.content);
            result.push_str("\n\n");
        }
    }

    result.trim_end().to_string() + "\n"
}

/// Processes a directory of Lean files and converts them to Markdown
///
/// # Arguments
///
/// * `src_dir` - Path to the source directory containing Lean files
/// * `tgt_dir` - Path to the target directory where Markdown files will be created
///
/// # Returns
///
/// Result containing `()` on success or an error message on failure
pub fn process_directory(src_dir: &Path, tgt_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Create the target directory if it doesn't exist
    fs::create_dir_all(tgt_dir)?;

    // Create quizzes directory at the same level as the target directory
    let parent_dir = tgt_dir.parent().unwrap_or(Path::new("."));
    let quizzes_dir = parent_dir.join("quizzes");
    fs::create_dir_all(&quizzes_dir)?;

    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Recursively process subdirectories
            let src_subdir = path.file_name().unwrap();
            let tgt_subdir = tgt_dir.join(src_subdir);
            process_directory(&path, &tgt_subdir)?;
        } else if let Some(ext) = path.extension() {
            if ext == "lean" {
                // Process lean file
                let content = fs::read_to_string(&path)?;

                // Get the output path
                let md_path = tgt_dir.join(path.file_stem().unwrap()).with_extension("md");

                // Display progress information
                println!("Converting {} to {}", path.display(), md_path.display());

                // Parse blocks and extract quizzes
                let (blocks, quizzes) = build_blocks(&content)?;

                // Generate markdown content
                let markdown = merge_blocks(&blocks);

                // Write quiz TOML files
                for (name, content) in quizzes {
                    let quiz_path = quizzes_dir.join(format!("{}.toml", name));
                    let mut file = File::create(&quiz_path)?;
                    file.write_all(content.as_bytes())?;

                    // Also show when a quiz file is created
                    println!("  Generated quiz: {}", quiz_path.display());
                }

                // Create output markdown file
                fs::write(md_path, markdown)?;
            }
        }
    }

    Ok(())
}