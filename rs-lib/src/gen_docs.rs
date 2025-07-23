use std::path::PathBuf;

use crate::error::{Error, Result};
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
) -> Result<GenDocs> {
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
    .ok_or("Failed to extract library name")
    .map_err(|e| Error::Config(e.to_string()))?;

  let json_string = std::fs::read_to_string(&json_path)?;
  let json_data: Crate = serde_json::from_str(&json_string)?;

  Ok(GenDocs {
    lib_name,
    docs: json_data,
  })
}

/// Generate docs for a given Cargo.toml file using the automatically detected toolchain.
///
/// This function detects whether the current Rust installation is nightly or stable
/// and uses the appropriate toolchain to generate documentation.
///
/// # Arguments
///
/// * `manifest_path` - The path to the Cargo.toml file.
///
/// # Returns
///
/// * `Result<GenDocs, Box<dyn std::error::Error>>` - The generated documentation data.
///
/// # Examples
///
/// ```no_run
/// use std::path::PathBuf;
/// let docs = gen_docs_with_all_features_auto_toolchain(PathBuf::from("Cargo.toml")).unwrap();
/// ```
pub fn gen_docs_with_all_features_auto_toolchain(
  manifest_path: PathBuf,
) -> Result<GenDocs> {
  let json_path = match rustversion::cfg!(nightly) {
    true => rustdoc_json_stable::Builder::default().toolchain("nightly"),
    false => rustdoc_json_stable::Builder::stable().toolchain("stable"),
  }
  .manifest_path(manifest_path)
  .all_features(true)
  .quiet(true)
  .build()?;

  let lib_name = json_path
    .as_path()
    .file_stem()
    .and_then(|stem| stem.to_str())
    .map(String::from)
    .ok_or("Failed to extract library name")
    .map_err(|e| Error::Config(e.to_string()))?;

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
) -> Result<GenDocs> {
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
    .ok_or("Failed to extract library name")
    .map_err(|e| Error::Config(e.to_string()))?;

  let json_string = std::fs::read_to_string(&json_path)?;
  let json_data: Crate = serde_json::from_str(&json_string)?;

  Ok(GenDocs {
    lib_name,
    docs: json_data,
  })
}

/// Generate docs for a given Cargo.toml file using automatically detected toolchain.
///
/// This function detects whether the current Rust installation is nightly or stable
/// and uses the appropriate toolchain to generate documentation with specified features.
///
/// # Arguments
///
/// * `manifest_path` - The path to the Cargo.toml file.
/// * `no_default_features` - Whether to disable default features.
/// * `features` - Optional list of specific features to enable.
///
/// # Returns
///
/// * `Result<GenDocs, Box<dyn std::error::Error>>` - The generated documentation data.
///
/// # Examples
///
/// ```no_run
/// use std::path::PathBuf;
/// let docs = gen_docs_with_features_auto_toolchain(
///     PathBuf::from("Cargo.toml"),
///     true,
///     Some(vec!["async".to_string()])
/// ).unwrap();
/// ```
pub fn gen_docs_with_features_auto_toolchain(
  manifest_path: PathBuf,
  no_default_features: bool,
  features: Option<Vec<String>>,
) -> Result<GenDocs> {
  let mut builder = match rustversion::cfg!(nightly) {
    true => rustdoc_json_stable::Builder::default().toolchain("nightly"),
    false => rustdoc_json_stable::Builder::stable().toolchain("stable"),
  }
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
    .ok_or("Failed to extract library name")
    .map_err(|e| Error::Config(e.to_string()))?;

  let json_string = std::fs::read_to_string(&json_path)?;
  let json_data: Crate = serde_json::from_str(&json_string)?;

  Ok(GenDocs {
    lib_name,
    docs: json_data,
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::gen_docs::{gen_docs_with_all_features, gen_docs_with_features};
  use std::env;

  #[cfg(feature = "rustdoc")]
  #[test]
  fn test_gen_docs_with_all_features_success() {
    let current_dir = std::env::current_dir().unwrap();
    let gen_docs_struct =
      gen_docs_with_all_features("stable", current_dir.join("Cargo.toml"))
        .unwrap();

    assert_eq!(gen_docs_struct.lib_name, "crates_llms_txt");
  }

  #[cfg(feature = "rustdoc")]
  #[test]
  fn test_gen_docs_with_features_success() {
    let current_dir = std::env::current_dir().unwrap();
    let gen_docs_struct = gen_docs_with_features(
      "stable",
      current_dir.join("Cargo.toml"),
      true,
      Some(vec!["rustdoc".to_string()]),
    )
    .unwrap();
    assert_eq!(gen_docs_struct.lib_name, "crates_llms_txt");
  }

  #[cfg(feature = "rustdoc")]
  #[test]
  fn test_gen_docs_with_all_features_auto_toolchain() {
    let current_dir = env::current_dir().unwrap();
    let gen_docs_struct =
      gen_docs_with_all_features_auto_toolchain(current_dir.join("Cargo.toml"))
        .unwrap();

    assert_eq!(gen_docs_struct.lib_name, "crates_llms_txt");
  }

  #[cfg(feature = "rustdoc")]
  #[test]
  #[should_panic(expected = "No such file or directory")]
  fn test_gen_docs_with_all_features_auto_toolchain_invalid_path() {
    let invalid_path = PathBuf::from("nonexistent/Cargo.toml");
    gen_docs_with_all_features_auto_toolchain(invalid_path).unwrap();
  }

  #[cfg(feature = "rustdoc")]
  #[test]
  fn test_gen_docs_with_features_auto_toolchain_success() {
    let current_dir = env::current_dir().unwrap();
    let gen_docs_struct = gen_docs_with_features_auto_toolchain(
      current_dir.join("Cargo.toml"),
      true,
      Some(vec!["rustdoc".to_string()]),
    )
    .unwrap();

    assert_eq!(gen_docs_struct.lib_name, "crates_llms_txt");
  }

  #[cfg(feature = "rustdoc")]
  #[test]
  #[should_panic(expected = "No such file or directory")]
  fn test_gen_docs_with_features_auto_toolchain_invalid_path() {
    let invalid_path = PathBuf::from("nonexistent/Cargo.toml");
    gen_docs_with_features_auto_toolchain(invalid_path, false, None).unwrap();
  }

  #[cfg(feature = "rustdoc")]
  #[test]
  fn test_gen_docs_with_features_auto_toolchain_no_features() {
    let current_dir = env::current_dir().unwrap();
    let gen_docs_struct = gen_docs_with_features_auto_toolchain(
      current_dir.join("Cargo.toml"),
      false,
      None,
    )
    .unwrap();

    assert_eq!(gen_docs_struct.lib_name, "crates_llms_txt");
  }
}
