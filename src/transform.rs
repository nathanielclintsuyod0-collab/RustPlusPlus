use crate::{language::Language, AssistantReply};
use anyhow::{bail, Result};
use std::{fs, path::Path};

/// Conservative transformation that adds a language-correct comment header.
/// It never attempts arbitrary code rewriting.
pub fn add_header(source: &str, lang: Language, label: &str) -> Result<(String, AssistantReply)> {
    let clean = label.replace('\r', " ").replace('\n', " ").trim().to_string();
    if clean.is_empty() { bail!("Header text cannot be empty"); }
    let prefix = lang.comment_prefix();
    let header = format!("{prefix} {clean}\n");
    if source.starts_with(&header) {
        bail!("That exact header is already present");
    }
    let updated = format!("{header}{source}");
    let reply = AssistantReply {
        title: "Source header added".into(),
        message: format!("Added a {0} comment header without changing the remaining source text.", lang.display()),
        code: Some(updated.clone()),
        explanation: vec![
            format!("Inserted `{header}` at the beginning of the file."),
            "Preserved the original source text byte-for-byte after the inserted header.".into(),
            "This is a text-level safe transformation, not a full syntax-tree rewrite.".into(),
        ],
        source: "built-in transformation: add-header".into(),
    };
    Ok((updated, reply))
}

pub fn transform_file(path: &Path, lang: Language, label: &str) -> Result<AssistantReply> {
    let source = fs::read_to_string(path)?;
    let (updated, reply) = add_header(&source, lang, label)?;
    let backup = path.with_extension(format!("{}.bak", lang.extension()));
    fs::write(&backup, source)?;
    fs::write(path, updated)?;
    Ok(reply)
}
