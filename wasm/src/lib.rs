use crates_llms_txt::LLMsStandardConfig;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn get_llms_config(lib_name: &str, version: Option<String>) -> Result<JsValue, JsValue> {
    match LLMsStandardConfig::get_llms_config(lib_name, version).await {
        Ok(config) => serde_wasm_bindgen::to_value(&config)
            .map_err(|err| JsValue::from(js_sys::Error::new(&format!("{:#}", err)))),
        Err(err) => Err(JsValue::from(js_sys::Error::new(&format!("{:#}", err)))),
    }
}
