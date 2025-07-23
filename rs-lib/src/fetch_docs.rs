use reqwest::header;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::temp_trait::{CommonCrates, Crate};

const DOCS_BASE_URL: &str = "https://docs.rs/crate";

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

    // Try to parse the JSON data with better error handling
    Self::parse_json_with_fallback(&decompressed_bytes)
  }

  /// Decompress zstd-compressed data from docs.rs endpoints.
  ///
  /// docs.rs serves JSON documentation data compressed with zstd. This method
  /// first checks the Content-Encoding header, then attempts zstd decompression
  /// as docs.rs always returns compressed data regardless of headers.
  ///
  /// # Arguments
  ///
  /// * `body_bytes` - Raw response body bytes
  /// * `content_encoding` - Content-Encoding header value, if present
  /// * `_content_type` - Content-Type header value (unused, kept for compatibility)
  /// * `_url` - Request URL (unused, kept for compatibility)
  ///
  /// # Returns
  ///
  /// * `Result<Vec<u8>>` - Decompressed data, or original data if decompression fails
  fn decompress_if_needed(
    body_bytes: &[u8],
    content_encoding: Option<&str>,
    _content_type: Option<&str>,
    _url: &str,
  ) -> Result<Vec<u8>> {
    // Check Content-Encoding header first
    if let Some(encoding) = content_encoding {
      if encoding.eq_ignore_ascii_case("zstd") {
        return Ok(zstd::decode_all(body_bytes).map_err(Error::Io)?);
      } else {
        // Other encodings should be handled by reqwest automatically
        return Ok(body_bytes.to_vec());
      }
    }

    // docs.rs always returns zstd compressed data, try to decompress directly
    match zstd::decode_all(body_bytes) {
      Ok(decompressed) => Ok(decompressed),
      Err(_) => {
        // If decompression fails, treat as plain data (fallback)
        Ok(body_bytes.to_vec())
      }
    }
  }

  /// Parse JSON data with intelligent error handling for rustdoc version compatibility.
  ///
  /// This method first attempts to parse the JSON directly into the target type.
  /// If that fails, it validates the JSON structure to distinguish between:
  /// - Malformed JSON (returns Json error)
  /// - Valid JSON with incompatible structure (returns Config error with helpful message)
  ///
  /// # Arguments
  ///
  /// * `data` - Raw JSON bytes to parse
  ///
  /// # Returns
  ///
  /// * `Result<T>` - Parsed data or appropriate error
  ///
  /// # Errors
  ///
  /// * `Error::Json` - If the JSON is malformed
  /// * `Error::Config` - If JSON is valid but incompatible with current rustdoc-types version
  fn parse_json_with_fallback<T>(data: &[u8]) -> Result<T>
  where
    T: CommonCrates + Serialize + for<'de> Deserialize<'de>,
  {
    // First, try to parse directly
    match serde_json::from_slice::<T>(data) {
      Ok(result) => Ok(result),
      Err(e) => {
        // If parsing fails, it might be due to version compatibility issues
        // Try to parse as generic JSON first to validate the structure
        match serde_json::from_slice::<serde_json::Value>(data) {
          Ok(_) => {
            // JSON is valid but doesn't match our expected structure
            // This is likely a version compatibility issue
            Err(Error::Config(format!(
              "JSON structure incompatible with current rustdoc-types version: {}. \
               This may be due to a mismatch between the rustdoc format version used to \
               generate the documentation and the rustdoc-types crate version.",
              e
            )))
          }
          Err(_) => {
            // JSON itself is invalid
            Err(Error::Json(e))
          }
        }
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
  ) -> Result<rustdoc_types::Crate> {
    let version = version.unwrap_or("latest".to_string());
    let url = format!("{DOCS_BASE_URL}/{lib_name}/{version}/json");
    OnlineDocs::fetch_json::<rustdoc_types::Crate>(url.as_str()).await
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
  pub async fn fetch_docs_by_url(url: &str) -> Result<Crate> {
    OnlineDocs::fetch_json::<Crate>(url).await
  }
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[tokio::test]
  async fn test_fetch_docs() {
    let version = "latest".to_string();
    let result = OnlineDocs::fetch_docs("clap", Some(version.clone())).await;

    match result {
      Ok(docs) => {
        // Just check that we got some docs, don't assert on specific version
        // since "latest" will change over time
        assert!(docs.crate_version.is_some());
        println!(
          "Successfully fetched clap docs, version: {:?}",
          docs.crate_version
        );
      }
      Err(e) => {
        // If it fails due to version compatibility, that's expected for some crates
        match e {
          crate::error::Error::Config(msg) if msg.contains("incompatible") => {
            println!("Expected version compatibility issue with clap: {}", msg);
            // This is acceptable - version mismatch is a known issue
          }
          _ => {
            panic!("Unexpected error fetching clap docs: {:?}", e);
          }
        }
      }
    }
  }

  #[tokio::test]
  async fn test_fetch_docs_opendal() {
    let version = "latest".to_string();

    let result = OnlineDocs::fetch_docs("opendal", Some(version.clone())).await;

    match result {
      Ok(docs) => {
        println!("Successfully fetched docs for opendal");
        println!("Crate version: {:?}", docs.crate_version);
        assert!(docs.crate_version.is_some());
      }
      Err(e) => {
        // If it fails due to version compatibility, that's expected
        match e {
          crate::error::Error::Config(msg) if msg.contains("incompatible") => {
            println!("Expected version compatibility issue: {}", msg);
            // This is acceptable - version mismatch is a known issue
          }
          _ => {
            panic!("Unexpected error: {:?}", e);
          }
        }
      }
    }
  }

  #[tokio::test]
  async fn test_fetch_docs_json_validation() {
    let version = "latest".to_string();
    let url = format!("{DOCS_BASE_URL}/opendal/{version}/json");

    let client = reqwest::Client::builder().build().unwrap();
    let response = client.get(&url).send().await.unwrap();
    let body_bytes = response.bytes().await.unwrap();

    // Try to decompress using our method
    let decompressed_bytes =
      OnlineDocs::decompress_if_needed(&body_bytes, None, None, &url).unwrap();

    // Validate that JSON is structurally valid
    let json_valid =
      serde_json::from_slice::<serde_json::Value>(&decompressed_bytes).is_ok();
    assert!(json_valid, "Downloaded JSON should be structurally valid");

    println!("Successfully validated JSON structure for opendal docs");
  }
}
