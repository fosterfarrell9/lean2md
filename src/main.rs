use std::env;
use std::path::PathBuf;
use lean2md::process_directory;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "--version" {
        // Define version here since it's not exported from the library
        let version = env!("CARGO_PKG_VERSION");
        println!("lean2md version {}", version);
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