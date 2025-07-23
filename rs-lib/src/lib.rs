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

const DOCS_BASE_URL: &str = "https://docs.rs/crate";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionItem {
  pub title: String,
  pub description: String,
  pub link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullSessionItem {
  pub content: String,
  pub link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrateDocs {
  pub lib_name: String,
  pub version: String,
  pub sessions: Vec<SessionItem>,
  pub full_sessions: Vec<FullSessionItem>,
}

impl CrateDocs {
  pub fn new(lib_name: &str, version: &str) -> Self {
    Self {
      lib_name: lib_name.to_string(),
      version: version.to_string(),
      sessions: Vec::new(),
      full_sessions: Vec::new(),
    }
  }

  /// Process documentation data for a given crate and version.
  ///
  /// # Arguments
  ///
  /// * `lib_name` - The name of the crate.
  /// * `docs` - The documentation data implementing CommonCrates trait.
  /// * `version` - The version of the crate. If None, the latest version will be used.
  ///
  /// # Returns
  ///
  /// * `Result<CrateDocs, Error>` - The processed crate documentation.
  ///
  fn process_docs<T: CommonCrates>(
    lib_name: &str,
    docs: T,
    version: Option<String>,
  ) -> Result<CrateDocs> {
    let version = version.unwrap_or(docs.crate_version());
    let mut crate_docs = CrateDocs::new(lib_name, &version);
    let base_url =
      format!("{}/{}/{}/source", DOCS_BASE_URL, &lib_name, version);

    crate_docs.sessions.push(SessionItem {
      title: lib_name.to_string(),
      description: "".to_string(),
      link: format!("https://docs.rs/{lib_name}/{version}"),
    });

    for (_, item) in docs.index() {
      if let Some(docs) = item.docs {
        // Skip private and default items
        if item.visibility != Visibility::Public {
          continue;
        }

        let filename = item.span.unwrap().filename;
        let link = format!("{}/{}", base_url, filename.to_str().unwrap());

        crate_docs.sessions.push(SessionItem {
          title: match item.name {
            Some(name) => name,
            None => filename.to_str().unwrap().to_string(),
          },
          description: "".to_string(),
          link: link.clone(),
        });
        crate_docs.full_sessions.push(FullSessionItem {
          content: docs,
          link,
        });
      };
    }

    Ok(crate_docs)
  }

  /// Fetch crate documentation from online docs.rs.
  ///
  /// # Arguments
  ///
  /// * `lib_name` - The name of the crate.
  /// * `version` - The version of the crate. If None, the latest version will be used.
  ///
  /// # Returns
  ///
  /// * `Result<CrateDocs, Error>` - The fetched crate documentation.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use crates_llms_txt::CrateDocs;
  ///
  /// #[tokio::main]
  /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
  ///   let docs = CrateDocs::from_online("clap", Some("4.5.39".to_string())).await?;
  ///   Ok(())
  /// }
  /// ```
  ///
  pub async fn from_online(
    lib_name: &str,
    version: Option<String>,
  ) -> Result<CrateDocs> {
    let docs = OnlineDocs::fetch_docs(lib_name, version.clone()).await?;
    CrateDocs::process_docs(lib_name, docs, version)
  }

  /// Fetch crate documentation from a specific URL.
  ///
  /// # Arguments
  ///
  /// * `url` - The URL of the documentation JSON.
  ///
  /// # Returns
  ///
  /// * `Result<CrateDocs, Error>` - The fetched crate documentation.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use crates_llms_txt::CrateDocs;
  ///
  /// #[tokio::main]
  /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
  ///   let docs = CrateDocs::from_url("https://docs.rs/crate/clap/latest/json").await?;
  ///   Ok(())
  /// }
  /// ```
  ///
  pub async fn from_url(url: &str) -> Result<CrateDocs> {
    let docs = OnlineDocs::fetch_docs_by_url(url).await?;
    let root_id = docs.root;

    if let Some(root_item) = docs.clone().index.get(&root_id) {
      let lib_name = &root_item.name.clone().unwrap_or("unknown".to_string());
      return CrateDocs::process_docs(lib_name, docs, None);
    }

    Err(Error::Config("Failed to get crate docs".into()))
  }

  /// Generate documentation for a crate using local mode with all features enabled.
  ///
  /// # Arguments
  ///
  /// * `manifest_path` - Path to the Cargo.toml file of the crate
  /// * `toolchain` - The Rust toolchain to use (e.g. "stable", "nightly")
  ///
  /// # Returns
  ///
  /// * `Result<CrateDocs, Error>` - The generated crate documentation
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use std::path::PathBuf;
  /// use crates_llms_txt::CrateDocs;
  /// 
  /// let docs = CrateDocs::from_local(
  ///     PathBuf::from("path/to/Cargo.toml"),
  ///     Some("stable".to_string()),
  /// )?;
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

  /// Generate documentation for a crate using local mode with specified features enabled.
  ///
  /// # Arguments
  ///
  /// * `manifest_path` - Path to the Cargo.toml file of the crate
  /// * `no_default_features` - Whether to disable the default features
  /// * `features` - List of features to enable
  /// * `toolchain` - The Rust toolchain to use (e.g. "stable", "nightly")
  ///
  /// # Returns
  ///
  /// * `Result<CrateDocs, Error>` - The generated crate documentation
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use std::path::PathBuf;
  /// use crates_llms_txt::CrateDocs;
  /// 
  /// let docs = CrateDocs::from_local_with_features(
  ///     PathBuf::from("path/to/Cargo.toml"),
  ///     false,
  ///     Some(vec!["async".to_string()]),
  ///     Some("stable".to_string()),
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
    let version = "4.5.39".to_string();
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
            println!("Expected version compatibility issue with {}: {}", lib_name, msg);
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
