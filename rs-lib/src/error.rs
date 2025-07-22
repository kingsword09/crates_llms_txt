use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("network error: {0}")]
  Network(#[from] reqwest::Error),
  #[error("JSON parsing error: {0}")]
  Json(#[from] serde_json::Error),
  #[error("I/O error: {0}")]
  Io(#[from] std::io::Error),
  #[cfg(feature = "rustdoc")]
  #[error("rustdoc build error: {0}")]
  Build(#[from] rustdoc_json_stable::BuildError),
  #[error("failed to get llms config: {0}")]
  Config(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
