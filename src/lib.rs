#[cfg(feature = "discord")]
pub mod discord;
pub mod engine;
pub mod language;
pub mod knowledge;
pub mod transform;
pub mod validate;

use anyhow::{anyhow, Result};
use language::Language;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantReply {
    pub title: String,
    pub message: String,
    pub code: Option<String>,
    pub explanation: Vec<String>,
    pub source: String,
}

impl AssistantReply {
    pub fn render_text(&self) -> String {
        let mut out = format!("**{}**\n{}\n", self.title, self.message);
        if let Some(code) = &self.code {
            out.push_str("\n```text\n");
            out.push_str(code);
            out.push_str("\n```\n");
        }
        if !self.explanation.is_empty() {
            out.push_str("\n**What this did / means:**\n");
            for item in &self.explanation {
                out.push_str("- ");
                out.push_str(item);
                out.push('\n');
            }
        }
        out.push_str(&format!("\n_Source: {}_", self.source));
        out
    }
}

pub fn parse_language(input: &str) -> Result<Language> {
    Language::from_alias(input).ok_or_else(|| anyhow!("Unsupported language: {input}"))
}
