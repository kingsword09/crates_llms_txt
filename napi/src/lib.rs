#![deny(clippy::all)]

use std::path::PathBuf;

use crates_llms_txt::LLMsStandardConfig;
use napi_derive::napi;

#[napi(object)]
pub struct LLMsConfig {
  pub lib_name: String,
  pub version: String,
  pub sessions: String,
  pub full_sessions: String,
}

#[napi]
/// Get llms config by online
///
/// # Arguments
///
/// * `lib_name` - The name of the crate
/// * `version` - The version of the crate (optional)
///
/// # Returns
///
/// * `Option<LLMsConfig>` - The generated documentation configuration
///
/// # Examples
///
/// ```no_run
/// let config = get_llms_config_online("clap", Some("4.5.39"));
/// ```
pub async fn get_llms_config_online(
  lib_name: String,
  version: Option<String>,
) -> Option<LLMsConfig> {
  match LLMsStandardConfig::get_llms_config_online(&lib_name, version).await {
    Ok(config) => Some(LLMsConfig {
      lib_name: config.lib_name,
      version: config.version,
      sessions: config.sessions,
      full_sessions: config.full_sessions,
    }),
    Err(_) => None,
  }
}

#[napi]
/// Get llms config by rustdoc all features
///
/// # Arguments
///
/// * `toolchain` - The Rust toolchain to use (e.g. "stable", "nightly")
/// * `manifest_path` - Path to the Cargo.toml file of the crate
///
/// # Returns
///
/// * `Option<LLMsConfig>` - The generated documentation configuration
///
/// # Examples
///
/// ```no_run
/// let config = get_llms_config_by_rustdoc_all_features("stable", "path/to/Cargo.toml");
/// ```
pub fn get_llms_config_by_rustdoc_all_features(
  toolchain: String,
  manifest_path: String,
) -> Option<LLMsConfig> {
  let manifest_path = PathBuf::from(manifest_path);
  match LLMsStandardConfig::get_llms_config_offline_with_all_features(
    &toolchain,
    manifest_path,
  ) {
    Ok(config) => Some(LLMsConfig {
      lib_name: config.lib_name,
      version: config.version,
      sessions: config.sessions,
      full_sessions: config.full_sessions,
    }),
    Err(_) => None,
  }
}

#[napi]
/// Get llms config by rustdoc features
///
/// # Arguments
///
/// * `toolchain` - The Rust toolchain to use (e.g. "stable", "nightly")
/// * `manifest_path` - Path to the Cargo.toml file of the crate
/// * `no_default_features` - Whether to include the default features
/// * `features` - The features to include
///
/// # Returns
///
/// * `Option<LLMsConfig>` - The generated documentation configuration
///
/// # Examples
///
/// ```no_run
/// let config = get_llms_config_by_rustdoc_features("stable", "path/to/Cargo.toml", false, Some(vec!["async".to_string()]));
/// ```
pub fn get_llms_config_by_rustdoc_features(
  toolchain: String,
  manifest_path: String,
  no_default_features: bool,
  features: Option<Vec<String>>,
) -> Option<LLMsConfig> {
  let manifest_path = PathBuf::from(manifest_path);
  match LLMsStandardConfig::get_llms_config_offline_with_features(
    &toolchain,
    manifest_path,
    no_default_features,
    features,
  ) {
    Ok(config) => Some(LLMsConfig {
      lib_name: config.lib_name,
      version: config.version,
      sessions: config.sessions,
      full_sessions: config.full_sessions,
    }),
    Err(_) => None,
  }
}
