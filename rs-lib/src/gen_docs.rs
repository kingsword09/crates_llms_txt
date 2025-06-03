use std::path::PathBuf;

/// Generate docs for a given Cargo.toml file.
///
/// # Arguments
///
/// * `toolchain` - The toolchain to use.
/// * `manifest_path` - The path to the Cargo.toml file.
///
/// # Returns
///
/// * `Result<PathBuf, Box<dyn std::error::Error>>` - The path to the generated docs.
///
/// # Examples
///
/// ```
/// let docs_path = gen_docs("nightly", "Cargo.toml", None).unwrap();
/// ```
pub fn gen_docs_with_all_features(
  toolchain: &str,
  manifest_path: PathBuf,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
  let json_path = rustdoc_json::Builder::default()
    .toolchain(toolchain)
    .manifest_path(manifest_path)
    .all_features(true)
    .build()?;

  Ok(json_path)
}

/// Generate docs for a given Cargo.toml file.
///
/// # Arguments
///
/// * `toolchain` - The toolchain to use.
/// * `manifest_path` - The path to the Cargo.toml file.
/// * `no_default_features` - Whether to include the default features.
/// * `features` - The features to include.
///
/// # Returns
///
/// * `Result<PathBuf, Box<dyn std::error::Error>>` - The path to the generated docs.
///
/// # Examples
///
/// ```
/// let docs_path = gen_docs_with_features("nightly", "Cargo.toml", true, vec!["async".to_string()]).unwrap();
/// ```
pub fn gen_docs_with_features(
  toolchain: &str,
  manifest_path: &str,
  no_default_features: bool,
  features: Option<Vec<String>>,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
  let mut builder = rustdoc_json::Builder::default()
    .toolchain(toolchain)
    .manifest_path(manifest_path);
  if no_default_features {
    builder = builder.no_default_features(true);
  }

  if features.is_some() {
    builder = builder.features(features.unwrap());
  }

  let json_path = builder.build()?;

  Ok(json_path)
}

#[cfg(test)]
mod tests {
  use crate::gen_docs::gen_docs_with_all_features;

  #[tokio::test]
  async fn test_gen_docs_with_all_features() {
    let current_dir = std::env::current_dir().unwrap();
    let json_path =
      gen_docs_with_all_features("stable", current_dir.join("Cargo.toml"))
        .unwrap_err();

    println!("{:?}", json_path);
  }
}
