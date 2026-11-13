use crate::{language::Language, AssistantReply};

pub fn lightweight_check(source: &str, lang: Language) -> AssistantReply {
    let mut notes = Vec::new();
    if source.trim().is_empty() {
        notes.push("The file is empty.".to_string());
    }
    let opens = source.chars().filter(|c| *c == '{').count();
    let closes = source.chars().filter(|c| *c == '}').count();
    if opens != closes {
        notes.push(format!("Brace count differs: {opens} opening and {closes} closing braces."));
    }
    match lang {
        Language::Python => {
            if source.contains("    ") && source.contains("\t") {
                notes.push("The file appears to mix tabs and spaces; check indentation consistency.".into());
            }
        }
        Language::Cpp | Language::Rust | Language::Java | Language::CSharp | Language::JavaScript => {
            if source.contains("TODO") { notes.push("A TODO marker remains in the source.".into()); }
        }
    }
    if notes.is_empty() {
        notes.push("No issues were found by the lightweight checks.".into());
    }
    notes.push("This is not a real compiler or complete parser. Use the language's compiler/interpreter for authoritative validation.".into());
    AssistantReply {
        title: format!("{} lightweight validation", lang.display()),
        message: "Completed bundled structural checks.".into(),
        code: None,
        explanation: notes,
        source: "local lightweight validator".into(),
    }
}
