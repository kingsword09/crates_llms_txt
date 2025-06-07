use rustdoc_types::{Id, Item};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait CommonCrates {
  fn crate_version(&self) -> String;
  fn index(&self) -> HashMap<Id, Item>;
}

impl CommonCrates for rustdoc_types::Crate {
  fn crate_version(&self) -> String {
    self.crate_version.clone().unwrap_or("latest".to_string())
  }

  fn index(&self) -> HashMap<Id, Item> {
    self.index.clone()
  }
}

impl CommonCrates for Crate {
  fn crate_version(&self) -> String {
    self.crate_version.clone().unwrap_or("latest".to_string())
  }
  fn index(&self) -> HashMap<Id, Item> {
    self.index.clone()
  }
}

/// A custom Crate structure that serves as a compatibility layer between rustdoc_types::Crate and local docs.
/// Since local documentation generation may not produce fields that exactly match rustdoc_types,
/// this structure provides a simplified schema to prevent deserialization failures.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Crate {
  /// The id of the root [`Module`] item of the local crate.
  pub root: Id,
  /// The version string given to `--crate-version`, if any.
  pub crate_version: Option<String>,
  /// Whether or not the output includes private items.
  pub includes_private: bool,
  /// A collection of all items in the local crate as well as some external traits and their
  /// items that are referenced locally.
  pub index: HashMap<Id, Item>,
  /// A single version number to be used in the future when making backwards incompatible changes
  /// to the JSON output.
  pub format_version: u32,
}
