use crate::context::{from_tags_list, to_text_with_context};
use crate::types::{Context, ProcessResult, Template, Token};

/// Process a template with a context, discarding tags not present in the context.
pub fn process(template: &Template, context: &Context) -> String {
    process_with_default("", template, context)
}

/// Process a template with a context, replacing missing tags with a default value.
pub fn process_with_default(default: &str, template: &Template, context: &Context) -> String {
    to_text_with_context(|_| default.to_string(), context, &template.content)
}

/// Process a (sub)context present in the given template, leaving other tags untouched.
///
/// Returns a new template with the processed tags replaced by their values.
pub fn partial_process(template: &Template, context: &Context) -> Template {
    Template {
        content: template
            .content
            .iter()
            .map(|token| match token {
                Token::Tag(name) => context
                    .variables
                    .get(name)
                    .map(|value| Token::Literal(value.clone()))
                    .unwrap_or_else(|| token.clone()),
                Token::Literal(_) => token.clone(),
            })
            .collect(),
    }
}

/// Process a (sub)context present in the given template, returning either a final string or a new template with its unset context.
pub fn partial_process_result(template: &Template, context: &Context) -> ProcessResult {
    let mut processed: Vec<Token> = Vec::new();
    let mut tags: Vec<String> = Vec::new();
    for token in &template.content {
        match token {
            Token::Tag(name) => match context.variables.get(name) {
                Some(value) => processed.push(Token::Literal(value.clone())),
                None => {
                    processed.push(token.clone());
                    tags.push(name.clone());
                }
            },
            Token::Literal(_) => processed.push(token.clone()),
        }
    }
    if tags.is_empty() {
        let text = to_text_with_context(|_| String::new(), context, &processed);
        ProcessResult::Final(text)
    } else {
        let names: Vec<&str> = tags.iter().map(String::as_str).collect();
        let context = from_tags_list(&names);
        ProcessResult::Partial {
            template: Template { content: processed },
            context,
        }
    }
}