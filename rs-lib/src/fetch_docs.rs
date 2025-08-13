//! # Online Documentation Fetcher
//!
//! This module provides functionality to fetch and process rustdoc JSON data from online sources,
//! primarily docs.rs. It handles automatic decompression of zstd-compressed data and provides
//! fallback mechanisms for different rustdoc format versions.
//!
//! ## Key Features
//!
//! - **Automatic Decompression**: Handles zstd compression used by docs.rs
//! - **Version Compatibility**: Falls back between different rustdoc JSON formats
//! - **Error Handling**: Comprehensive error handling for network and parsing issues
//! - **Flexible URLs**: Support for both docs.rs and custom documentation servers

use reqwest::header;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::temp_trait::{CommonCrates, Crate};

/// Base URL for docs.rs crate documentation API endpoints
const DOCS_BASE_URL: &str = "https://docs.rs/crate";

/// Utility struct for fetching online documentation from docs.rs and other sources.
///
/// This struct provides static methods for downloading, decompressing, and parsing
/// rustdoc JSON data from various online sources.
pub struct OnlineDocs;

impl OnlineDocs {
  /// Fetch and parse JSON data from a URL, handling zstd compression automatically.
  ///
  /// This method automatically detects and decompresses zstd-compressed data from docs.rs,
  /// then attempts to parse it into the specified type. It includes fallback error handling
  /// for version compatibility issues between different rustdoc format versions.
  ///
  /// # Arguments
  ///
  /// * `url` - The URL to fetch JSON data from (typically a docs.rs JSON endpoint)
  ///
  /// # Returns
  ///
  /// * `Result<T>` - The parsed JSON data of type T, or an error if fetching/parsing fails
  ///
  /// # Errors
  ///
  /// * `Error::Network` - If the HTTP request fails
  /// * `Error::Io` - If zstd decompression fails
  /// * `Error::Json` - If JSON parsing fails due to malformed data
  /// * `Error::Config` - If JSON structure is incompatible with the expected rustdoc-types version
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use crates_llms_txt::fetch_docs::OnlineDocs;
  ///
  /// #[tokio::main]
  /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
  ///   let json = OnlineDocs::fetch_json::<rustdoc_types::Crate>(
  ///     "https://docs.rs/crate/clap/latest/json"
  ///   ).await?;
  ///
  ///   Ok(())
  /// }
  /// ```
  ///
  pub async fn fetch_json<T>(url: &str) -> Result<T>
  where
    T: CommonCrates + Serialize + for<'de> Deserialize<'de>,
  {
    let client = reqwest::Client::builder().build()?;
    let response = client.get(url).send().await?;
    let headers = response.headers().clone();

    // Get the response body as raw bytes
    let body_bytes = response.bytes().await?;

    let content_encoding = headers
      .get(header::CONTENT_ENCODING)
      .and_then(|value| value.to_str().ok());

    let content_type = headers
      .get(header::CONTENT_TYPE)
      .and_then(|value| value.to_str().ok());

    // Determine if we need to decompress
    let decompressed_bytes = Self::decompress_if_needed(
      &body_bytes,
      content_encoding,
      content_type,
      url,
    )?;

    serde_json::from_slice::<T>(&decompressed_bytes).map_err(Error::Json)
  }

  /// Decompress zstd-compressed data from docs.rs endpoints.
  ///
  /// docs.rs serves JSON documentation data compressed with zstd to reduce bandwidth.
  /// This method implements a two-stage decompression strategy:
  /// 1. Check the Content-Encoding header for explicit zstd indication
  /// 2. Attempt zstd decompression regardless, as docs.rs always compresses
  ///
  /// # Arguments
  ///
  /// * `body_bytes` - Raw HTTP response body bytes
  /// * `content_encoding` - Content-Encoding header value, if present
  /// * `_content_type` - Content-Type header (reserved for future use)
  /// * `_url` - Request URL (reserved for future use)
  ///
  /// # Returns
  ///
  /// * `Result<Vec<u8>>` - Decompressed data, or original data as fallback
  ///
  /// # Errors
  ///
  /// * `Error::Io` - If zstd decompression fails when explicitly indicated by headers
  fn decompress_if_needed(
    body_bytes: &[u8],
    content_encoding: Option<&str>,
    _content_type: Option<&str>,
    _url: &str,
  ) -> Result<Vec<u8>> {
    // First, check if the server explicitly indicates zstd compression
    if let Some(encoding) = content_encoding {
      if encoding.eq_ignore_ascii_case("zstd") {
        return zstd::decode_all(body_bytes).map_err(Error::Io);
      } else {
        // Other encodings (gzip, deflate) are handled automatically by reqwest
        return Ok(body_bytes.to_vec());
      }
    }

    // docs.rs always serves zstd-compressed data regardless of headers,
    // so attempt decompression even without explicit Content-Encoding
    match zstd::decode_all(body_bytes) {
      Ok(decompressed) => Ok(decompressed),
      Err(_) => {
        // If decompression fails, assume the data is already uncompressed
        // This handles cases where the server doesn't compress or uses different compression
        Ok(body_bytes.to_vec())
      }
    }
  }

  /// Fetch rustdoc documentation for a specific crate and version from docs.rs.
  ///
  /// This method constructs the appropriate docs.rs URL and fetches the zstd-compressed
  /// JSON documentation data. It handles decompression and parsing automatically.
  ///
  /// # Arguments
  ///
  /// * `lib_name` - The name of the crate (e.g., "clap", "serde")
  /// * `version` - The version of the crate. If None, "latest" will be used
  ///
  /// # Returns
  ///
  /// * `Result<rustdoc_types::Crate>` - The parsed rustdoc documentation data
  ///
  /// # Errors
  ///
  /// * `Error::Network` - If the HTTP request to docs.rs fails
  /// * `Error::Config` - If the rustdoc format version is incompatible
  /// * `Error::Json` - If the JSON data is malformed
  /// * `Error::Io` - If decompression fails
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use crates_llms_txt::fetch_docs::OnlineDocs;
  ///
  /// #[tokio::main]
  /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
  ///   // Fetch latest version
  ///   let docs = OnlineDocs::fetch_docs("clap", None).await?;
  ///   
  ///   // Fetch specific version
  ///   let docs = OnlineDocs::fetch_docs("clap", Some("4.5.39".to_string())).await?;
  ///
  ///   Ok(())
  /// }
  /// ```
  ///
  pub async fn fetch_docs(
    lib_name: &str,
    version: Option<String>,
  ) -> Result<Box<dyn CommonCrates>> {
    let version = version.unwrap_or("latest".to_string());
    let url = format!("{DOCS_BASE_URL}/{lib_name}/{version}/json");

    match OnlineDocs::fetch_json::<rustdoc_types::Crate>(url.as_str()).await {
      Ok(result) => Ok(Box::new(result)),
      Err(_) => match OnlineDocs::fetch_json::<Crate>(url.as_str()).await {
        Ok(result) => Ok(Box::new(result)),
        Err(err) => Err(err),
      },
    }
  }

  /// Fetch rustdoc documentation from a custom URL.
  ///
  /// This method allows fetching documentation from any URL that serves rustdoc JSON data,
  /// not just docs.rs. It handles zstd decompression and parsing automatically.
  ///
  /// # Arguments
  ///
  /// * `url` - The complete URL to fetch documentation from
  ///
  /// # Returns
  ///
  /// * `Result<Crate>` - The parsed rustdoc documentation data using the internal Crate type
  ///
  /// # Errors
  ///
  /// * `Error::Network` - If the HTTP request fails
  /// * `Error::Config` - If the rustdoc format version is incompatible
  /// * `Error::Json` - If the JSON data is malformed
  /// * `Error::Io` - If decompression fails
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use crates_llms_txt::fetch_docs::OnlineDocs;
  ///
  /// #[tokio::main]
  /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
  ///   // Fetch from docs.rs
  ///   let docs = OnlineDocs::fetch_docs_by_url(
  ///     "https://docs.rs/crate/clap/latest/json"
  ///   ).await?;
  ///   
  ///   // Fetch from custom documentation server
  ///   let docs = OnlineDocs::fetch_docs_by_url(
  ///     "https://my-docs-server.com/crate/my-crate/1.0.0/json"
  ///   ).await?;
  ///
  ///   Ok(())
  /// }
  /// ```
  ///
  pub async fn fetch_docs_by_url(url: &str) -> Result<Box<dyn CommonCrates>> {
    match OnlineDocs::fetch_json::<rustdoc_types::Crate>(url).await {
      Ok(result) => Ok(Box::new(result)),
      Err(_) => match OnlineDocs::fetch_json::<Crate>(url).await {
        Ok(result) => Ok(Box::new(result)),
        Err(err) => Err(err),
      },
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::{temp_trait::Crate, OnlineDocs, DOCS_BASE_URL};

  #[tokio::test]
  async fn test_fetch_docs() {
    let version = "latest".to_string();
    let docs = OnlineDocs::fetch_docs("clap", Some(version.clone()))
      .await
      .unwrap();

    // Just check that we got some docs, don't assert on specific version
    // since "latest" will change over time
    assert!(!docs.crate_version().is_empty());
    println!(
      "Successfully fetched clap docs, version: {:?}",
      docs.crate_version()
    );
  }

  #[tokio::test]
  async fn test_fetch_docs_opendal() {
    let version = "latest".to_string();

    let docs = OnlineDocs::fetch_docs("opendal", Some(version.clone()))
      .await
      .unwrap();

    println!("Successfully fetched docs for opendal");
    println!("Crate version: {:?}", docs.crate_version());
    assert!(!docs.crate_version().is_empty());
  }

  #[tokio::test]
  async fn test_fetch_docs_json_validation() {
    let version = "latest".to_string();
    let url = format!("{DOCS_BASE_URL}/serde/{version}/json");

    let client = reqwest::Client::builder().build().unwrap();
    let response = client.get(&url).send().await.unwrap();
    let body_bytes = response.bytes().await.unwrap();

    // Try to decompress using our method
    let decompressed_bytes =
      OnlineDocs::decompress_if_needed(&body_bytes, None, None, &url).unwrap();

    // Validate that JSON is structurally valid
    let json_valid =
      serde_json::from_slice::<Crate>(&decompressed_bytes).is_ok();
    assert!(json_valid, "Downloaded JSON should be structurally valid");

    println!("Successfully validated JSON structure for serde docs");
  }

  #[tokio::test]
  async fn test_fetch_docs_json_validation_x() {
    let version = "latest".to_string();
    let json_valid = OnlineDocs::fetch_docs("serde", Some(version.clone()))
      .await
      .is_ok();

    assert!(json_valid, "Downloaded JSON should be structurally valid");

    println!("Successfully validated JSON structure for serde docs");
  }
}
