use std::path::PathBuf;

use crates_llms_txt::LLMsStandardConfig;
use napi::Either;
use napi_derive::napi;

#[napi(object)]
pub struct LLMsConfig {
  pub lib_name: String,
  pub version: String,
  pub sessions: String,
  pub full_sessions: String,
}

#[napi(object)]
pub struct LLMsConfigByCrate {
  pub lib_name: String,
  pub version: Option<String>,
}

#[napi(object)]
pub struct LLMsConfigByUrl {
  pub url: String,
}

#[napi(object)]
pub struct LLMsConfigRustdocByAllFeatures {
  pub toolchain: Option<String>,
  pub manifest_path: String,
}

#[napi(object)]
pub struct LLMsConfigRustdocByFeatures {
  pub toolchain: Option<String>,
  pub manifest_path: String,
  pub no_default_features: bool,
  pub features: Option<Vec<String>>,
}

#[napi]
/// Get llms config online by crates name
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
/// let config = get_llms_config_online("clap".to_string(), Some("4.5.39".to_string())).await?;
/// ```
///
pub async fn get_llms_config_online_by_crates_name(
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
/// Get llms config by online by url
///
/// # Arguments
///
/// * `url` - The url of the crate
///
/// # Returns
///
/// * `Option<LLMsConfig>` - The generated documentation configuration
///
/// # Examples
///
/// ```no_run
/// let config = get_llms_config_online_by_url("https://docs.rs/crate/clap/latest/json".to_string()).await?;
/// ```
///
pub async fn get_llms_config_online_by_url(url: String) -> Option<LLMsConfig> {
  match LLMsStandardConfig::get_llms_config_online_by_url(&url).await {
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
/// Get llms config online by either crate name or URL
///
/// # Arguments
///
/// * `params` - Either a LLMsConfigByCrate or LLMsConfigByUrl object containing:
///   - For crate: lib_name and optional version
///   - For URL: url string
///
/// # Returns
///
/// * `Option<LLMsConfig>` - The generated documentation configuration
///
/// # Examples
///
/// ```no_run
/// // By crate name
/// let config = get_llms_config_online(Either::A(LLMsConfigByCrate {
///   lib_name: "clap".to_string(),
///   version: Some("4.5.39".to_string())
/// })).await?;
///
/// // By URL
/// let config = get_llms_config_online(Either::B(LLMsConfigByUrl {
///   url: "https://docs.rs/crate/clap/latest/json".to_string()
/// })).await?;
/// ```
pub async fn get_llms_config_online(
  params: Either<LLMsConfigByCrate, LLMsConfigByUrl>,
) -> Option<LLMsConfig> {
  match params {
    Either::A(params) => {
      get_llms_config_online_by_crates_name(params.lib_name, params.version)
        .await
    }
    Either::B(params) => get_llms_config_online_by_url(params.url).await,
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
/// let config = get_llms_config_by_rustdoc_all_features("stable".to_string(), "path/to/Cargo.toml".to_string());
/// ```
pub fn get_llms_config_by_rustdoc_all_features(
  manifest_path: String,
  toolchain: Option<String>,
) -> Option<LLMsConfig> {
  let manifest_path = PathBuf::from(manifest_path);
  match LLMsStandardConfig::get_llms_config_offline_with_all_features(
    manifest_path,
    toolchain,
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
/// let config = get_llms_config_by_rustdoc_features("stable".to_string(), "path/to/Cargo.toml".to_string(), false, Some(vec!["async".to_string()]));
/// ```
pub fn get_llms_config_by_rustdoc_features(
  manifest_path: String,
  no_default_features: bool,
  features: Option<Vec<String>>,
  toolchain: Option<String>,
) -> Option<LLMsConfig> {
  let manifest_path = PathBuf::from(manifest_path);
  match LLMsStandardConfig::get_llms_config_offline_with_features(
    manifest_path,
    no_default_features,
    features,
    toolchain,
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
/// Get llms config by rustdoc with either all features or specific features
///
/// # Arguments
///
/// * `params` - Either a LLMsConfigRustdocByAllFeatures or LLMsConfigRustdocByFeatures object containing:
///   - For all features: toolchain and manifest_path
///   - For specific features: toolchain, manifest_path, no_default_features flag, and optional features list
///
/// # Returns
///
/// * `Option<LLMsConfig>` - The generated documentation configuration
///
/// # Examples
///
/// ```no_run
/// // With all features
/// let config = get_llms_config_by_rustdoc(Either::A(LLMsConfigRustdocByAllFeatures {
///   toolchain: "stable".to_string(),
///   manifest_path: "path/to/Cargo.toml".to_string()
/// }));
///
/// // With specific features
/// let config = get_llms_config_by_rustdoc(Either::B(LLMsConfigRustdocByFeatures {
///   toolchain: "stable".to_string(),
///   manifest_path: "path/to/Cargo.toml".to_string(),
///   no_default_features: false,
///   features: Some(vec!["async".to_string()])
/// }));
/// ```
pub fn get_llms_config_by_rustdoc(
  params: Either<LLMsConfigRustdocByAllFeatures, LLMsConfigRustdocByFeatures>,
) -> Option<LLMsConfig> {
  match params {
    Either::A(params) => get_llms_config_by_rustdoc_all_features(
      params.manifest_path,
      params.toolchain,
    ),
    Either::B(params) => get_llms_config_by_rustdoc_features(
      params.manifest_path,
      params.no_default_features,
      params.features,
      params.toolchain,
    ),
  }
}
