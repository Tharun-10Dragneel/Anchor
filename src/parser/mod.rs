//! Parser module — tree-sitter based multi-language AST extraction.
//!
//! Handles parsing source files into structured symbol data
//! that feeds into the code graph.

pub mod extractor;
pub mod language;

pub use extractor::extract_file;
pub use language::SupportedLanguage;
