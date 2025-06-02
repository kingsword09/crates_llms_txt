use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const DOCS_BASE_URL: &'static str = "https://docs.rs/crate";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SessionItem {
    title: String,
    description: String,
    link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FullSessionItem {
    content: String,
    link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsRoot {
    pub crate_version: String,
    pub index: HashMap<String, IndexItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexItem {
    pub id: u32,
    pub crate_id: u32,
    pub name: String,
    pub span: Span,
    pub visibility: String,
    pub docs: Option<String>,
    pub links: HashMap<String, String>,
    pub attrs: Vec<String>,
    pub deprecation: Option<String>,
    pub inner: Inner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span {
    pub filename: String,
    pub begin: Vec<u32>,
    pub end: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inner {
    pub module: Module,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub is_crate: bool,
    pub items: Vec<String>,
    pub is_stripped: bool,
}

struct StdDocs;

impl StdDocs {
    pub async fn fetch_docs(
        lib_name: &str,
        version: Option<&str>,
    ) -> Result<DocsRoot, Box<dyn std::error::Error>> {
        let version = version.unwrap_or("latest");

        let response = reqwest::get(format!("{}/{}/{}/json", DOCS_BASE_URL, lib_name, version))
            .await?
            .text()
            .await?;

        let root: DocsRoot = match serde_json::from_str(&response) {
            Ok(val) => val,
            Err(e) => {
                return Err(Box::new(e));
            }
        };

        Ok(root)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMsStandardConfig {
    sessions: Vec<SessionItem>,
    full_sessions: Vec<FullSessionItem>,
}

impl LLMsStandardConfig {
    pub fn new() -> Self {
        Self {
            sessions: Vec::new(),
            full_sessions: Vec::new(),
        }
    }

    pub async fn get_llms_config(
        lib_name: &str,
        version: Option<&str>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut config = LLMsStandardConfig::new();
        if let Ok(docs) = StdDocs::fetch_docs(lib_name, version).await {
            let base_url = format!(
                "{}/{}/{}/source",
                DOCS_BASE_URL,
                lib_name,
                version.unwrap_or(docs.crate_version.as_str())
            );
            config.sessions.push(SessionItem {
                title: lib_name.to_string(),
                description: "".to_string(),
                link: format!(
                    "https://docs.rs/{}/{}",
                    lib_name,
                    version.unwrap_or(&docs.crate_version)
                ),
            });
            for (_, item) in docs.index {
                if let Some(docs) = item.docs {
                    let link = format!("{}/{}", base_url, item.span.filename);
                    config.sessions.push(SessionItem {
                        title: item.span.filename,
                        description: "".to_string(),
                        link: link.clone(),
                    });
                    config.full_sessions.push(FullSessionItem {
                        content: docs,
                        link: link,
                    });
                }
            }
        }

        return Ok(config);
    }
}
