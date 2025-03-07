use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
struct Block {
    content: String,
    is_code: bool,
    is_admonish: bool,
}

fn build_blocks(content: &str) -> Result<Vec<Block>, String> {
    let mut blocks = Vec::new();
    let mut current_content = String::new();
    let mut in_comment_block = false;
    let mut in_ignore_block = false;
    let mut in_code_example = false;
    
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
        });
    }
    
    // If we're still in a comment block at the end, that's an error
    if in_comment_block {
        return Err("Unclosed comment block at the end of file".to_string());
    }
    
    Ok(blocks)
}

fn merge_blocks(blocks: &[Block]) -> String {
    let mut result = String::new();
    
    for block in blocks {
        if block.content.is_empty() {
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

fn lean_file_2_md(filename: &Path) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    
    let blocks = build_blocks(&content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    
    Ok(merge_blocks(&blocks))
}

// New function to recursively process directories
fn process_directory(src_dir: &Path, tgt_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Create the target directory if it doesn't exist
    fs::create_dir_all(tgt_dir)?;
    
    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            // Get the directory name
            let dir_name = path.file_name().unwrap();
            
            // Create corresponding target directory
            let new_tgt_dir = tgt_dir.join(dir_name);
            
            // Process the subdirectory recursively
            process_directory(&path, &new_tgt_dir)?;
        } else if path.is_file() && path.extension().map_or(false, |ext| ext == "lean") {
            // Get the file name without extension
            let file_stem = path.file_stem().unwrap().to_str().unwrap();
            let tgt_filename = tgt_dir.join(format!("{}.md", file_stem));
            
            println!("Converting {} to {}", path.display(), tgt_filename.display());
            
            // Convert and write the file
            let md_content = lean_file_2_md(&path)?;
            let mut file = File::create(tgt_filename)?;
            file.write_all(md_content.as_bytes())?;
        }
    }
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "--version" {
        println!("lean2md version {}", VERSION);
        return Ok(());
    }    

    if args.len() != 3 {
        println!("Usage: lean2md <lean_src_dir> <md_tgt_dir>");
        return Ok(());
    }
    
    let src = PathBuf::from(&args[1]);
    let tgt = PathBuf::from(&args[2]);
    
    process_directory(&src, &tgt)?;
    
    Ok(())
}