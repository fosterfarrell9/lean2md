//! # lean2md
//!
//! A library for converting Lean files to Markdown with special features for documentation.
//!
//! ## Features
//!
//! - Converts Lean comments to Markdown text
//! - Maintains Lean code blocks inside Markdown code fences
//! - Supports special markers for controlling output
//! - Handles quiz generation for mdbook-quiz integration

mod lean2md_core; // Move core functionality to this module

// Export public functions for other crates to use
pub use lean2md_core::{build_blocks, process_directory, process_file, Block};
