use lean2md::{process_directory, process_file};
use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        if args[1] == "--version" {
            let version = env!("CARGO_PKG_VERSION");
            println!("lean2md version {}", version);
            return Ok(());
        } else {
            // Case: lean2md <file.lean>
            let src = PathBuf::from(&args[1]);

            if src.is_file() && src.extension().is_some_and(|ext| ext == "lean") {
                let tgt = src.with_extension("md");
                return process_file(&src, &tgt);
            }
        }
    } else if args.len() == 3 {
        let src = PathBuf::from(&args[1]);
        let tgt = PathBuf::from(&args[2]);

        if src.is_file() {
            // Case: lean2md <lean_src_file> <md_tgt_file>
            return process_file(&src, &tgt);
        } else if src.is_dir() {
            // Case: lean2md <lean_src_dir> <md_tgt_dir>
            return process_directory(&src, &tgt);
        }
    }

    // If we reach here, arguments were invalid
    println!("Usage:");
    println!("  lean2md <file.lean>                  # Convert to <file.md>");
    println!("  lean2md <lean_src_file> <md_tgt_file>   # Convert file to file");
    println!("  lean2md <lean_src_dir> <md_tgt_dir>     # Convert directory to directory");
    println!("  lean2md --version                    # Display version information");
    Ok(())
}
