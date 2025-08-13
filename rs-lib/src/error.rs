//! # Error Types and Result Definitions
//!
//! This module defines the error types used throughout the crates_llms_txt library.
//! It provides comprehensive error handling for network operations, JSON parsing,
//! file I/O, and documentation generation.

use thiserror::Error;

/// Comprehensive error type for all operations in the crates_llms_txt library.
///
/// This enum covers all possible error conditions that can occur during
/// documentation fetching, parsing, and generation operations.
#[derive(Error, Debug)]
pub enum Error {
  /// Network-related errors from HTTP requests to docs.rs or other servers
  ///
  /// This includes connection failures, timeouts, HTTP error status codes,
  /// and other network-level issues when fetching documentation.
  #[error("network error: {0}")]
  Network(#[from] reqwest::Error),
  
  /// JSON parsing and serialization errors
  ///
  /// Occurs when rustdoc JSON data cannot be parsed, typically due to
  /// format incompatibilities or corrupted data.
  #[error("JSON parsing error: {0}")]
  Json(#[from] serde_json::Error),
  
  /// File system and I/O related errors
  ///
  /// Includes file not found, permission denied, disk full, and other
  /// file system operations that can fail.
  #[error("I/O error: {0}")]
  Io(#[from] std::io::Error),
  
  /// Documentation build errors when generating local documentation
  ///
  /// This error type is only available when the "rustdoc" feature is enabled.
  /// It covers cargo doc failures, toolchain issues, and build problems.
  #[cfg(feature = "rustdoc")]
  #[error("rustdoc build error: {0}")]
  Build(#[from] rustdoc_json_stable::BuildError),
  
  /// Configuration and validation errors
  ///
  /// Used for invalid crate names, version mismatches, missing required
  /// fields, and other configuration-related issues.
  #[error("configuration error: {0}")]
  Config(String),
}

/// Convenience type alias for Results using our Error type.
///
/// This allows for more concise error handling throughout the library
/// without having to specify the Error type repeatedly.
pub type Result<T, E = Error> = std::result::Result<T, E>;
