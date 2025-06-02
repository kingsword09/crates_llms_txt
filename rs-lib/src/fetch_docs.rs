use reqwest::header;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const DOCS_BASE_URL: &'static str = "https://docs.rs/crate";

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

pub struct StdDocs;

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
    let json_data: DocsRoot = serde_json::from_slice(&decompressed_bytes)?;

    Ok(json_data)
  }
}
