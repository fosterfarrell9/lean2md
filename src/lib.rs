mod lean2md_core; // Move core functionality to this module

// Export public functions for other crates to use
pub use lean2md_core::{process_directory, build_blocks, Block};