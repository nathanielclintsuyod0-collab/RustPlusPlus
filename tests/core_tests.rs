use rust_plus_plus::{engine, language::Language, knowledge, transform, validate};

#[test]
fn all_language_templates_exist() {
    for name in ["cpp", "rust", "python", "javascript", "java", "csharp", "lua"] {
        let lang = rust_plus_plus::parse_language(name).unwrap();
        assert!(knowledge::template(lang).code.is_some());
    }
}

#[test]
fn add_header_preserves_original_source() {
    let original = "int main() { return 0; }\n";
    let (updated, reply) = transform::add_header(original, Language::Cpp, "Demo").unwrap();
    assert!(updated.ends_with(original));
    assert!(reply.message.contains("without changing"));
}

#[test]
fn unknown_question_does_not_invent_answer() {
    let reply = engine::answer("invent a brand new distributed database");
    assert!(reply.title.contains("No built-in match"));
}

#[test]
fn validator_reports_brace_mismatch() {
    let reply = validate::lightweight_check("int main() {", Language::Cpp);
    assert!(reply.explanation.iter().any(|x| x.contains("Brace count differs")));
}
