//! # Local Documentation Generation
//!
//! This module provides functionality to generate rustdoc JSON documentation
//! from local Rust crate projects. It uses the `rustdoc_json_stable` crate
//! to invoke cargo doc and extract the resulting JSON data.
//!
//! ## Features
//!
//! - **Toolchain Support**: Works with both stable and nightly Rust toolchains
//! - **Feature Control**: Generate docs with all features or specific feature sets
//! - **Auto-detection**: Automatically detect the appropriate toolchain to use
//! - **Error Handling**: Comprehensive error reporting for build failures

use std::path::PathBuf;

use crate::error::{Error, Result};
use crate::temp_trait::Crate;

/// Container for generated documentation data and metadata.
///
/// This structure holds the results of local documentation generation,
/// including the extracted crate name and the complete documentation data.
#[derive(Debug, Clone)]
pub struct GenDocs {
  /// The name of the crate that was documented
  pub lib_name: String,
  /// Complete documentation data in our internal format
  pub docs: Crate,
}

/// Generate documentation for a local crate with all features enabled.
///
/// This function uses `cargo doc --all-features` to generate comprehensive
/// documentation that includes all optional features and dependencies.
/// The generated JSON is then parsed and returned as structured data.
///
/// # Arguments
///
/// * `toolchain` - Rust toolchain to use ("stable", "nightly", etc.)
/// * `manifest_path` - Path to the target crate's Cargo.toml file
///
/// # Returns
///
/// * `Result<GenDocs>` - Generated documentation data and metadata
///
/// # Errors
///
/// * `Error::Build` - If cargo doc fails to build the documentation
/// * `Error::Io` - If the manifest path is invalid or JSON file cannot be read
/// * `Error::Json` - If the generated JSON cannot be parsed
/// * `Error::Config` - If the crate name cannot be extracted
///
/// # Examples
///
/// ```no_run
/// use std::path::PathBuf;
/// use crates_llms_txt::gen_docs::gen_docs_with_all_features;
///
/// let docs = gen_docs_with_all_features(
///     "stable",
///     PathBuf::from("./Cargo.toml")
/// )?;
///
/// println!("Generated docs for: {}", docs.lib_name);
/// ```
pub fn gen_docs_with_all_features(
  toolchain: &str,
  manifest_path: PathBuf,
) -> Result<GenDocs> {
  // Configure the rustdoc builder based on the specified toolchain
  let json_path = match toolchain {
    "nightly" => rustdoc_json_stable::Builder::default(),
    _ => rustdoc_json_stable::Builder::stable(),
  }
  .toolchain(toolchain)
  .manifest_path(manifest_path)
  .all_features(true) // Enable all available features for comprehensive docs
  .quiet(true) // Suppress cargo output for cleaner execution
  .build()?;

  // Extract the crate name from the generated JSON filename
  let lib_name = json_path
    .as_path()
    .file_stem()
    .and_then(|stem| stem.to_str())
    .map(String::from)
    .ok_or("Failed to extract library name from generated JSON path")
    .map_err(|e| Error::Config(e.to_string()))?;

  // Read and parse the generated JSON documentation
  let json_string = std::fs::read_to_string(&json_path)?;
  let json_data: Crate = serde_json::from_str(&json_string)?;

  Ok(GenDocs {
    lib_name,
    docs: json_data,
  })
}

/// Generate docs for a given Cargo.toml file using the automatically detected toolchain.
///
/// This function detects whether the current Rust installation is nightly or stable
/// and uses the appropriate toolchain to generate documentation.
///
/// # Arguments
///
/// * `manifest_path` - The path to the Cargo.toml file.
///
/// # Returns
///
/// * `Result<GenDocs, Box<dyn std::error::Error>>` - The generated documentation data.
///
/// # Examples
///
/// ```no_run
/// use std::path::PathBuf;
/// let docs = gen_docs_with_all_features_auto_toolchain(PathBuf::from("Cargo.toml")).unwrap();
/// ```
pub fn gen_docs_with_all_features_auto_toolchain(
  manifest_path: PathBuf,
) -> Result<GenDocs> {
  // Auto-detect the appropriate toolchain based on the current Rust installation
  let json_path = match rustversion::cfg!(nightly) {
    true => rustdoc_json_stable::Builder::default().toolchain("nightly"),
    false => rustdoc_json_stable::Builder::stable().toolchain("stable"),
  }
  .manifest_path(manifest_path)
  .all_features(true) // Enable all features for comprehensive documentation
  .quiet(true) // Suppress cargo output
  .build()?;

  // Extract crate name from the generated JSON file path
  let lib_name = json_path
    .as_path()
    .file_stem()
    .and_then(|stem| stem.to_str())
    .map(String::from)
    .ok_or("Failed to extract library name from generated JSON path")
    .map_err(|e| Error::Config(e.to_string()))?;

  // Load and parse the generated documentation JSON
  let json_string = std::fs::read_to_string(&json_path)?;
  let json_data: Crate = serde_json::from_str(&json_string)?;

  Ok(GenDocs {
    lib_name,
    docs: json_data,
  })
}

/// Generate documentation for a local crate with custom feature configuration.
///
/// This function provides fine-grained control over which features are enabled
/// during documentation generation. It allows disabling default features and
/// specifying exactly which optional features should be included.
///
/// # Arguments
///
/// * `toolchain` - Rust toolchain to use ("stable", "nightly", etc.)
/// * `manifest_path` - Path to the target crate's Cargo.toml file
/// * `no_default_features` - If true, disables all default features
/// * `features` - Optional list of specific features to enable
///
/// # Returns
///
/// * `Result<GenDocs>` - Generated documentation data and metadata
///
/// # Errors
///
/// * `Error::Build` - If cargo doc fails (e.g., feature conflicts, missing dependencies)
/// * `Error::Io` - If the manifest path is invalid or JSON file cannot be read
/// * `Error::Json` - If the generated JSON cannot be parsed
/// * `Error::Config` - If the crate name cannot be extracted
///
/// # Examples
///
/// ```no_run
/// use std::path::PathBuf;
/// use crates_llms_txt::gen_docs::gen_docs_with_features;
///
/// // Generate docs with only specific features
/// let docs = gen_docs_with_features(
///     "stable",
///     PathBuf::from("./Cargo.toml"),
///     true, // disable default features
///     Some(vec!["async".to_string(), "json".to_string()])
/// )?;
///
/// // Generate docs with default features plus additional ones
/// let docs = gen_docs_with_features(
///     "stable",
///     PathBuf::from("./Cargo.toml"),
///     false, // keep default features
///     Some(vec!["experimental".to_string()])
/// )?;
/// ```
pub fn gen_docs_with_features(
  toolchain: &str,
  manifest_path: PathBuf,
  no_default_features: bool,
  features: Option<Vec<String>>,
) -> Result<GenDocs> {
  // Initialize the builder with the appropriate toolchain
  let mut builder = match toolchain {
    "nightly" => rustdoc_json_stable::Builder::default(),
    _ => rustdoc_json_stable::Builder::stable(),
  }
  .toolchain(toolchain)
  .manifest_path(manifest_path)
  .quiet(true); // Suppress cargo output for cleaner execution

  // Configure feature settings based on parameters
  if no_default_features {
    builder = builder.no_default_features(true);
  }

  if let Some(feature_list) = features {
    builder = builder.features(feature_list);
  }

  // Generate the documentation JSON
  let json_path = builder.build()?;

  // Extract the crate name from the generated file
  let lib_name = json_path
    .as_path()
    .file_stem()
    .and_then(|stem| stem.to_str())
    .map(String::from)
    .ok_or("Failed to extract library name from generated JSON path")
    .map_err(|e| Error::Config(e.to_string()))?;

  // Read and parse the generated documentation
  let json_string = std::fs::read_to_string(&json_path)?;
  let json_data: Crate = serde_json::from_str(&json_string)?;

  Ok(GenDocs {
    lib_name,
    docs: json_data,
  })
}

/// Generate docs for a given Cargo.toml file using automatically detected toolchain.
///
/// This function detects whether the current Rust installation is nightly or stable
/// and uses the appropriate toolchain to generate documentation with specified features.
///
/// # Arguments
///
/// * `manifest_path` - The path to the Cargo.toml file.
/// * `no_default_features` - Whether to disable default features.
/// * `features` - Optional list of specific features to enable.
///
/// # Returns
///
/// * `Result<GenDocs, Box<dyn std::error::Error>>` - The generated documentation data.
///
/// # Examples
///
/// ```no_run
/// use std::path::PathBuf;
/// let docs = gen_docs_with_features_auto_toolchain(
///     PathBuf::from("Cargo.toml"),
///     true,
///     Some(vec!["async".to_string()])
/// ).unwrap();
/// ```
pub fn gen_docs_with_features_auto_toolchain(
  manifest_path: PathBuf,
  no_default_features: bool,
  features: Option<Vec<String>>,
) -> Result<GenDocs> {
  // Auto-detect and configure the appropriate toolchain
  let mut builder = match rustversion::cfg!(nightly) {
    true => rustdoc_json_stable::Builder::default().toolchain("nightly"),
    false => rustdoc_json_stable::Builder::stable().toolchain("stable"),
  }
  .manifest_path(manifest_path)
  .quiet(true); // Suppress cargo output

  // Apply feature configuration
  if no_default_features {
    builder = builder.no_default_features(true);
  }

  if let Some(feature_list) = features {
    builder = builder.features(feature_list);
  }

  // Build the documentation and extract metadata
  let json_path = builder.build()?;
  let lib_name = json_path
    .as_path()
    .file_stem()
    .and_then(|stem| stem.to_str())
    .map(String::from)
    .ok_or("Failed to extract library name from generated JSON path")
    .map_err(|e| Error::Config(e.to_string()))?;

  // Load and parse the documentation data
  let json_string = std::fs::read_to_string(&json_path)?;
  let json_data: Crate = serde_json::from_str(&json_string)?;

  Ok(GenDocs {
    lib_name,
    docs: json_data,
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::gen_docs::{gen_docs_with_all_features, gen_docs_with_features};
  use std::env;

  #[cfg(feature = "rustdoc")]
  #[test]
  fn test_gen_docs_with_all_features_success() {
    let current_dir = std::env::current_dir().unwrap();
    let gen_docs_struct =
      gen_docs_with_all_features("stable", current_dir.join("Cargo.toml"))
        .unwrap();

    assert_eq!(gen_docs_struct.lib_name, "crates_llms_txt");
  }

  #[cfg(feature = "rustdoc")]
  #[test]
  fn test_gen_docs_with_features_success() {
    let current_dir = std::env::current_dir().unwrap();
    let gen_docs_struct = gen_docs_with_features(
      "stable",
      current_dir.join("Cargo.toml"),
      true,
      Some(vec!["rustdoc".to_string()]),
    )
    .unwrap();
    assert_eq!(gen_docs_struct.lib_name, "crates_llms_txt");
  }

  #[cfg(feature = "rustdoc")]
  #[test]
  fn test_gen_docs_with_all_features_auto_toolchain() {
    let current_dir = env::current_dir().unwrap();
    let gen_docs_struct =
      gen_docs_with_all_features_auto_toolchain(current_dir.join("Cargo.toml"))
        .unwrap();

    assert_eq!(gen_docs_struct.lib_name, "crates_llms_txt");
  }

  #[cfg(feature = "rustdoc")]
  #[test]
  #[should_panic(expected = "No such file or directory")]
  fn test_gen_docs_with_all_features_auto_toolchain_invalid_path() {
    let invalid_path = PathBuf::from("nonexistent/Cargo.toml");
    gen_docs_with_all_features_auto_toolchain(invalid_path).unwrap();
  }

  #[cfg(feature = "rustdoc")]
  #[test]
  fn test_gen_docs_with_features_auto_toolchain_success() {
    let current_dir = env::current_dir().unwrap();
    let gen_docs_struct = gen_docs_with_features_auto_toolchain(
      current_dir.join("Cargo.toml"),
      true,
      Some(vec!["rustdoc".to_string()]),
    )
    .unwrap();

    assert_eq!(gen_docs_struct.lib_name, "crates_llms_txt");
  }

  #[cfg(feature = "rustdoc")]
  #[test]
  #[should_panic(expected = "No such file or directory")]
  fn test_gen_docs_with_features_auto_toolchain_invalid_path() {
    let invalid_path = PathBuf::from("nonexistent/Cargo.toml");
    gen_docs_with_features_auto_toolchain(invalid_path, false, None).unwrap();
  }

  #[cfg(feature = "rustdoc")]
  #[test]
  fn test_gen_docs_with_features_auto_toolchain_no_features() {
    let current_dir = env::current_dir().unwrap();
    let gen_docs_struct = gen_docs_with_features_auto_toolchain(
      current_dir.join("Cargo.toml"),
      false,
      None,
    )
    .unwrap();

    assert_eq!(gen_docs_struct.lib_name, "crates_llms_txt");
  }
}
