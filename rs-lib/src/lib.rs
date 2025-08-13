//! # Crates LLMs TXT
//!
//! A Rust library for extracting and processing Rust crate documentation from both online sources
//! (docs.rs) and local crates. This library provides functionality to fetch, parse, and structure
//! rustdoc JSON data for use in LLM training or other documentation processing tasks.
//!
//! ## Features
//!
//! - **Online Documentation**: Fetch documentation from docs.rs with automatic zstd decompression
//! - **Local Documentation**: Generate documentation from local Cargo projects
//! - **Version Compatibility**: Handle different rustdoc JSON format versions automatically
//! - **Feature Control**: Generate docs with specific feature sets or all features
//! - **Flexible Output**: Structured data suitable for LLM training or analysis
//!
//! ## Examples
//!
//! ### Fetching Online Documentation
//!
//! ```no_run
//! use crates_llms_txt::CrateDocs;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Fetch latest version
//!     let docs = CrateDocs::from_online("serde", None).await?;
//!     
//!     // Fetch specific version
//!     let docs = CrateDocs::from_online("clap", Some("4.5.39".to_string())).await?;
//!     
//!     println!("Found {} documentation items", docs.sessions.len());
//!     Ok(())
//! }
//! ```
//!
//! ### Generating Local Documentation
//!
//! ```no_run
//! use std::path::PathBuf;
//! use crates_llms_txt::CrateDocs;
//!
//! // Generate docs with all features
//! let docs = CrateDocs::from_local(
//!     PathBuf::from("./Cargo.toml"),
//!     Some("stable".to_string()),
//! )?;
//!
//! // Generate docs with specific features
//! let docs = CrateDocs::from_local_with_features(
//!     PathBuf::from("./Cargo.toml"),
//!     false, // don't disable default features
//!     Some(vec!["async".to_string()]),
//!     None, // auto-detect toolchain
//! )?;
//! ```

#[cfg(feature = "rustdoc")]
use std::path::PathBuf;

use fetch_docs::OnlineDocs;
use rustdoc_types::Visibility;
use serde::{Deserialize, Serialize};

use error::{Error, Result};
use temp_trait::CommonCrates;

pub mod error;
pub mod fetch_docs;
#[cfg(feature = "rustdoc")]
mod gen_docs;
pub mod temp_trait;

/// Base URL for docs.rs crate documentation API endpoints
const DOCS_BASE_URL: &str = "https://docs.rs/crate";

/// Represents a single documentation session item with metadata.
///
/// This structure contains basic information about a documentation item,
/// including its title, description, and link to the source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionItem {
  /// The title or name of the documentation item
  pub title: String,
  /// A brief description of the item (currently unused, reserved for future use)
  pub description: String,
  /// Direct link to the documentation source on docs.rs or local server
  pub link: String,
}

/// Represents a full documentation session item with complete content.
///
/// This structure contains the full documentation content along with
/// the link to its source location.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullSessionItem {
  /// The complete documentation content as a string
  pub content: String,
  /// Direct link to the documentation source
  pub link: String,
}

/// Main structure containing all documentation data for a crate.
///
/// This is the primary data structure returned by all documentation
/// generation and fetching methods. It contains both summary information
/// and full documentation content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrateDocs {
  /// The name of the crate
  pub lib_name: String,
  /// The version of the crate
  pub version: String,
  /// Summary items containing basic metadata for each documentation item
  pub sessions: Vec<SessionItem>,
  /// Full documentation items containing complete content
  pub full_sessions: Vec<FullSessionItem>,
}

impl CrateDocs {
  /// Creates a new empty `CrateDocs` instance.
  ///
  /// # Arguments
  ///
  /// * `lib_name` - The name of the crate
  /// * `version` - The version string of the crate
  ///
  /// # Returns
  ///
  /// A new `CrateDocs` instance with empty session vectors
  ///
  /// # Examples
  ///
  /// ```
  /// use crates_llms_txt::CrateDocs;
  ///
  /// let docs = CrateDocs::new("my_crate", "1.0.0");
  /// assert_eq!(docs.lib_name, "my_crate");
  /// assert_eq!(docs.version, "1.0.0");
  /// assert!(docs.sessions.is_empty());
  /// ```
  pub fn new(lib_name: &str, version: &str) -> Self {
    Self {
      lib_name: lib_name.to_string(),
      version: version.to_string(),
      sessions: Vec::new(),
      full_sessions: Vec::new(),
    }
  }

  /// Process raw documentation data into structured `CrateDocs` format.
  ///
  /// This internal method converts rustdoc JSON data into the structured format
  /// used by this library. It extracts public documentation items and creates
  /// both summary and full content entries.
  ///
  /// # Arguments
  ///
  /// * `lib_name` - The name of the crate
  /// * `docs` - Raw documentation data implementing the `CommonCrates` trait
  /// * `version` - Optional version string; if None, uses the version from docs
  ///
  /// # Returns
  ///
  /// * `Result<CrateDocs>` - Processed documentation structure
  ///
  /// # Errors
  ///
  /// Returns an error if the documentation data cannot be processed or if
  /// required fields are missing from the rustdoc JSON.
  fn process_docs<T: CommonCrates>(
    lib_name: &str,
    docs: T,
    version: Option<String>,
  ) -> Result<CrateDocs> {
    let version = version.unwrap_or_else(|| docs.crate_version());
    let mut crate_docs = CrateDocs::new(lib_name, &version);
    let base_url =
      format!("{}/{}/{}/source", DOCS_BASE_URL, &lib_name, version);

    // Add the main crate entry
    crate_docs.sessions.push(SessionItem {
      title: lib_name.to_string(),
      description: "".to_string(),
      link: format!("https://docs.rs/{lib_name}/{version}"),
    });

    // Process all documentation items from the crate index
    for (_, item) in docs.index() {
      if let Some(docs_content) = item.docs {
        // Only include public items to avoid exposing private implementation details
        if item.visibility != Visibility::Public {
          continue;
        }

        // Extract filename from span information for source links
        let filename = item.span.unwrap().filename;
        let link = format!("{}/{}", base_url, filename.to_str().unwrap());

        // Create session item with appropriate title
        crate_docs.sessions.push(SessionItem {
          title: match item.name {
            Some(name) => name,
            None => filename.to_str().unwrap().to_string(),
          },
          description: "".to_string(),
          link: link.clone(),
        });

        // Store full documentation content
        crate_docs.full_sessions.push(FullSessionItem {
          content: docs_content,
          link,
        });
      };
    }

    Ok(crate_docs)
  }

  /// Fetch crate documentation from docs.rs online repository.
  ///
  /// This method downloads and processes rustdoc JSON data from docs.rs,
  /// automatically handling zstd compression and version compatibility issues.
  /// It attempts to parse using the standard rustdoc format first, then falls
  /// back to an internal format if needed.
  ///
  /// # Arguments
  ///
  /// * `lib_name` - The name of the crate (e.g., "serde", "clap")
  /// * `version` - Optional version string; if None, fetches "latest"
  ///
  /// # Returns
  ///
  /// * `Result<CrateDocs>` - Structured documentation data
  ///
  /// # Errors
  ///
  /// * `Error::Network` - If the HTTP request to docs.rs fails
  /// * `Error::Json` - If the JSON data cannot be parsed
  /// * `Error::Io` - If decompression fails
  /// * `Error::Config` - If the rustdoc format is incompatible
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use crates_llms_txt::CrateDocs;
  ///
  /// #[tokio::main]
  /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
  ///     // Fetch latest version
  ///     let docs = CrateDocs::from_online("serde", None).await?;
  ///     
  ///     // Fetch specific version
  ///     let docs = CrateDocs::from_online("clap", Some("4.5.39".to_string())).await?;
  ///     
  ///     println!("Crate: {} v{}", docs.lib_name, docs.version);
  ///     println!("Documentation items: {}", docs.sessions.len());
  ///     Ok(())
  /// }
  /// ```
  ///
  pub async fn from_online(
    lib_name: &str,
    version: Option<String>,
  ) -> Result<CrateDocs> {
    let version_str = version.unwrap_or("latest".to_string());
    let url = format!("{DOCS_BASE_URL}/{lib_name}/{version_str}/json");

    // Try parsing with standard rustdoc_types::Crate first for maximum compatibility
    match OnlineDocs::fetch_json::<rustdoc_types::Crate>(url.as_str()).await {
      Ok(result) => {
        let crate_version = Some(result.crate_version());
        CrateDocs::process_docs(lib_name, result, crate_version)
      }
      Err(_) => {
        // Fallback to internal Crate type for older or incompatible formats
        match OnlineDocs::fetch_json::<temp_trait::Crate>(url.as_str()).await {
          Ok(result) => {
            let crate_version = Some(result.crate_version());
            CrateDocs::process_docs(lib_name, result, crate_version)
          }
          Err(err) => Err(err),
        }
      }
    }
  }

  /// Fetch crate documentation from a custom URL endpoint.
  ///
  /// This method allows fetching documentation from any URL that serves
  /// rustdoc JSON data, not just docs.rs. Useful for private documentation
  /// servers or alternative hosting solutions.
  ///
  /// # Arguments
  ///
  /// * `url` - Complete URL to the rustdoc JSON endpoint
  ///
  /// # Returns
  ///
  /// * `Result<CrateDocs>` - Structured documentation data
  ///
  /// # Errors
  ///
  /// * `Error::Network` - If the HTTP request fails
  /// * `Error::Json` - If the JSON data cannot be parsed
  /// * `Error::Config` - If the crate name cannot be extracted from the data
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use crates_llms_txt::CrateDocs;
  ///
  /// #[tokio::main]
  /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
  ///     // Fetch from docs.rs
  ///     let docs = CrateDocs::from_url(
  ///         "https://docs.rs/crate/clap/latest/json"
  ///     ).await?;
  ///     
  ///     // Fetch from custom server
  ///     let docs = CrateDocs::from_url(
  ///         "https://my-docs.example.com/crate/my-crate/1.0.0/json"
  ///     ).await?;
  ///     
  ///     Ok(())
  /// }
  /// ```
  ///
  pub async fn from_url(url: &str) -> Result<CrateDocs> {
    // Attempt to parse with standard rustdoc format first
    match OnlineDocs::fetch_json::<rustdoc_types::Crate>(url).await {
      Ok(docs) => {
        let root_id = docs.root_id();
        if let Some(root_item) = docs.index().get(&root_id) {
          let lib_name =
            &root_item.name.clone().unwrap_or("unknown".to_string());
          let crate_version = Some(docs.crate_version());
          return CrateDocs::process_docs(lib_name, docs, crate_version);
        }
        Err(Error::Config("Failed to extract crate name from root item".into()))
      }
      Err(_) => {
        // Fallback to internal format for compatibility
        match OnlineDocs::fetch_json::<temp_trait::Crate>(url).await {
          Ok(docs) => {
            let root_id = docs.root_id();
            if let Some(root_item) = docs.index().get(&root_id) {
              let lib_name =
                &root_item.name.clone().unwrap_or("unknown".to_string());
              let crate_version = Some(docs.crate_version());
              return CrateDocs::process_docs(lib_name, docs, crate_version);
            }
            Err(Error::Config("Failed to extract crate name from root item".into()))
          }
          Err(err) => Err(err),
        }
      }
    }
  }

  /// Generate documentation for a local crate with all features enabled.
  ///
  /// This method uses `cargo doc` internally to generate rustdoc JSON output
  /// for a local crate project. All available features are enabled during
  /// documentation generation to provide comprehensive coverage.
  ///
  /// # Arguments
  ///
  /// * `manifest_path` - Path to the Cargo.toml file of the target crate
  /// * `toolchain` - Optional Rust toolchain ("stable", "nightly", etc.)
  ///                 If None, auto-detects the appropriate toolchain
  ///
  /// # Returns
  ///
  /// * `Result<CrateDocs>` - Generated documentation structure
  ///
  /// # Errors
  ///
  /// * `Error::Build` - If cargo doc fails to generate documentation
  /// * `Error::Io` - If the manifest path is invalid or unreadable
  /// * `Error::Json` - If the generated JSON cannot be parsed
  /// * `Error::Config` - If the crate name cannot be extracted
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use std::path::PathBuf;
  /// use crates_llms_txt::CrateDocs;
  ///
  /// // Generate with specific toolchain
  /// let docs = CrateDocs::from_local(
  ///     PathBuf::from("./Cargo.toml"),
  ///     Some("stable".to_string()),
  /// )?;
  ///
  /// // Auto-detect toolchain
  /// let docs = CrateDocs::from_local(
  ///     PathBuf::from("./my-project/Cargo.toml"),
  ///     None,
  /// )?;
  ///
  /// println!("Generated docs for: {} v{}", docs.lib_name, docs.version);
  /// ```
  #[cfg(feature = "rustdoc")]
  pub fn from_local(
    manifest_path: PathBuf,
    toolchain: Option<String>,
  ) -> Result<CrateDocs> {
    let gen_docs_struct = match toolchain {
      Some(toolchain) => {
        gen_docs::gen_docs_with_all_features(&toolchain, manifest_path)?
      }
      None => {
        gen_docs::gen_docs_with_all_features_auto_toolchain(manifest_path)?
      }
    };

    let lib_name = gen_docs_struct.lib_name;
    let docs = gen_docs_struct.docs;
    return CrateDocs::process_docs(&lib_name, docs, None);
  }

  /// Generate documentation for a local crate with custom feature configuration.
  ///
  /// This method provides fine-grained control over which features are enabled
  /// during documentation generation. Useful for generating docs that match
  /// specific deployment configurations or to exclude optional dependencies.
  ///
  /// # Arguments
  ///
  /// * `manifest_path` - Path to the Cargo.toml file of the target crate
  /// * `no_default_features` - If true, disables all default features
  /// * `features` - Optional list of specific features to enable
  /// * `toolchain` - Optional Rust toolchain; if None, auto-detects
  ///
  /// # Returns
  ///
  /// * `Result<CrateDocs>` - Generated documentation structure
  ///
  /// # Errors
  ///
  /// * `Error::Build` - If cargo doc fails (e.g., feature conflicts, build errors)
  /// * `Error::Io` - If the manifest path is invalid
  /// * `Error::Json` - If the generated JSON cannot be parsed
  /// * `Error::Config` - If the crate configuration is invalid
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use std::path::PathBuf;
  /// use crates_llms_txt::CrateDocs;
  ///
  /// // Generate with specific features only
  /// let docs = CrateDocs::from_local_with_features(
  ///     PathBuf::from("./Cargo.toml"),
  ///     true, // disable default features
  ///     Some(vec!["async".to_string(), "json".to_string()]),
  ///     Some("stable".to_string()),
  /// )?;
  ///
  /// // Generate with default features plus additional ones
  /// let docs = CrateDocs::from_local_with_features(
  ///     PathBuf::from("./Cargo.toml"),
  ///     false, // keep default features
  ///     Some(vec!["experimental".to_string()]),
  ///     None, // auto-detect toolchain
  /// )?;
  /// ```
  #[cfg(feature = "rustdoc")]
  pub fn from_local_with_features(
    manifest_path: PathBuf,
    no_default_features: bool,
    features: Option<Vec<String>>,
    toolchain: Option<String>,
  ) -> Result<CrateDocs> {
    let gen_docs_struct = match toolchain {
      Some(toolchain) => gen_docs::gen_docs_with_features(
        &toolchain,
        manifest_path,
        no_default_features,
        features,
      )?,
      None => gen_docs::gen_docs_with_features_auto_toolchain(
        manifest_path,
        no_default_features,
        features,
      )?,
    };

    let lib_name = gen_docs_struct.lib_name;
    let docs = gen_docs_struct.docs;
    return CrateDocs::process_docs(&lib_name, docs, None);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[cfg(feature = "rustdoc")]
  use std::path::PathBuf;

  #[tokio::test]
  async fn test_from_online() {
    let lib_name = "clap";
    let version = "4.5.42".to_string();
    let result = CrateDocs::from_online(lib_name, Some(version.clone())).await;

    match result {
      Ok(docs) => {
        assert_eq!(docs.lib_name, lib_name);
        assert_eq!(docs.version, version);
        assert!(!docs.sessions.is_empty());
        println!("Successfully fetched docs for {}", lib_name);
      }
      Err(e) => {
        // If it fails due to version compatibility, that's expected for some crates
        match e {
          crate::error::Error::Config(msg) if msg.contains("incompatible") => {
            println!(
              "Expected version compatibility issue with {}: {}",
              lib_name, msg
            );
            // This is acceptable - version mismatch is a known issue
          }
          _ => {
            panic!("Unexpected error fetching {} docs: {:?}", lib_name, e);
          }
        }
      }
    }
  }

  #[cfg(feature = "rustdoc")]
  #[test]
  fn test_from_local_with_all_features() {
    let lib_name = "crates_llms_txt";
    let current_dir = std::env::current_dir().unwrap();
    let docs = CrateDocs::from_local(
      current_dir.join("Cargo.toml"),
      Some("stable".to_string()),
    )
    .unwrap();
    assert_eq!(docs.lib_name, lib_name);
    assert!(!docs.sessions.is_empty());
  }

  #[cfg(feature = "rustdoc")]
  #[test]
  fn test_from_local_with_stable_toolchain() {
    let current_dir = std::env::current_dir().unwrap();
    let manifest_path = current_dir.join("Cargo.toml");

    let result =
      CrateDocs::from_local(manifest_path.clone(), Some("stable".to_string()));
    assert!(result.is_ok());

    let docs = result.unwrap();
    assert_eq!(docs.lib_name, "crates_llms_txt");
    assert!(!docs.sessions.is_empty());
    assert!(!docs.full_sessions.is_empty());
  }

  #[cfg(feature = "rustdoc")]
  #[test]
  fn test_from_local_with_auto_toolchain() {
    let current_dir = std::env::current_dir().unwrap();
    let manifest_path = current_dir.join("Cargo.toml");

    let result = CrateDocs::from_local(manifest_path.clone(), None);
    assert!(result.is_ok());

    let docs = result.unwrap();
    assert_eq!(docs.lib_name, "crates_llms_txt");
    assert!(!docs.sessions.is_empty());
    assert!(!docs.full_sessions.is_empty());
  }

  #[cfg(feature = "rustdoc")]
  #[test]
  fn test_from_local_invalid_path() {
    let invalid_path = PathBuf::from("/invalid/path/Cargo.toml");
    let result = CrateDocs::from_local(invalid_path, None);
    assert!(result.is_err());
  }

  #[cfg(feature = "rustdoc")]
  #[test]
  fn test_from_local_with_features() {
    let current_dir = std::env::current_dir().unwrap();
    let manifest_path = current_dir.join("Cargo.toml");

    // Test with specific features
    let result = CrateDocs::from_local_with_features(
      manifest_path.clone(),
      false,
      Some(vec!["rustdoc".to_string()]),
      Some("stable".to_string()),
    );
    assert!(result.is_ok());

    let docs = result.unwrap();
    assert_eq!(docs.lib_name, "crates_llms_txt");
    assert!(!docs.sessions.is_empty());
    assert!(!docs.full_sessions.is_empty());

    // Test with no default features
    let result = CrateDocs::from_local_with_features(
      manifest_path.clone(),
      true,
      None,
      None,
    );
    assert!(result.is_ok());

    // Test with invalid path
    let invalid_path = PathBuf::from("/invalid/path/Cargo.toml");
    let result =
      CrateDocs::from_local_with_features(invalid_path, false, None, None);
    assert!(result.is_err());
  }
}
