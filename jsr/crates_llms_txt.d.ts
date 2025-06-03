// @generated file from wasmbuild -- do not edit
// deno-lint-ignore-file
// deno-fmt-ignore-file

/**
 * Get the LLM config for a given crate and version.
 *
 * # Arguments
 *
 * * `lib_name` - The name of the crate.
 * * `version` - The version of the crate. If None, the latest version will be used.
 *
 * # Returns
 *
 * * `Result<LLMsStandardConfig, Box<dyn std::error::Error>>` - The LLM config for the crate.
 *
 * # Examples
 *
 * ```
 * let config = LLMsStandardConfig::get_llms_config("clap", Some("4.5.39")).await.unwrap();
 * ```
 */
export function get_llms_config_online(
  lib_name: string,
  version?: string | null,
): Promise<any>;
/**
 * Get the LLM config by generating rustdoc with all features enabled.
 *
 * This function takes a toolchain and a manifest path, generates rustdoc JSON
 * for the specified crate with all features enabled, and returns the LLM config
 * as a JavaScript object.
 *
 * # Arguments
 *
 * * `toolchain` - The Rust toolchain to use (e.g., "stable", "nightly").
 * * `manifest_path` - The path to the Cargo.toml file of the crate.
 *
 * # Returns
 *
 * * `Result<JsValue, JsValue>` - A JavaScript object representing the LLM config
 *   on success, or `JsValue::undefined()` on failure.
 *
 * # Examples
 *
 * ```no_run
 * let config = get_llms_config_by_rustdoc_all_features("stable", "Cargo.toml").await;
 * ```
 */
export function get_llms_config_by_rustdoc_all_features(
  toolchain: string,
  manifest_path: string,
): any;
/**
 * Get the LLM config by generating rustdoc with specified features enabled.
 *
 * This function takes a toolchain, manifest path, and a list of features,
 * generates rustdoc JSON for the specified crate with the specified features
 * enabled, and returns the LLM config as a JavaScript object.
 *
 * # Arguments
 *
 * * `toolchain` - The Rust toolchain to use (e.g., "stable", "nightly").
 * * `manifest_path` - The path to the Cargo.toml file of the crate.
 * * `no_default_features` - Whether to include the default features.
 * * `features` - The features to include.
 *
 * # Returns
 *
 * * `Result<JsValue, JsValue>` - A JavaScript object representing the LLM config
 *   on success, or `JsValue::undefined()` on failure.
 *
 * # Examples
 *
 * ```no_run
 * let config = get_llms_config_by_rustdoc_features("stable", "Cargo.toml", true, vec!["async".to_string()]).await;
 * ```
 */
export function get_llms_config_by_rustdoc_features(
  toolchain: string,
  manifest_path: string,
  no_default_features: boolean,
  features?: string[] | null,
): Promise<any>;
