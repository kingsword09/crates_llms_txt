use std::path::PathBuf;

use crate::temp_trait::Crate;

#[derive(Debug, Clone)]
pub struct GenDocs {
  pub lib_name: String,
  // pub docs: rustdoc_types::Crate,
  pub docs: Crate,
}

/// Generate docs for a given Cargo.toml file.
///
/// # Arguments
///
/// * `toolchain` - The toolchain to use.
/// * `manifest_path` - The path to the Cargo.toml file.
///
/// # Returns
///
/// * `Result<GenDocs, Box<dyn std::error::Error>>` - The path to the generated docs.
///
/// # Examples
///
/// ```no_run
/// let docs_path = gen_docs_with_all_features("nightly", "Cargo.toml").unwrap();
/// ```
pub fn gen_docs_with_all_features(
  toolchain: &str,
  manifest_path: PathBuf,
) -> Result<GenDocs, Box<dyn std::error::Error>> {
  let json_path = match toolchain {
    "nightly" => rustdoc_json_stable::Builder::default(),
    _ => rustdoc_json_stable::Builder::stable(),
  }
  .toolchain(toolchain)
  .manifest_path(manifest_path)
  .all_features(true)
  .quiet(true)
  .build()?;

  let lib_name = json_path
    .as_path()
    .file_stem()
    .and_then(|stem| stem.to_str())
    .map(String::from)
    .ok_or("Failed to extract library name")?;

  let json_string = std::fs::read_to_string(&json_path)?;
  let json_data: Crate = serde_json::from_str(&json_string)?;

  Ok(GenDocs {
    lib_name,
    docs: json_data,
  })
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
/// * `Result<GenDocs, Box<dyn std::error::Error>>` - The path to the generated docs.
///
/// # Examples
///
/// ```no_run
/// let docs_path = gen_docs_with_features("nightly", "Cargo.toml", true, Some(vec!["async".to_string()])).unwrap();
/// ```
pub fn gen_docs_with_features(
  toolchain: &str,
  manifest_path: PathBuf,
  no_default_features: bool,
  features: Option<Vec<String>>,
) -> Result<GenDocs, Box<dyn std::error::Error>> {
  let mut builder = match toolchain {
    "nightly" => rustdoc_json_stable::Builder::default(),
    _ => rustdoc_json_stable::Builder::stable(),
  }
  .toolchain(toolchain)
  .manifest_path(manifest_path)
  .quiet(true);

  if no_default_features {
    builder = builder.no_default_features(true);
  }

  if features.is_some() {
    builder = builder.features(features.unwrap());
  }

  let json_path = builder.build()?;
  let lib_name = json_path
    .as_path()
    .file_stem()
    .and_then(|stem| stem.to_str())
    .map(String::from)
    .ok_or("Failed to extract library name")?;
  let json_string = std::fs::read_to_string(&json_path)?;
  let json_data: Crate = serde_json::from_str(&json_string)?;

  Ok(GenDocs {
    lib_name,
    docs: json_data,
  })
}

#[cfg(test)]
mod tests {
  use crate::gen_docs::gen_docs_with_all_features;

  #[cfg(feature = "rustdoc")]
  #[tokio::test]
  async fn test_gen_docs_with_all_features_success() {
    let current_dir = std::env::current_dir().unwrap();
    let gen_docs_struct =
      gen_docs_with_all_features("stable", current_dir.join("Cargo.toml"))
        .unwrap();

    assert_eq!(gen_docs_struct.lib_name, "crates_llms_txt");
  }
}
