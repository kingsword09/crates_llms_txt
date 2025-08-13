use std::path::PathBuf;

use crates_llms_txt::CrateDocs;
use napi::Either;
use napi_derive::napi;

/// Represents a documentation session item with metadata
/// @interface SessionItem
/// @property title - The title of the documentation section
/// @property description - A brief description of the section content
/// @property link - The URL link to the full documentation
#[napi(object)]
pub struct SessionItem {
  pub title: String,
  pub description: String,
  pub link: String,
}

/// Represents a full documentation session with complete content
/// @interface FullSessionItem
/// @property content - The complete documentation content as a string
/// @property link - The URL link to the original documentation source
#[napi(object)]
pub struct FullSessionItem {
  pub content: String,
  pub link: String,
}

/// Main configuration object containing all documentation data for LLM consumption
/// @interface LLMsConfig
/// @property libName - The name of the Rust crate
/// @property version - The version string of the crate
/// @property sessions - Array of session items with metadata for quick reference
/// @property fullSessions - Array of full session items with complete documentation content
#[napi(object)]
pub struct LLMsConfig {
  pub lib_name: String,
  pub version: String,
  pub sessions: Vec<SessionItem>,
  pub full_sessions: Vec<FullSessionItem>,
}

/// Configuration for fetching documentation by crate name
/// @interface LLMsConfigByCrate
/// @property libName - The name of the crate to fetch documentation for
/// @property version - Optional version string. If not provided, latest version will be used
#[napi(object)]
pub struct LLMsConfigByCrate {
  pub lib_name: String,
  pub version: Option<String>,
}

/// Configuration for fetching documentation from a specific URL
/// @interface LLMsConfigByUrl
/// @property url - The direct URL to the crate's JSON documentation (e.g., "https://docs.rs/crate/clap/latest/json")
#[napi(object)]
pub struct LLMsConfigByUrl {
  pub url: String,
}

/// Configuration for generating local documentation with all features enabled
/// @interface LLMsConfigRustdocByAllFeatures
/// @property toolchain - Optional Rust toolchain to use (e.g., "stable", "nightly"). Defaults to system default if not provided
/// @property manifestPath - Absolute or relative path to the Cargo.toml file of the target crate
#[napi(object)]
pub struct LLMsConfigRustdocByAllFeatures {
  pub toolchain: Option<String>,
  pub manifest_path: String,
}

/// Configuration for generating local documentation with specific features
/// @interface LLMsConfigRustdocByFeatures
/// @property toolchain - Optional Rust toolchain to use (e.g., "stable", "nightly"). Defaults to system default if not provided
/// @property manifestPath - Absolute or relative path to the Cargo.toml file of the target crate
/// @property noDefaultFeatures - If true, disables the default features of the crate
/// @property features - Optional array of feature names to enable (e.g., ["async", "serde"])
#[napi(object)]
pub struct LLMsConfigRustdocByFeatures {
  pub toolchain: Option<String>,
  pub manifest_path: String,
  pub no_default_features: bool,
  pub features: Option<Vec<String>>,
}

/// Internal utility function to convert CrateDocs to LLMsConfig format for NAPI compatibility
/// This function transforms the internal Rust documentation structure into the TypeScript-friendly format
/// @param docs - The internal CrateDocs structure from the Rust library
/// @returns LLMsConfig - The converted configuration ready for TypeScript consumption
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
  let full_sessions: Vec<_> = docs
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
/// Fetches Rust crate documentation from docs.rs by crate name and version
/// This function retrieves pre-built documentation from the official Rust documentation registry.
/// It's the most convenient way to get documentation for published crates.
///
/// @param libName - The name of the crate as it appears on crates.io (e.g., "clap", "serde", "tokio")
/// @param version - Optional version string. If not provided or null, the latest version will be fetched
/// @returns Promise<LLMsConfig | null> - A promise that resolves to the documentation configuration, or null if the crate is not found or an error occurs
/// @throws Will return null instead of throwing errors for better TypeScript error handling
/// @example
/// ```typescript
/// import { fromCrateName } from 'crates-llms-txt-napi';
///
/// // Fetch latest version
/// const latestConfig = await fromCrateName("clap");
///
/// // Fetch specific version
/// const specificConfig = await fromCrateName("clap", "4.5.39");
///
/// if (specificConfig) {
///   console.log(`Fetched docs for ${specificConfig.libName} v${specificConfig.version}`);
///   console.log(`Found ${specificConfig.sessions.length} documentation sections`);
/// }
/// ```
pub async fn from_crate_name(
  lib_name: String,
  version: Option<String>,
) -> Option<LLMsConfig> {
  match CrateDocs::from_online(&lib_name, version).await {
    Ok(docs) => Some(convert_crate_docs_to_llms_config(docs)),
    Err(_) => None,
  }
}

#[napi]
/// Fetches Rust crate documentation from a direct URL to the JSON documentation
/// This function allows you to fetch documentation from any valid docs.rs JSON endpoint,
/// giving you more control over the exact documentation source.
///
/// @param url - The direct URL to the crate's JSON documentation index (must be a valid docs.rs JSON endpoint)
/// @returns Promise<LLMsConfig | null> - A promise that resolves to the documentation configuration, or null if the URL is unreachable or invalid
/// @throws Will return null instead of throwing errors for better TypeScript error handling
/// @example
/// ```typescript
/// import { fromUrl } from 'crates-llms-txt-napi';
///
/// // Fetch from specific docs.rs JSON endpoint
/// const config = await fromUrl("https://docs.rs/crate/clap/4.5.39/json");
///
/// // Or use the latest endpoint
/// const latestConfig = await fromUrl("https://docs.rs/crate/clap/latest/json");
///
/// if (config) {
///   console.log(`Fetched docs from URL for: ${config.libName}`);
///   console.log(`Documentation contains ${config.fullSessions.length} full sections`);
/// }
/// ```
pub async fn from_url(url: String) -> Option<LLMsConfig> {
  match CrateDocs::from_url(&url).await {
    Ok(docs) => Some(convert_crate_docs_to_llms_config(docs)),
    Err(_) => None,
  }
}

#[napi]
/// Unified function to fetch crate documentation from online sources using either crate name or direct URL
/// This is a convenience function that accepts either a crate configuration or URL configuration,
/// automatically routing to the appropriate fetching method.
///
/// @param params - A union type that accepts either:
///   - LLMsConfigByCrate: { libName: string, version?: string }
///   - LLMsConfigByUrl: { url: string }
/// @returns Promise<LLMsConfig | null> - A promise that resolves to the documentation configuration, or null if failed
/// @throws Will return null instead of throwing errors for better TypeScript error handling
/// @example
/// ```typescript
/// import { fromOnline } from 'crates-llms-txt-napi';
///
/// // Fetch by crate name and version
/// const configByCrate = await fromOnline({
///   libName: "clap",
///   version: "4.5.39"
/// });
///
/// // Fetch by direct URL
/// const configByUrl = await fromOnline({
///   url: "https://docs.rs/crate/clap/latest/json"
/// });
///
/// // Both return the same LLMsConfig structure
/// if (configByCrate) {
///   console.log(`Crate method: ${configByCrate.libName} v${configByCrate.version}`);
/// }
/// ```
pub async fn from_online(
  params: Either<LLMsConfigByCrate, LLMsConfigByUrl>,
) -> Option<LLMsConfig> {
  match params {
    Either::A(params) => from_crate_name(params.lib_name, params.version).await,
    Either::B(params) => from_url(params.url).await,
  }
}

#[napi]
/// Generates documentation for a local Rust crate by running `cargo doc --all-features`
/// This function requires a local Rust toolchain and will compile the crate with all features enabled
/// to generate comprehensive documentation. Useful for local development and unpublished crates.
///
/// @param manifestPath - Absolute or relative path to the Cargo.toml file of the target crate
/// @param toolchain - Optional Rust toolchain to use (e.g., "stable", "nightly", "1.70.0"). If not provided, uses system default
/// @returns LLMsConfig | null - The generated documentation configuration, or null if compilation fails or toolchain is unavailable
/// @throws Will return null instead of throwing errors for better TypeScript error handling
/// @example
/// ```typescript
/// import { fromLocal } from 'crates-llms-txt-napi';
///
/// // Use default toolchain
/// const config = fromLocal("./Cargo.toml");
///
/// // Use specific toolchain
/// const stableConfig = fromLocal("./my-crate/Cargo.toml", "stable");
/// const nightlyConfig = fromLocal("./experimental-crate/Cargo.toml", "nightly");
///
/// if (config) {
///   console.log(`Generated local docs for ${config.libName} v${config.version}`);
///   console.log(`Documentation includes ${config.sessions.length} sections`);
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
/// Generates documentation for a local Rust crate with fine-grained feature control
/// This function allows you to specify exactly which features to enable or disable when generating
/// documentation, giving you precise control over what gets documented. Runs `cargo doc` with custom feature flags.
///
/// @param manifestPath - Absolute or relative path to the Cargo.toml file of the target crate
/// @param noDefaultFeatures - If true, disables all default features of the crate (equivalent to --no-default-features)
/// @param features - Optional array of specific feature names to enable (e.g., ["async", "serde", "tokio"])
/// @param toolchain - Optional Rust toolchain to use (e.g., "stable", "nightly", "1.70.0"). If not provided, uses system default
/// @returns LLMsConfig | null - The generated documentation configuration, or null if compilation fails or features are invalid
/// @throws Will return null instead of throwing errors for better TypeScript error handling
/// @example
/// ```typescript
/// import { fromLocalWithFeatures } from 'crates-llms-txt-napi';
///
/// // Enable specific features while keeping defaults
/// const withAsync = fromLocalWithFeatures("./Cargo.toml", false, ["async", "tokio"], "stable");
///
/// // Disable defaults and enable only specific features
/// const minimalConfig = fromLocalWithFeatures("./Cargo.toml", true, ["core"], "stable");
///
/// // Enable features without specifying toolchain
/// const defaultToolchain = fromLocalWithFeatures("./Cargo.toml", false, ["serde"]);
///
/// if (withAsync) {
///   console.log(`Generated docs for ${withAsync.libName} with async features`);
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
/// Unified function to generate local crate documentation using rustdoc with flexible feature configuration
/// This is a convenience function that accepts either an "all features" configuration or a "specific features"
/// configuration, automatically routing to the appropriate documentation generation method.
///
/// @param params - A union type that accepts either:
///   - LLMsConfigRustdocByAllFeatures: { toolchain?: string, manifestPath: string }
///   - LLMsConfigRustdocByFeatures: { toolchain?: string, manifestPath: string, noDefaultFeatures: boolean, features?: string[] }
/// @returns LLMsConfig | null - The generated documentation configuration, or null if compilation fails
/// @throws Will return null instead of throwing errors for better TypeScript error handling
/// @example
/// ```typescript
/// import { fromLocalByRustdoc } from 'crates-llms-txt-napi';
///
/// // Generate docs with all features enabled
/// const allFeaturesConfig = fromLocalByRustdoc({
///   toolchain: "stable",
///   manifestPath: "./Cargo.toml"
/// });
///
/// // Generate docs with specific feature control
/// const customFeaturesConfig = fromLocalByRustdoc({
///   toolchain: "nightly",
///   manifestPath: "./advanced-crate/Cargo.toml",
///   noDefaultFeatures: true,
///   features: ["experimental", "async"]
/// });
///
/// // The function automatically detects which configuration type you're using
/// if (allFeaturesConfig) {
///   console.log(`All features docs: ${allFeaturesConfig.libName}`);
/// }
/// if (customFeaturesConfig) {
///   console.log(`Custom features docs: ${customFeaturesConfig.libName}`);
/// }
/// ```
pub fn from_local_by_rustdoc(
  params: Either<LLMsConfigRustdocByAllFeatures, LLMsConfigRustdocByFeatures>,
) -> Option<LLMsConfig> {
  match params {
    Either::A(params) => from_local(params.manifest_path, params.toolchain),
    Either::B(params) => from_local_with_features(
      params.manifest_path,
      params.no_default_features,
      params.features,
      params.toolchain,
    ),
  }
}
