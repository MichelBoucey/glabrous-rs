use glabrous::{
    add_tag, compress, delete_variables, from_list, from_tags_list, from_template, from_text,
    init_context, init_context_file, insert_many_templates, insert_template, is_final, is_literal,
    is_set, is_tag, join, partial_process, partial_process_result, process, process_with_default,
    read_context_file, read_template_file, set_variables, tags_of, tags_rename, to_final_text,
    to_text, unset_context, variables_of, write_context_file, write_template_file, Context,
    ProcessResult, Template, Token,
};
use serde_json;

#[test]
fn from_text_parses_simple_template() {
    let template = from_text("Hello {{name}}!").expect("parse should succeed");
    assert_eq!(
        template.content,
        vec![
            Token::Literal("Hello ".to_string()),
            Token::Tag("name".to_string()),
            Token::Literal("!".to_string()),
        ]
    );
}

#[test]
fn from_text_parses_multiple_tags() {
    let template = from_text("{{a}} and {{b}}").expect("parse should succeed");
    assert_eq!(
        template.content,
        vec![
            Token::Tag("a".to_string()),
            Token::Literal(" and ".to_string()),
            Token::Tag("b".to_string()),
        ]
    );
}

#[test]
fn from_text_parses_adjacent_tags() {
    let template = from_text("{{a}}{{b}}").expect("parse should succeed");
    assert_eq!(
        template.content,
        vec![
            Token::Tag("a".to_string()),
            Token::Tag("b".to_string()),
        ]
    );
}

#[test]
fn from_text_parses_text_without_tags() {
    let template = from_text("Hello world").expect("parse should succeed");
    assert_eq!(template.content, vec![Token::Literal("Hello world".to_string())]);
}

#[test]
fn from_text_parses_empty_string() {
    let template = from_text("").expect("parse should succeed");
    assert!(template.content.is_empty());
}

#[test]
fn from_text_matches_haskell_readme_example() {
    let template =
        from_text("Glabrous templates use only the simplest Mustache tag: {{name}}.")
            .expect("parse should succeed");
    assert_eq!(
        template.content,
        vec![
            Token::Literal("Glabrous templates use only the simplest Mustache tag: ".to_string()),
            Token::Tag("name".to_string()),
            Token::Literal(".".to_string()),
        ]
    );
}

#[test]
fn from_text_errors_on_unclosed_tag() {
    assert!(from_text("Hello {{name").is_err());
}

#[test]
fn from_text_errors_on_empty_tag() {
    assert!(from_text("{{}}").is_err());
}

#[test]
fn from_text_errors_on_tag_with_whitespace() {
    assert!(from_text("{{ name }}").is_err());
    assert!(from_text("{{name }}").is_err());
    assert!(from_text("{{ name}}").is_err());
    assert!(from_text("{{a b}}").is_err());
}

#[test]
fn from_text_errors_on_unmatched_braces_run() {
    assert!(from_text("a{{{b").is_err());
}

#[test]
fn is_literal_reports_literal_tokens() {
    assert!(is_literal(&Token::Literal("x".to_string())));
    assert!(!is_literal(&Token::Tag("x".to_string())));
}

#[test]
fn is_tag_reports_tag_tokens() {
    assert!(is_tag(&Token::Tag("x".to_string())));
    assert!(!is_tag(&Token::Literal("x".to_string())));
}

#[test]
fn read_template_file_reads_from_disk() {
    let path = std::env::temp_dir().join("glabrous_test_template.txt");
    std::fs::write(&path, "Hi {{user}}!").expect("write should succeed");

    let template = read_template_file(path.to_str().unwrap()).expect("read should succeed");
    let expected = Template {
        content: vec![
            Token::Literal("Hi ".to_string()),
            Token::Tag("user".to_string()),
            Token::Literal("!".to_string()),
        ],
    };
    assert_eq!(template, expected);

    let _ = std::fs::remove_file(path);
}

#[test]
fn read_template_file_fails_on_missing_file() {
    let path = std::env::temp_dir().join("glabrous_does_not_exist.txt");
    assert!(read_template_file(path.to_str().unwrap()).is_err());
}

#[test]
fn add_tag_replaces_text_with_tag() {
    let template = from_text("Hello world").expect("parse should succeed");
    let result = add_tag(&template, "world", "name").expect("tag should be added");
    assert_eq!(
        result.content,
        vec![
            Token::Literal("Hello ".to_string()),
            Token::Tag("name".to_string()),
        ]
    );
}

#[test]
fn add_tag_replaces_multiple_occurrences() {
    let template = from_text("a-b-a").expect("parse should succeed");
    let result = add_tag(&template, "-", "x").expect("tag should be added");
    assert_eq!(
        result.content,
        vec![
            Token::Literal("a".to_string()),
            Token::Tag("x".to_string()),
            Token::Literal("b".to_string()),
            Token::Tag("x".to_string()),
            Token::Literal("a".to_string()),
        ]
    );
}

#[test]
fn add_tag_keeps_existing_tags() {
    let template = from_text("{{keep}} text").expect("parse should succeed");
    let result = add_tag(&template, "text", "new").expect("tag should be added");
    assert_eq!(
        result.content,
        vec![
            Token::Tag("keep".to_string()),
            Token::Literal(" ".to_string()),
            Token::Tag("new".to_string()),
        ]
    );
}

#[test]
fn add_tag_returns_none_when_text_absent() {
    let template = from_text("Hello world").expect("parse should succeed");
    assert_eq!(add_tag(&template, "xyz", "name"), None);
}

#[test]
fn add_tag_returns_none_when_replacement_is_whole_literal() {
    let template = from_text("world").expect("parse should succeed");
    assert_eq!(add_tag(&template, "world", "name"), None);
}

#[test]
fn tags_of_returns_tags_in_document_order() {
    let template = from_text("a {{ipsum}} b {{tortor}} c {{lectus}}").expect("parse should succeed");
    assert_eq!(
        tags_of(&template),
        vec![
            Token::Tag("ipsum".to_string()),
            Token::Tag("tortor".to_string()),
            Token::Tag("lectus".to_string()),
        ]
    );
}

#[test]
fn tags_of_returns_empty_for_tagless_template() {
    let template = from_text("plain text").expect("parse should succeed");
    assert!(tags_of(&template).is_empty());
}

#[test]
fn tags_of_ignores_single_braces() {
    let template = from_text("a {congue} b").expect("parse should succeed");
    assert!(tags_of(&template).is_empty());
}

#[test]
fn tags_rename_renames_matching_tags() {
    let template = from_text("{{a}} and {{b}}").expect("parse should succeed");
    let result = tags_rename(&[("a", "x")], &template);
    assert_eq!(
        result.content,
        vec![
            Token::Tag("x".to_string()),
            Token::Literal(" and ".to_string()),
            Token::Tag("b".to_string()),
        ]
    );
}

#[test]
fn tags_rename_renames_all_matching_tags() {
    let template = from_text("{{a}} and {{b}}").expect("parse should succeed");
    let result = tags_rename(&[("a", "x"), ("b", "y")], &template);
    assert_eq!(
        result.content,
        vec![
            Token::Tag("x".to_string()),
            Token::Literal(" and ".to_string()),
            Token::Tag("y".to_string()),
        ]
    );
}

#[test]
fn tags_rename_keeps_unmatched_tags_and_literals() {
    let template = from_text("hello {{a}}").expect("parse should succeed");
    let result = tags_rename(&[("hello", "x")], &template);
    assert_eq!(
        result.content,
        vec![
            Token::Literal("hello ".to_string()),
            Token::Tag("a".to_string()),
        ]
    );
}

#[test]
fn tags_rename_is_identity_without_renames() {
    let template = from_text("{{a}} and {{b}}").expect("parse should succeed");
    let result = tags_rename(&[], &template);
    assert_eq!(result, template);
}

#[test]
fn is_final_true_when_no_tags() {
    let template = from_text("plain text").expect("parse should succeed");
    assert!(is_final(&template));
}

#[test]
fn is_final_true_for_empty_template() {
    let template = from_text("").expect("parse should succeed");
    assert!(is_final(&template));
}

#[test]
fn is_final_false_when_tags_present() {
    let template = from_text("Hello {{name}}!").expect("parse should succeed");
    assert!(!is_final(&template));
}

#[test]
fn to_text_matches_haskell_readme_example() {
    let template =
        from_text("Glabrous templates use only the simplest Mustache tag: {{name}}.")
            .expect("parse should succeed");
    assert_eq!(
        to_text(&template),
        "Glabrous templates use only the simplest Mustache tag: {{name}}."
    );
}

#[test]
fn to_text_renders_template_as_it_is() {
    let template = from_text("Hello {{name}}!").expect("parse should succeed");
    assert_eq!(to_text(&template), "Hello {{name}}!");
}

#[test]
fn to_text_renders_tags_in_document_order() {
    let template = from_text("{{a}} and {{b}}").expect("parse should succeed");
    assert_eq!(to_text(&template), "{{a}} and {{b}}");
}

#[test]
fn to_text_keeps_text_without_tags() {
    let template = from_text("plain text").expect("parse should succeed");
    assert_eq!(to_text(&template), "plain text");
}

#[test]
fn to_final_text_removes_tags() {
    let template = from_text("Hello {{name}}!").expect("parse should succeed");
    assert_eq!(to_final_text(&template), "Hello !");
}

#[test]
fn to_final_text_removes_all_tags_keeping_literals() {
    let template = from_text("{{a}} and {{b}}").expect("parse should succeed");
    assert_eq!(to_final_text(&template), " and ");
}

#[test]
fn to_final_text_keeps_text_without_tags() {
    let template = from_text("plain text").expect("parse should succeed");
    assert_eq!(to_final_text(&template), "plain text");
}

#[test]
fn to_final_text_returns_empty_string_for_empty_template() {
    let template = from_text("").expect("parse should succeed");
    assert_eq!(to_final_text(&template), "");
}

#[test]
fn to_text_roundtrips_through_read_template_file() {
    let template = from_text("Hi {{user}} and {{theme}}!").expect("parse should succeed");
    assert_eq!(to_text(&template), "Hi {{user}} and {{theme}}!");
}

#[test]
fn init_context_builds_empty_context() {
    let context = init_context();
    assert!(context.variables.is_empty());
}

#[test]
fn init_context_is_empty_context() {
    let context = init_context();
    let expected = Context {
        variables: std::collections::HashMap::new(),
    };
    assert_eq!(context, expected);
}

#[test]
fn from_list_builds_context_from_pairs() {
    let context = from_list(&[("tag", "replacement"), ("etc.", "...")]);
    assert_eq!(context.variables.get("tag"), Some(&"replacement".to_string()));
    assert_eq!(context.variables.get("etc."), Some(&"...".to_string()));
    assert_eq!(context.variables.len(), 2);
}

#[test]
fn from_tags_list_builds_unset_context() {
    let context = from_tags_list(&["tag", "etc."]);
    assert_eq!(context.variables.get("tag"), Some(&"".to_string()));
    assert_eq!(context.variables.get("etc."), Some(&"".to_string()));
    assert_eq!(context.variables.len(), 2);
}

#[test]
fn from_template_builds_unset_context_from_tags() {
    let template = from_text("Lorem {{ipsum}} dolor {{lectus}}").expect("parse should succeed");
    let context = from_template(&template);
    assert_eq!(context.variables.get("ipsum"), Some(&"".to_string()));
    assert_eq!(context.variables.get("lectus"), Some(&"".to_string()));
    assert_eq!(context.variables.len(), 2);
}

#[test]
fn from_template_builds_empty_context_for_tagless_template() {
    let template = from_text("plain text").expect("parse should succeed");
    let context = from_template(&template);
    assert!(context.variables.is_empty());
}

#[test]
fn set_variables_populates_and_updates_context() {
    let context = init_context();
    let context = set_variables(&[("tag", "replacement"), ("theme", "Haskell")], &context);
    assert_eq!(context.variables.get("tag"), Some(&"replacement".to_string()));
    assert_eq!(context.variables.get("theme"), Some(&"Haskell".to_string()));
    assert_eq!(context.variables.len(), 2);
}

#[test]
fn set_variables_updates_existing_values() {
    let context = from_list(&[("a", "1")]);
    let context = set_variables(&[("a", "2"), ("b", "3")], &context);
    assert_eq!(context.variables.get("a"), Some(&"2".to_string()));
    assert_eq!(context.variables.get("b"), Some(&"3".to_string()));
    assert_eq!(context.variables.len(), 2);
}

#[test]
fn set_variables_from_template_leaves_rest_unset() {
    let template = from_text("{{name}} and {{theme}}").expect("parse should succeed");
    let context = set_variables(&[("theme", "Haskell")], &from_template(&template));
    assert_eq!(context.variables.get("name"), Some(&"".to_string()));
    assert_eq!(context.variables.get("theme"), Some(&"Haskell".to_string()));
}

#[test]
fn delete_variables_removes_named_variables() {
    let context = from_list(&[("a", "1"), ("b", "2"), ("c", "3")]);
    let context = delete_variables(&["a", "c"], &context);
    assert_eq!(context.variables.get("b"), Some(&"2".to_string()));
    assert_eq!(context.variables.len(), 1);
}

#[test]
fn variables_of_returns_variable_names() {
    let context = from_list(&[("a", "1"), ("b", "2")]);
    let mut names = variables_of(&context);
    names.sort();
    assert_eq!(names, vec!["a".to_string(), "b".to_string()]);
}

#[test]
fn is_set_true_when_all_variables_set() {
    let context = from_list(&[("a", "x"), ("b", "y")]);
    assert!(is_set(&context));
}

#[test]
fn is_set_true_for_empty_context() {
    assert!(is_set(&init_context()));
}

#[test]
fn is_set_false_when_any_variable_unset() {
    let context = from_list(&[("a", ""), ("b", "y")]);
    assert!(!is_set(&context));
}

#[test]
fn unset_context_returns_subset_of_unset_variables() {
    let context = from_list(&[("name", ""), ("theme", "Haskell")]);
    let unset = unset_context(&context).expect("unset context should exist");
    assert_eq!(unset.variables.get("name"), Some(&"".to_string()));
    assert_eq!(unset.variables.len(), 1);
}

#[test]
fn unset_context_returns_none_when_all_variables_set() {
    let context = from_list(&[("name", "x"), ("theme", "Haskell")]);
    assert_eq!(unset_context(&context), None);
}

#[test]
fn join_returns_union_when_disjoint() {
    let joined = join(&from_list(&[("tag", "replacement")]), &from_list(&[("theme", "Haskell")]))
        .expect("disjoint contexts should join");
    assert_eq!(joined.variables.get("tag"), Some(&"replacement".to_string()));
    assert_eq!(joined.variables.get("theme"), Some(&"Haskell".to_string()));
    assert_eq!(joined.variables.len(), 2);
}

#[test]
fn join_returns_intersection_when_shared() {
    let intersection = join(
        &from_list(&[("tag", "replacement")]),
        &from_list(&[("tag", "other"), ("theme", "Haskell")]),
    )
    .expect_err("shared variables should intersect");
    assert_eq!(intersection.variables.get("tag"), Some(&"replacement".to_string()));
    assert_eq!(intersection.variables.len(), 1);
}

#[test]
fn compress_merges_adjacent_literals() {
    let template = from_text("{{a}}{{b}}x").expect("parse should succeed");
    let partial = partial_process(&template, &from_list(&[("a", "1"), ("b", "2")]));
    assert_eq!(
        partial.content,
        vec![
            Token::Literal("1".to_string()),
            Token::Literal("2".to_string()),
            Token::Literal("x".to_string()),
        ]
    );
    let compressed = compress(&partial);
    assert_eq!(compressed.content, vec![Token::Literal("12x".to_string())]);
}

#[test]
fn insert_template_inserts_at_tag() {
    let template = from_text("a{{x}}d").expect("parse should succeed");
    let inserted = from_text(" b {{y}} {{z}} c ").expect("parse should succeed");
    let result = insert_template(&template, &Token::Tag("x".to_string()), &inserted)
        .expect("tag should be present");
    assert_eq!(
        result.content,
        vec![
            Token::Literal("a".to_string()),
            Token::Literal(" b ".to_string()),
            Token::Tag("y".to_string()),
            Token::Literal(" ".to_string()),
            Token::Tag("z".to_string()),
            Token::Literal(" c ".to_string()),
            Token::Literal("d".to_string()),
        ]
    );
}

#[test]
fn insert_template_returns_none_for_missing_tag() {
    let template = from_text("a{{x}}b").expect("parse should succeed");
    let inserted = from_text("HI").expect("parse should succeed");
    assert_eq!(
        insert_template(&template, &Token::Tag("no".to_string()), &inserted),
        None
    );
}

#[test]
fn insert_template_returns_none_for_literal_token() {
    let template = from_text("a{{x}}b").expect("parse should succeed");
    let inserted = from_text("HI").expect("parse should succeed");
    assert_eq!(
        insert_template(&template, &Token::Literal("x".to_string()), &inserted),
        None
    );
}

#[test]
fn insert_many_templates_inserts_in_order() {
    let template = from_text("A{{x}}B{{y}}C").expect("parse should succeed");
    let inserted_x = from_text("1").expect("parse should succeed");
    let inserted_y = from_text("2").expect("parse should succeed");
    let result = insert_many_templates(
        &template,
        &[
            (&Token::Tag("x".to_string()), &inserted_x),
            (&Token::Tag("y".to_string()), &inserted_y),
        ],
    )
    .expect("tags should be present in order");
    assert_eq!(
        result.content,
        vec![
            Token::Literal("A".to_string()),
            Token::Literal("1".to_string()),
            Token::Literal("B".to_string()),
            Token::Literal("2".to_string()),
            Token::Literal("C".to_string()),
        ]
    );
}

#[test]
fn insert_many_templates_returns_none_for_missing_tag() {
    let template = from_text("A{{x}}B").expect("parse should succeed");
    let inserted = from_text("1").expect("parse should succeed");
    assert_eq!(
        insert_many_templates(
            &template,
            &[(&Token::Tag("no".to_string()), &inserted)],
        ),
        None
    );
}

#[test]
fn insert_many_templates_returns_none_when_out_of_order() {
    let template = from_text("A{{x}}B{{y}}C").expect("parse should succeed");
    let inserted_x = from_text("1").expect("parse should succeed");
    let inserted_y = from_text("2").expect("parse should succeed");
    assert_eq!(
        insert_many_templates(
            &template,
            &[
                (&Token::Tag("y".to_string()), &inserted_y),
                (&Token::Tag("x".to_string()), &inserted_x),
            ],
        ),
        None
    );
}

#[test]
fn process_replaces_tags_from_context() {
    let template = from_text("Hello {{name}}!").expect("parse should succeed");
    let context = from_list(&[("name", "World")]);
    assert_eq!(process(&template, &context), "Hello World!");
}

#[test]
fn process_removes_unset_tags() {
    let template = from_text("Hello {{name}}!").expect("parse should succeed");
    let context = init_context();
    assert_eq!(process(&template, &context), "Hello !");
}

#[test]
fn process_with_default_replaces_unset_tags() {
    let template = from_text("Hello {{name}}!").expect("parse should succeed");
    let context = init_context();
    assert_eq!(process_with_default("X", &template, &context), "Hello X!");
}

#[test]
fn partial_process_replaces_only_present_tags() {
    let template = from_text("{{a}} X {{b}}").expect("parse should succeed");
    let context = from_list(&[("a", "1")]);
    let partial = partial_process(&template, &context);
    assert_eq!(
        partial.content,
        vec![
            Token::Literal("1".to_string()),
            Token::Literal(" X ".to_string()),
            Token::Tag("b".to_string()),
        ]
    );
}

#[test]
fn partial_process_result_returns_final_when_set() {
    let template = from_text("{{a}} X {{b}}").expect("parse should succeed");
    let context = from_list(&[("a", "1"), ("b", "2")]);
    assert_eq!(
        partial_process_result(&template, &context),
        ProcessResult::Final("1 X 2".to_string())
    );
}

#[test]
fn partial_process_result_returns_partial_with_unset_tags() {
    let template = from_text("{{a}} X {{b}}").expect("parse should succeed");
    let context = from_list(&[("a", "1")]);
    let result = partial_process_result(&template, &context);
    match result {
        ProcessResult::Partial { template, context } => {
            assert_eq!(
                template.content,
                vec![
                    Token::Literal("1".to_string()),
                    Token::Literal(" X ".to_string()),
                    Token::Tag("b".to_string()),
                ]
            );
            assert_eq!(context.variables.get("b"), Some(&"".to_string()));
            assert_eq!(context.variables.len(), 1);
        }
        ProcessResult::Final(_) => panic!("expected Partial"),
    }
}

#[test]
fn write_template_file_writes_template_as_is() {
    let path = std::env::temp_dir().join("glabrous_write_template.txt");
    let template = from_text("Hello {{name}}!").expect("parse should succeed");
    write_template_file(path.to_str().unwrap(), &template).expect("write should succeed");
    let read_back = read_template_file(path.to_str().unwrap()).expect("read should succeed");
    assert_eq!(read_back, template);
    let _ = std::fs::remove_file(path);
}

#[test]
fn write_context_file_and_read_context_file_roundtrip() {
    let path = std::env::temp_dir().join("glabrous_context.json");
    let context = from_list(&[("tag", "replacement"), ("etc.", "...")]);
    write_context_file(path.to_str().unwrap(), &context).expect("write should succeed");
    let read_back = read_context_file(path.to_str().unwrap())
        .expect("read should succeed")
        .expect("context should parse");
    assert_eq!(read_back, context);
    let _ = std::fs::remove_file(path);
}

#[test]
fn init_context_file_writes_empty_values() {
    let path = std::env::temp_dir().join("glabrous_context_init.json");
    let context = from_list(&[("tag", "replacement"), ("etc.", "...")]);
    init_context_file(path.to_str().unwrap(), &context).expect("write should succeed");
    let read_back = read_context_file(path.to_str().unwrap())
        .expect("read should succeed")
        .expect("context should parse");
    assert_eq!(read_back.variables.get("tag"), Some(&"".to_string()));
    assert_eq!(read_back.variables.get("etc."), Some(&"".to_string()));
    assert_eq!(read_back.variables.len(), 2);
    let _ = std::fs::remove_file(path);
}

#[test]
fn read_context_file_returns_none_for_invalid_json() {
    let path = std::env::temp_dir().join("glabrous_invalid.json");
    std::fs::write(&path, "not json").expect("write should succeed");
    assert_eq!(
        read_context_file(path.to_str().unwrap()).expect("read should succeed"),
        None
    );
    let _ = std::fs::remove_file(path);
}

#[test]
fn read_context_file_returns_none_for_non_string_values() {
    let path = std::env::temp_dir().join("glabrous_non_string.json");
    std::fs::write(&path, r#"{"a": 5}"#).expect("write should succeed");
    assert_eq!(
        read_context_file(path.to_str().unwrap()).expect("read should succeed"),
        None
    );
    let _ = std::fs::remove_file(path);
}

#[test]
fn read_context_file_errors_on_missing_file() {
    let path = std::env::temp_dir().join("glabrous_missing.json");
    assert!(read_context_file(path.to_str().unwrap()).is_err());
}

#[test]
fn display_of_tag_matches_mustache_syntax() {
    let tag = Token::Tag("name".to_string());
    assert_eq!(tag.to_string(), "{{name}}");
}

#[test]
fn display_of_literal_renders_as_is() {
    let literal = Token::Literal("hello".to_string());
    assert_eq!(literal.to_string(), "hello");
}

#[test]
fn display_of_template_renders_content_as_is() {
    let template = from_text("Hello, {{name}}!").unwrap();
    assert_eq!(template.to_string(), "Hello, {{name}}!");
}

#[test]
fn display_of_context_lists_variables() {
    let context = from_list(&[("a", "1")]);
    assert_eq!(context.to_string(), "{a: 1}");
}

#[test]
fn display_of_context_separates_variables() {
    let context = from_list(&[("a", "1"), ("b", "2")]);
    let displayed = context.to_string();
    assert!(displayed.starts_with('{') && displayed.ends_with('}'));
    assert!(displayed.contains("a: 1") && displayed.contains("b: 2"));
}

#[test]
fn token_tag_serializes_to_json() {
    let token = Token::Tag("name".to_string());
    let json = serde_json::to_string(&token).expect("serialize should succeed");
    assert_eq!(json, r#"{"Tag":"name"}"#);
}

#[test]
fn token_literal_serializes_to_json() {
    let token = Token::Literal("hello".to_string());
    let json = serde_json::to_string(&token).expect("serialize should succeed");
    assert_eq!(json, r#"{"Literal":"hello"}"#);
}

#[test]
fn token_tag_roundtrips_through_json() {
    let token = Token::Tag("theme".to_string());
    let json = serde_json::to_string(&token).expect("serialize should succeed");
    let deserialized: Token = serde_json::from_str(&json).expect("deserialize should succeed");
    assert_eq!(deserialized, token);
}

#[test]
fn token_literal_roundtrips_through_json() {
    let token = Token::Literal("some text".to_string());
    let json = serde_json::to_string(&token).expect("serialize should succeed");
    let deserialized: Token = serde_json::from_str(&json).expect("deserialize should succeed");
    assert_eq!(deserialized, token);
}

#[test]
fn template_roundtrips_through_json() {
    let template = from_text("Hello {{name}}!").expect("parse should succeed");
    let json = serde_json::to_string(&template).expect("serialize should succeed");
    let deserialized: Template = serde_json::from_str(&json).expect("deserialize should succeed");
    assert_eq!(deserialized, template);
}

#[test]
fn template_empty_roundtrips_through_json() {
    let template = from_text("").expect("parse should succeed");
    let json = serde_json::to_string(&template).expect("serialize should succeed");
    let deserialized: Template = serde_json::from_str(&json).expect("deserialize should succeed");
    assert_eq!(deserialized, template);
}

#[test]
fn template_with_multiple_tags_roundtrips_through_json() {
    let template = from_text("{{a}} and {{b}}").expect("parse should succeed");
    let json = serde_json::to_string(&template).expect("serialize should succeed");
    let deserialized: Template = serde_json::from_str(&json).expect("deserialize should succeed");
    assert_eq!(deserialized, template);
}

#[test]
fn template_deserializes_from_literal_json() {
    let json = r#"{"content":[{"Literal":"foo "},{"Tag":"bar"},{"Literal":" baz"}]}"#;
    let template: Template = serde_json::from_str(json).expect("deserialize should succeed");
    assert_eq!(
        template.content,
        vec![
            Token::Literal("foo ".to_string()),
            Token::Tag("bar".to_string()),
            Token::Literal(" baz".to_string()),
        ]
    );
}
