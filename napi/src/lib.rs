use std::path::PathBuf;

use crates_llms_txt::CrateDocs;
use napi::Either;
use napi_derive::napi;

#[napi(object)]
pub struct SessionItem {
  pub title: String,
  pub description: String,
  pub link: String,
}

#[napi(object)]
pub struct FullSessionItem {
  pub content: String,
  pub link: String,
}

#[napi(object)]
pub struct LLMsConfig {
  pub lib_name: String,
  pub version: String,
  pub sessions: Vec<SessionItem>,
  pub full_sessions: Vec<FullSessionItem>,
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

/// Convert CrateDocs to LLMsConfig format for NAPI compatibility
fn convert_crate_docs_to_llms_config(docs: CrateDocs) -> LLMsConfig {
  // Convert sessions
  let sessions = docs
    .sessions
    .into_iter()
    .map(|session| SessionItem {
      title: session.title,
      description: session.description,
      link: session.link,
    })
    .collect();

  // Convert full_sessions
  let full_sessions = docs
    .full_sessions
    .into_iter()
    .map(|full_session| FullSessionItem {
      content: full_session.content,
      link: full_session.link,
    })
    .collect();

  LLMsConfig {
    lib_name: docs.lib_name,
    version: docs.version,
    sessions,
    full_sessions,
  }
}

#[napi]
/// Fetch crate documentation from online docs.rs
///
/// @param libName - The name of the crate
/// @param version - The version of the crate (optional)
/// @returns Promise<LLMsConfig | null> - The generated documentation configuration or null if failed
///
/// @example
/// ```typescript
/// const config = await fromOnline("clap", "4.5.39");
/// if (config) {
///   console.log(`Fetched docs for ${config.libName} v${config.version}`);
/// }
/// ```
///
pub async fn from_online(
  lib_name: String,
  version: Option<String>,
) -> Option<LLMsConfig> {
  match CrateDocs::from_online(&lib_name, version).await {
    Ok(docs) => Some(convert_crate_docs_to_llms_config(docs)),
    Err(_) => None,
  }
}

#[napi]
/// Fetch crate documentation from a specific URL
///
/// @param url - The URL of the documentation JSON
/// @returns Promise<LLMsConfig | null> - The generated documentation configuration or null if failed
///
/// @example
/// ```typescript
/// const config = await fromUrl("https://docs.rs/crate/clap/latest/json");
/// if (config) {
///   console.log(`Fetched docs from URL: ${config.libName}`);
/// }
/// ```
///
pub async fn from_url(url: String) -> Option<LLMsConfig> {
  match CrateDocs::from_url(&url).await {
    Ok(docs) => Some(convert_crate_docs_to_llms_config(docs)),
    Err(_) => None,
  }
}

// Backward compatibility aliases
#[napi]
/// Get llms config online by crates name (alias for fromOnline)
///
/// @param libName - The name of the crate
/// @param version - The version of the crate (optional)
/// @returns Promise<LLMsConfig | null> - The generated documentation configuration or null if failed
/// @deprecated Use fromOnline instead
pub async fn get_llms_config_online_by_crates_name(
  lib_name: String,
  version: Option<String>,
) -> Option<LLMsConfig> {
  from_online(lib_name, version).await
}

#[napi]
/// Get llms config by online by url (alias for fromUrl)
///
/// @param url - The URL of the documentation JSON
/// @returns Promise<LLMsConfig | null> - The generated documentation configuration or null if failed
/// @deprecated Use fromUrl instead
pub async fn get_llms_config_online_by_url(url: String) -> Option<LLMsConfig> {
  from_url(url).await
}

#[napi]
/// Get llms config by rustdoc all features (alias for fromLocal)
///
/// @param manifestPath - Path to the Cargo.toml file of the crate
/// @param toolchain - The Rust toolchain to use (e.g. "stable", "nightly")
/// @returns LLMsConfig | null - The generated documentation configuration or null if failed
/// @deprecated Use fromLocal instead
pub fn get_llms_config_by_rustdoc_all_features(
  manifest_path: String,
  toolchain: Option<String>,
) -> Option<LLMsConfig> {
  from_local(manifest_path, toolchain)
}

#[napi]
/// Get llms config by rustdoc features (alias for fromLocalWithFeatures)
///
/// @param manifestPath - Path to the Cargo.toml file of the crate
/// @param noDefaultFeatures - Whether to disable the default features
/// @param features - List of features to enable
/// @param toolchain - The Rust toolchain to use (e.g. "stable", "nightly")
/// @returns LLMsConfig | null - The generated documentation configuration or null if failed
/// @deprecated Use fromLocalWithFeatures instead
pub fn get_llms_config_by_rustdoc_features(
  manifest_path: String,
  no_default_features: bool,
  features: Option<Vec<String>>,
  toolchain: Option<String>,
) -> Option<LLMsConfig> {
  from_local_with_features(
    manifest_path,
    no_default_features,
    features,
    toolchain,
  )
}

#[napi]
/// Get llms config online by either crate name or URL
///
/// @param params - Either a LLMsConfigByCrate or LLMsConfigByUrl object containing:
///   - For crate: libName and optional version
///   - For URL: url string
/// @returns Promise<LLMsConfig | null> - The generated documentation configuration or null if failed
///
/// @example
/// ```typescript
/// // By crate name
/// const configByCrate = await getLlmsConfigOnline({
///   libName: "clap",
///   version: "4.5.39"
/// });
///
/// // By URL
/// const configByUrl = await getLlmsConfigOnline({
///   url: "https://docs.rs/crate/clap/latest/json"
/// });
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
/// Generate documentation for a crate using local mode with all features enabled
///
/// @param manifestPath - Path to the Cargo.toml file of the crate
/// @param toolchain - The Rust toolchain to use (e.g. "stable", "nightly")
/// @returns LLMsConfig | null - The generated documentation configuration or null if failed
///
/// @example
/// ```typescript
/// const config = fromLocal("./Cargo.toml", "stable");
/// if (config) {
///   console.log(`Generated docs for ${config.libName}`);
/// }
/// ```
pub fn from_local(
  manifest_path: String,
  toolchain: Option<String>,
) -> Option<LLMsConfig> {
  let manifest_path = PathBuf::from(manifest_path);
  match CrateDocs::from_local(manifest_path, toolchain) {
    Ok(docs) => Some(convert_crate_docs_to_llms_config(docs)),
    Err(_) => None,
  }
}

#[napi]
/// Generate documentation for a crate using local mode with specified features enabled
///
/// @param manifestPath - Path to the Cargo.toml file of the crate
/// @param noDefaultFeatures - Whether to disable the default features
/// @param features - List of features to enable
/// @param toolchain - The Rust toolchain to use (e.g. "stable", "nightly")
/// @returns LLMsConfig | null - The generated documentation configuration or null if failed
///
/// @example
/// ```typescript
/// const config = fromLocalWithFeatures("./Cargo.toml", false, ["async"], "stable");
/// if (config) {
///   console.log(`Generated docs for ${config.libName} with features`);
/// }
/// ```
pub fn from_local_with_features(
  manifest_path: String,
  no_default_features: bool,
  features: Option<Vec<String>>,
  toolchain: Option<String>,
) -> Option<LLMsConfig> {
  let manifest_path = PathBuf::from(manifest_path);
  match CrateDocs::from_local_with_features(
    manifest_path,
    no_default_features,
    features,
    toolchain,
  ) {
    Ok(docs) => Some(convert_crate_docs_to_llms_config(docs)),
    Err(_) => None,
  }
}

#[napi]
/// Get llms config by rustdoc with either all features or specific features
///
/// @param params - Either a LLMsConfigRustdocByAllFeatures or LLMsConfigRustdocByFeatures object containing:
///   - For all features: toolchain and manifestPath
///   - For specific features: toolchain, manifestPath, noDefaultFeatures flag, and optional features list
/// @returns LLMsConfig | null - The generated documentation configuration or null if failed
///
/// @example
/// ```typescript
/// // With all features
/// const configAllFeatures = getLlmsConfigByRustdoc({
///   toolchain: "stable",
///   manifestPath: "./Cargo.toml"
/// });
///
/// // With specific features
/// const configWithFeatures = getLlmsConfigByRustdoc({
///   toolchain: "stable",
///   manifestPath: "./Cargo.toml",
///   noDefaultFeatures: false,
///   features: ["async"]
/// });
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
