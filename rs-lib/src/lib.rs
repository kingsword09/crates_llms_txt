use reqwest::header;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const DOCS_BASE_URL: &'static str = "https://docs.rs/crate";

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
pub struct DocsRoot {
  pub crate_version: String,
  pub index: HashMap<String, IndexItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexItem {
  // pub id: u32,
  // pub crate_id: u32,
  pub name: Option<String>,
  pub span: Span,
  // pub visibility: String,
  pub docs: Option<String>,
  // pub links: HashMap<String, String>,
  // pub attrs: Vec<String>,
  // pub deprecation: Option<String>,
  // pub inner: Inner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span {
  pub filename: String,
  pub begin: Vec<u32>,
  pub end: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inner {
  pub module: Module,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
  pub is_crate: bool,
  pub items: Vec<String>,
  pub is_stripped: bool,
}

struct StdDocs;

impl StdDocs {
  /// Fetch the docs for a given crate and version.
  ///
  /// # Arguments
  ///
  /// * `lib_name` - The name of the crate.
  /// * `version` - The version of the crate. If None, the latest version will be used.
  ///
  /// # Returns
  ///
  /// * `Result<DocsRoot, Box<dyn std::error::Error>>` - The docs for the crate.
  ///
  /// # Examples
  ///
  /// ```
  /// let docs = StdDocs::fetch_docs("clap", Some("4.5.39")).await.unwrap();
  /// ```
  ///
  pub async fn fetch_docs(
    lib_name: &str,
    version: Option<String>,
  ) -> Result<DocsRoot, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build().unwrap();
    let version = version.unwrap_or("latest".to_string());
    let url = format!("{}/{}/{}/json", DOCS_BASE_URL, lib_name, version);
    let response = client.get(url).send().await?;
    let headers = response.headers().clone();

    // Get the response body as raw bytes
    let body_bytes = response.bytes().await?;

    let content_encoding = headers
      .get(header::CONTENT_ENCODING)
      .and_then(|value| value.to_str().ok());

    let decompressed_bytes: Vec<u8>;

    if let Some(encoding) = content_encoding {
      if encoding.eq_ignore_ascii_case("zstd") {
        println!("Content-Encoding is Zstd. Decompressing...");
        decompressed_bytes = zstd::decode_all(&body_bytes[..])?;
      } else {
        println!("Content-Encoding is '{}', but we are only explicitly handling 'zstd'. Assuming reqwest handled it or it's plain.", encoding);
        decompressed_bytes = body_bytes.into_iter().collect(); // Convert Bytes to Vec<u8>
      }
    } else {
      println!("No Content-Encoding header. Assuming plain data.");
      decompressed_bytes = body_bytes.into_iter().collect(); // Convert Bytes to Vec<u8>
    }

    // Now, parse the (potentially decompressed) bytes as JSON
    let json_data: DocsRoot =
      serde_json::from_slice(&decompressed_bytes)?;

    Ok(json_data)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMsStandardStringConfig {
  pub lib_name: String,
  pub version: String,
  pub sessions: String,
  pub full_sessions: String,
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
  /// * `Result<LLMsStandardConfig, Box<dyn std::error::Error>>` - The LLM config for the crate.
  ///
  /// # Examples
  ///
  /// ```
  /// let config = LLMsStandardConfig::get_llms_config("clap", Some("4.5.39")).await?;
  /// ```
  ///
  pub async fn get_llms_config(
    lib_name: &str,
    version: Option<String>,
  ) -> Result<LLMsStandardStringConfig, Box<dyn std::error::Error>> {
    if let Ok(docs) = StdDocs::fetch_docs(lib_name, version.clone()).await {
      let version = version.unwrap_or(docs.crate_version);
      let mut config = LLMsStandardConfig::new(lib_name, &version);
      let base_url =
        format!("{}/{}/{}/source", DOCS_BASE_URL, lib_name, version);
      config.sessions.push(SessionItem {
        title: lib_name.to_string(),
        description: "".to_string(),
        link: format!("https://docs.rs/{}/{}", lib_name, version),
      });
      for (_, item) in docs.index {
        if let Some(docs) = item.docs {
          let link = format!("{}/{}", base_url, item.span.filename);
          config.sessions.push(SessionItem {
            title: match item.name {
              Some(name) => name,
              None => item.span.filename,
            },
            description: "".to_string(),
            link: link.clone(),
          });
          config.full_sessions.push(FullSessionItem {
            content: docs,
            link,
          });
        }
      }

      return Ok(LLMsStandardStringConfig {
        lib_name: config.lib_name,
        version: config.version,
        sessions: serde_json::to_string(&config.sessions).unwrap_or("".to_string()),
        full_sessions: serde_json::to_string(&config.full_sessions).unwrap_or("".to_string()),
      });
    }

    Err("Failed to get llms config".into())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_fetch_docs() {
    let version = "4.5.39".to_string();
    let docs = StdDocs::fetch_docs("clap", Some(version.clone()))
      .await
      .unwrap();

    assert_eq!(docs.crate_version, version);
  }

  #[tokio::test]
  async fn test_fetch_docs_failed() {
    let version = "0.53.3".to_string();
    
    let result = StdDocs::fetch_docs("opendal", Some(version.clone()))
        .await
        .unwrap_err();
    
    assert!(result.to_string().contains("expected value at line 1 column 1"));
  }

  #[tokio::test]
  async fn test_get_llms_config() {
    let lib_name = "clap";
    let version = "4.5.39".to_string();
    let config =
      LLMsStandardConfig::get_llms_config(lib_name, Some(version.clone()))
        .await
        .unwrap();

    assert_eq!(config.lib_name, lib_name);
    assert_eq!(config.version, version);
  }
}
