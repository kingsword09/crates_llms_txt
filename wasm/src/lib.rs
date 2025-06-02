use crates_llms_txt::LLMsStandardConfig;
use js_sys::{Object, Reflect};
use wasm_bindgen::prelude::*;

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
pub async fn get_llms_config(
  lib_name: &str,
  version: Option<String>,
) -> Result<JsValue, JsValue> {
  match LLMsStandardConfig::get_llms_config(lib_name, version).await {
    Ok(config) => {
      println!("{:#?}", config);
      let obj = Object::new();
      Reflect::set(
        &obj,
        &JsValue::from_str("lib_name"),
        &JsValue::from(config.lib_name),
      )?;
      Reflect::set(
        &obj,
        &JsValue::from_str("version"),
        &JsValue::from(config.version),
      )?;
      Reflect::set(
        &obj,
        &JsValue::from_str("sessions"),
        // &JsValue::from(serde_json::to_string(&config.sessions).unwrap()),
        &JsValue::from(&config.sessions),
      )?;
      Reflect::set(
        &obj,
        &JsValue::from_str("full_sessions"),
        // &JsValue::from(serde_json::to_string(&config.full_sessions).unwrap()),
        &JsValue::from(&config.full_sessions),
      )?;

      Ok(obj.into())
    }
    Err(_err) => Ok(JsValue::undefined()),
  }
}
