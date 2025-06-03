use std::collections::HashMap;

use rustdoc_types::{Id, Item};

#[cfg(feature = "rustdoc")]
use crate::gen_docs::Crate;

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

#[cfg(feature = "rustdoc")]
impl CommonCrates for Crate {
  fn crate_version(&self) -> String {
    self.crate_version.clone().unwrap_or("latest".to_string())
  }
  fn index(&self) -> HashMap<Id, Item> {
    self.index.clone()
  }
}
