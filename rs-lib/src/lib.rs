use fetch_docs::OnlineDocs;
use rustdoc_types::Visibility;
use serde::{Deserialize, Serialize};
use std::error::Error;
#[cfg(feature = "rustdoc")]
use std::path::PathBuf;

use temp_trait::CommonCrates;

pub mod fetch_docs;
#[cfg(feature = "rustdoc")]
mod gen_docs;
pub mod temp_trait;

const DOCS_BASE_URL: &str = "https://docs.rs/crate";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMsStandardStringConfig {
  pub lib_name: String,
  pub version: String,
  pub sessions: String,
  pub full_sessions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionItem {
  title: String,
  description: String,
  link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullSessionItem {
  content: String,
  link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMsStandardConfig {
  pub lib_name: String,
  pub version: String,
  pub sessions: Vec<SessionItem>,
  pub full_sessions: Vec<FullSessionItem>,
}

impl LLMsStandardConfig {
  pub fn new(lib_name: &str, version: &str) -> Self {
    Self {
      lib_name: lib_name.to_string(),
      version: version.to_string(),
      sessions: Vec::new(),
      full_sessions: Vec::new(),
    }
  }

  /// Get the LLM config for a given crate and version.
  ///
  /// # Arguments
  ///
  /// * `lib_name` - The name of the crate.
  /// * `version` - The version of the crate. If None, the latest version will be used.
  ///
  /// # Returns
  ///
  /// * `Result<LLMsStandardConfig, Box<dyn Error>>` - The LLM config for the crate.
  ///
  fn process_docs<T: CommonCrates>(
    lib_name: &str,
    docs: T,
    version: Option<String>,
  ) -> Result<LLMsStandardStringConfig, Box<dyn Error>> {
    let version = version.unwrap_or(docs.crate_version());
    let mut config = LLMsStandardConfig::new(lib_name, &version);
    let base_url =
      format!("{}/{}/{}/source", DOCS_BASE_URL, &lib_name, version);

    config.sessions.push(SessionItem {
      title: lib_name.to_string(),
      description: "".to_string(),
      link: format!("https://docs.rs/{}/{}", lib_name, version),
    });

    for (_, item) in docs.index() {
      if let Some(docs) = item.docs {
        // Skip private and default items
        if item.visibility != Visibility::Public {
          continue;
        }

        let filename = item.span.unwrap().filename;
        let link = format!("{}/{}", base_url, filename.to_str().unwrap());

        config.sessions.push(SessionItem {
          title: match item.name {
            Some(name) => name,
            None => filename.to_str().unwrap().to_string(),
          },
          description: "".to_string(),
          link: link.clone(),
        });
        config.full_sessions.push(FullSessionItem {
          content: docs,
          link,
        });
      };
    }

    Ok(LLMsStandardStringConfig {
      lib_name: config.lib_name,
      version: config.version,
      sessions: serde_json::to_string(&config.sessions)
        .unwrap_or("".to_string()),
      full_sessions: serde_json::to_string(&config.full_sessions)
        .unwrap_or("".to_string()),
    })
  }

  /// Get the LLM config for a given crate and version.
  ///
  /// # Arguments
  ///
  /// * `lib_name` - The name of the crate.
  /// * `version` - The version of the crate. If None, the latest version will be used.
  ///
  /// # Returns
  ///
  /// * `Result<LLMsStandardConfig, Box<dyn std::error::Error>>` - The LLM config for the crate.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use crates_llms_txt::LLMsStandardConfig;
  ///
  /// #[tokio::main]
  /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
  ///   let config = LLMsStandardConfig::get_llms_config_online("clap", Some("4.5.39".to_string())).await?;
  ///   Ok(())
  /// }
  /// ```
  ///
  pub async fn get_llms_config_online(
    lib_name: &str,
    version: Option<String>,
  ) -> Result<LLMsStandardStringConfig, Box<dyn std::error::Error>> {
    if let Ok(docs) = OnlineDocs::fetch_docs(lib_name, version.clone()).await {
      return LLMsStandardConfig::process_docs(lib_name, docs, version);
    }

    Err("Failed to get llms config".into())
  }

  /// Get the LLM config for a given url.
  ///
  /// # Arguments
  ///
  /// * `url` - The url of the json.
  ///
  /// # Returns
  ///
  /// * `Result<LLMsStandardConfig, Box<dyn Error>>` - The LLM config for the crate.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use crates_llms_txt::LLMsStandardConfig;
  ///
  /// #[tokio::main]
  /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
  ///   let config = LLMsStandardConfig::get_llms_config_online_by_url("https://docs.rs/crate/clap/latest/json").await?;
  ///   Ok(())
  /// }
  /// ```
  ///
  pub async fn get_llms_config_online_by_url(
    url: &str,
  ) -> Result<LLMsStandardStringConfig, Box<dyn Error>> {
    if let Ok(docs) = OnlineDocs::fetch_docs_by_url(url).await {
      let root_id = docs.root;
      if let Some(root_item) = docs.clone().index.get(&root_id) {
        let lib_name = &root_item.name.clone().unwrap_or("unknown".to_string());
        return LLMsStandardConfig::process_docs(lib_name, docs, None);
      }
    }

    Err("Failed to get llms config".into())
  }

  /// Generate documentation for a crate using offline mode with all features enabled.
  ///
  /// # Arguments
  ///
  /// * `toolchain` - The Rust toolchain to use (e.g. "stable", "nightly")
  /// * `manifest_path` - Path to the Cargo.toml file of the crate
  ///
  /// # Returns
  ///
  /// * `Result<LLMsStandardStringConfig, Box<dyn Error>>` - The generated documentation config
  ///
  /// # Examples
  ///
  /// ```no_run
  /// let config = LLMsStandardConfig::get_llms_config_offline_with_all_features(
  ///     "stable",
  ///     PathBuf::from("path/to/Cargo.toml")
  /// ).await?;
  /// ```
  #[cfg(feature = "rustdoc")]
  pub fn get_llms_config_offline_with_all_features(
    toolchain: &str,
    manifest_path: PathBuf,
  ) -> Result<LLMsStandardStringConfig, Box<dyn Error>> {
    if let Ok(gen_docs_struct) =
      gen_docs::gen_docs_with_all_features(toolchain, manifest_path)
    {
      let lib_name = gen_docs_struct.lib_name;
      let docs = gen_docs_struct.docs;
      return LLMsStandardConfig::process_docs(&lib_name, docs, None);
    }

    Err("Failed to get llms config".into())
  }

  /// Generate documentation for a crate using offline mode with specified features enabled.
  ///
  /// # Arguments
  ///
  /// * `toolchain` - The Rust toolchain to use (e.g. "stable", "nightly")
  /// * `manifest_path` - Path to the Cargo.toml file of the crate
  /// * `no_default_features` - Whether to disable the default features
  /// * `features` - List of features to enable
  ///
  /// # Returns
  ///
  /// * `Result<LLMsStandardStringConfig, Box<dyn Error>>` - The generated documentation config
  ///
  /// # Examples
  ///
  /// ```no_run
  /// let config = LLMsStandardConfig::get_llms_config_offline_with_features(
  ///     "stable",
  ///     PathBuf::from("path/to/Cargo.toml"),
  ///     false,
  ///     Some(vec!["async".to_string()])
  /// ).await?;
  /// ```
  #[cfg(feature = "rustdoc")]
  pub fn get_llms_config_offline_with_features(
    toolchain: &str,
    manifest_path: PathBuf,
    no_default_features: bool,
    features: Option<Vec<String>>,
  ) -> Result<LLMsStandardStringConfig, Box<dyn Error>> {
    if let Ok(gen_docs_struct) = gen_docs::gen_docs_with_features(
      toolchain,
      manifest_path,
      no_default_features,
      features,
    ) {
      let lib_name = gen_docs_struct.lib_name;
      let docs = gen_docs_struct.docs;
      return LLMsStandardConfig::process_docs(&lib_name, docs, None);
    }

    Err("Failed to get llms config".into())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_get_llms_config() {
    let lib_name = "clap";
    let version = "4.5.39".to_string();
    let config = LLMsStandardConfig::get_llms_config_online(
      lib_name,
      Some(version.clone()),
    )
    .await
    .unwrap();

    assert_eq!(config.lib_name, lib_name);
    assert_eq!(config.version, version);
  }

  #[cfg(feature = "rustdoc")]
  #[test]
  fn test_get_llms_config_offline_with_all_features() {
    let lib_name = "crates_llms_txt";
    let current_dir = std::env::current_dir().unwrap();
    let config = LLMsStandardConfig::get_llms_config_offline_with_all_features(
      "stable",
      current_dir.join("Cargo.toml"),
    )
    .unwrap();
    println!("{:?}", config);
    assert_eq!(config.lib_name, lib_name);
  }
}
