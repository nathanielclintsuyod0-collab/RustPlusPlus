use crate::{knowledge, parse_language, AssistantReply};

pub fn answer(input: &str) -> AssistantReply {
    let trimmed = input.trim();
    let lower = trimmed.to_ascii_lowercase();

    if let Some(reply) = knowledge::known_error(trimmed) {
        return reply;
    }

    // Explicit, deterministic request patterns. No generative fallback.
    if let Some(rest) = lower.strip_prefix("template ") {
        if let Ok(lang) = parse_language(rest) {
            return knowledge::template(lang);
        }
    }
    if let Some(rest) = lower.strip_prefix("explain ") {
        if let Ok(lang) = parse_language(rest) {
            return knowledge::explain_language(lang);
        }
    }
    if lower.contains("hello world") || lower.contains("hello, world") {
        for lang in ["cpp", "rust", "python", "javascript", "java", "csharp"] {
            if lower.contains(lang) || (lang == "cpp" && lower.contains("c++")) {
                return knowledge::template(parse_language(lang).unwrap());
            }
        }
    }

    AssistantReply {
        title: "No built-in match found".into(),
        message: "I can only answer using bundled rules, templates, and explanations. Try `template cpp`, `explain rust`, or ask about a supported known error such as `undefined reference` or `segmentation fault`.".into(),
        code: None,
        explanation: vec![
            "No external AI model or API was contacted.".into(),
            "This request is not yet covered by the installed knowledge rules.".into(),
            "A maintainer can add a new deterministic rule or template to the knowledge base.".into(),
        ],
        source: "offline rule matcher".into(),
    }
}
