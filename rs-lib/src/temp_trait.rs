//! # Temporary Trait Definitions and Compatibility Layer
//!
//! This module provides compatibility between different versions of rustdoc JSON formats.
//! It defines a common trait interface and internal data structures that can handle
//! variations in the rustdoc_types crate across different Rust versions.
//!
//! ## Purpose
//!
//! The rustdoc JSON format has evolved over time, and different versions of the
//! `rustdoc_types` crate may have incompatible structures. This module provides:
//!
//! - A unified `CommonCrates` trait for accessing crate data
//! - Internal `Item` and `Crate` structures for compatibility
//! - Conversion implementations between different format versions

use rustdoc_types::{
  Deprecation, ExternalCrate, Id, ItemEnum, ItemSummary, Span, Target,
  Visibility,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents any documented item in a Rust crate.
///
/// This structure mirrors `rustdoc_types::Item` but provides compatibility
/// across different rustdoc format versions. It can represent modules, structs,
/// enums, functions, traits, and other documented items.
///
/// The `Item` holds common fields that apply to all documentation items,
/// while type-specific details are stored in the `inner` field.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
  /// Unique identifier for this item within the crate's documentation
  pub id: Id,
  /// Identifier of the crate this item belongs to (for cross-crate references)
  pub crate_id: u32,
  /// Name of the item (None for anonymous items like impl blocks)
  pub name: Option<String>,
  /// Source code location information (None for generated or macro-expanded items)
  pub span: Option<Span>,
  /// Visibility level (public, private, crate-local, etc.)
  pub visibility: Visibility,
  /// Complete markdown documentation string (None if undocumented)
  pub docs: Option<String>,
  /// Mapping of intra-doc links to their target item IDs
  pub links: HashMap<String, Id>,
  /// Rust attributes applied to this item (excluding #[deprecated])
  ///
  /// Attributes are normalized to their canonical form:
  /// - `#[non_exhaustive]`, `#[must_use]` appear as-is
  /// - `#[repr(C)]` may have reordered parameters
  /// - Multiple similar attributes may be combined
  pub attrs: Vec<String>,
  /// Deprecation information if the item is deprecated
  pub deprecation: Option<Deprecation>,
  /// Type-specific data (function signatures, struct fields, etc.)
  pub inner: ItemEnum,
}

/// Common interface for accessing crate documentation data.
///
/// This trait provides a unified API for accessing rustdoc data regardless
/// of the underlying format version. It abstracts over differences between
/// `rustdoc_types::Crate` and our internal `Crate` structure.
///
/// All implementations must be thread-safe (`Send + Sync`) to support
/// concurrent processing of documentation data.
pub trait CommonCrates: Send + Sync {
  /// Returns the ID of the root module of this crate
  fn root_id(&self) -> Id;

  /// Returns the version string of this crate
  ///
  /// If no version is available, returns "latest" as a fallback
  fn crate_version(&self) -> String;

  /// Returns a mapping of all documented items in this crate
  ///
  /// The HashMap maps item IDs to their corresponding `Item` structures,
  /// providing access to all documentation content and metadata
  fn index(&self) -> HashMap<Id, Item>;
}

/// Implementation of `CommonCrates` for the standard `rustdoc_types::Crate`.
///
/// This implementation converts the standard rustdoc format into our internal
/// representation, handling attribute serialization and other format differences.
impl CommonCrates for rustdoc_types::Crate {
  fn root_id(&self) -> Id {
    self.root
  }

  fn crate_version(&self) -> String {
    self.crate_version.clone().unwrap_or("latest".to_string())
  }

  fn index(&self) -> HashMap<Id, Item> {
    // Pre-allocate HashMap with known capacity for better performance
    let mut hash_map = HashMap::with_capacity(self.index.len());

    // Convert each rustdoc_types::Item to our internal Item format
    for (&id, item) in &self.index {
      let converted_item = Item {
        id: item.id,
        crate_id: item.crate_id,
        name: item.name.clone(),
        span: item.span.clone(),
        visibility: item.visibility.clone(),
        docs: item.docs.clone(),
        links: item.links.clone(),
        // Convert attributes to JSON strings for consistent handling
        // Use try_collect to handle potential serialization errors gracefully
        attrs: item
          .attrs
          .iter()
          .filter_map(|attr| serde_json::to_string(attr).ok())
          .collect(),
        deprecation: item.deprecation.clone(),
        inner: item.inner.clone(),
      };
      hash_map.insert(id, converted_item);
    }

    hash_map
  }
}

/// Implementation of `CommonCrates` for our internal `Crate` structure.
///
/// This implementation provides direct access to the internal format without
/// any conversion overhead, as the data is already in the expected format.
impl CommonCrates for Crate {
  fn root_id(&self) -> Id {
    self.root
  }

  fn crate_version(&self) -> String {
    self.crate_version.clone().unwrap_or("latest".to_string())
  }

  fn index(&self) -> HashMap<Id, Item> {
    // Direct clone since the format already matches our internal representation
    self.index.clone()
  }
}

/// Internal representation of a Rust crate's documentation.
///
/// This structure mirrors `rustdoc_types::Crate` but uses our internal `Item`
/// format for compatibility across different rustdoc versions. It contains
/// all the metadata and documentation content for a complete crate.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Crate {
  /// ID of the root module item for this crate
  pub root: Id,
  /// Version string of the crate (from Cargo.toml or --crate-version)
  pub crate_version: Option<String>,
  /// Whether private items are included in the documentation
  pub includes_private: bool,
  /// Complete index of all documented items in this crate
  ///
  /// Maps item IDs to their full documentation data, including both
  /// local items and referenced external items
  pub index: HashMap<Id, Item>,
  /// Mapping of item IDs to their fully qualified paths and metadata
  ///
  /// Used for generating cross-references and navigation links
  pub paths: HashMap<Id, ItemSummary>,
  /// Information about external crates referenced by this crate
  ///
  /// Maps crate IDs to crate names and documentation URLs
  pub external_crates: HashMap<u32, ExternalCrate>,
  /// Target platform information for this documentation build
  pub target: Target,
  /// Format version number for backward compatibility
  ///
  /// Used to handle changes in the JSON schema over time
  pub format_version: u32,
}
