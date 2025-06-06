use reqwest::header;

const DOCS_BASE_URL: &str = "https://docs.rs/crate";

pub struct OnlineDocs;

impl OnlineDocs {
  /// Fetch the docs for a given crate and version.
  ///
  /// # Arguments
  ///
  /// * `lib_name` - The name of the crate.
  /// * `version` - The version of the crate. If None, the latest version will be used.
  ///
  /// # Returns
  ///
  /// * `Result<rustdoc_types::Crate, Box<dyn std::error::Error>>` - The docs for the crate.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// let docs = OnlineDocs::fetch_docs("clap", Some("4.5.39")).await.unwrap();
  /// ```
  ///
  pub async fn fetch_docs(
    lib_name: &str,
    version: Option<String>,
  ) -> Result<rustdoc_types::Crate, Box<dyn std::error::Error>> {
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
    let json_data: rustdoc_types::Crate =
      serde_json::from_slice(&decompressed_bytes)?;

    Ok(json_data)
  }
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[tokio::test]
  async fn test_fetch_docs() {
    let version = "4.5.39".to_string();
    let docs = OnlineDocs::fetch_docs("clap", Some(version.clone()))
      .await
      .unwrap();

    assert_eq!(docs.crate_version.unwrap(), version);
  }

  #[tokio::test]
  async fn test_fetch_docs_failed() {
    let version = "0.53.3".to_string();

    let result = OnlineDocs::fetch_docs("opendal", Some(version.clone()))
      .await
      .unwrap_err();

    assert!(result
      .to_string()
      .contains("expected value at line 1 column 1"));
  }
}
