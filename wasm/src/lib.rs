use std::path::PathBuf;

use crates_llms_txt::{LLMsStandardConfig, LLMsStandardStringConfig};
use js_sys::{Object, Reflect};
use wasm_bindgen::prelude::*;

///
/// Convert a LLMsStandardConfig to a JavaScript object.
///
/// # Arguments
///
/// * `config` - The LLMsStandardConfig to convert.
///
/// # Returns
///
/// * `Result<Object, JsError>` - The JavaScript object.
///
fn llms_config_to_js_object(
  config: LLMsStandardStringConfig,
) -> Result<Object, JsError> {
  let obj = Object::new();
  Reflect::set(
    &obj,
    &JsValue::from_str("lib_name"),
    &JsValue::from(config.lib_name),
  )
  .map_err(|e| JsError::new(&e.as_string().unwrap().as_str()))?;
  Reflect::set(
    &obj,
    &JsValue::from_str("version"),
    &JsValue::from(config.version),
  )
  .map_err(|e| JsError::new(&e.as_string().unwrap().as_str()))?;
  Reflect::set(
    &obj,
    &JsValue::from_str("sessions"),
    // &JsValue::from(serde_json::to_string(&config.sessions).unwrap()),
    &JsValue::from(&config.sessions),
  )
  .map_err(|e| JsError::new(&e.as_string().unwrap().as_str()))?;
  Reflect::set(
    &obj,
    &JsValue::from_str("full_sessions"),
    // &JsValue::from(serde_json::to_string(&config.full_sessions).unwrap()),
    &JsValue::from(&config.full_sessions),
  )
  .map_err(|e| JsError::new(&e.as_string().unwrap().as_str()))?;

  Ok(obj)
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
/// ```
/// let config = LLMsStandardConfig::get_llms_config("clap", Some("4.5.39")).await.unwrap();
/// ```
///
#[wasm_bindgen]
pub async fn get_llms_config_online(
  lib_name: &str,
  version: Option<String>,
) -> Result<JsValue, JsValue> {
  match LLMsStandardConfig::get_llms_config_online(lib_name, version).await {
    Ok(config) => match llms_config_to_js_object(config) {
      Ok(obj) => Ok(obj.into()),
      Err(_) => Ok(JsValue::undefined()),
    },
    Err(_err) => Ok(JsValue::undefined()),
  }
}

///
/// Get the LLM config by generating rustdoc with all features enabled.
///
/// This function takes a toolchain and a manifest path, generates rustdoc JSON
/// for the specified crate with all features enabled, and returns the LLM config
/// as a JavaScript object.
///
/// # Arguments
///
/// * `toolchain` - The Rust toolchain to use (e.g., "stable", "nightly").
/// * `manifest_path` - The path to the Cargo.toml file of the crate.
///
/// # Returns
///
/// * `Result<JsValue, JsValue>` - A JavaScript object representing the LLM config
///   on success, or `JsValue::undefined()` on failure.
///
/// # Examples
///
/// ```no_run
/// let config = get_llms_config_by_rustdoc_all_features("stable", "Cargo.toml").await;
/// ```
#[wasm_bindgen]
pub fn get_llms_config_by_rustdoc_all_features(
  toolchain: &str,
  manifest_path: &str,
) -> Result<JsValue, JsValue> {
  let manifest_path = PathBuf::from(manifest_path);
  match LLMsStandardConfig::get_llms_config_offline_with_all_features(
    toolchain,
    manifest_path,
  ) {
    Ok(config) => match llms_config_to_js_object(config) {
      Ok(obj) => Ok(obj.into()),
      Err(_) => Ok(JsValue::undefined()),
    },
    Err(_err) => Ok(JsValue::undefined()),
  }
}

///
/// Get the LLM config by generating rustdoc with specified features enabled.
///
/// This function takes a toolchain, manifest path, and a list of features,
/// generates rustdoc JSON for the specified crate with the specified features
/// enabled, and returns the LLM config as a JavaScript object.
///
/// # Arguments
///
/// * `toolchain` - The Rust toolchain to use (e.g., "stable", "nightly").
/// * `manifest_path` - The path to the Cargo.toml file of the crate.
/// * `no_default_features` - Whether to include the default features.
/// * `features` - The features to include.
///
/// # Returns
///
/// * `Result<JsValue, JsValue>` - A JavaScript object representing the LLM config
///   on success, or `JsValue::undefined()` on failure.
///
/// # Examples
///
/// ```no_run
/// let config = get_llms_config_by_rustdoc_features("stable", "Cargo.toml", true, vec!["async".to_string()]).await;
/// ```
#[wasm_bindgen]
pub async fn get_llms_config_by_rustdoc_features(
  toolchain: &str,
  manifest_path: &str,
  no_default_features: bool,
  features: Option<Vec<String>>,
) -> Result<JsValue, JsValue> {
  let manifest_path = PathBuf::from(manifest_path);
  match LLMsStandardConfig::get_llms_config_offline_with_features(
    toolchain,
    manifest_path,
    no_default_features,
    features,
  ) {
    Ok(config) => match llms_config_to_js_object(config) {
      Ok(obj) => Ok(obj.into()),
      Err(_) => Ok(JsValue::undefined()),
    },
    Err(_err) => Ok(JsValue::undefined()),
  }
}
