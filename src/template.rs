use std::collections::HashMap;

use crate::parser::{is_literal, is_tag};
use crate::types::{Template, Token};

/// Add a new tag to a template by replacing a literal string.
///
/// Returns `Some(Template)` if the replacement was successful, or `None` if the string was not found.
pub fn add_tag(
    template: &Template,
    replacement: &str,
    new_tag_name: &str,
) -> Option<Template> {
    let content: Vec<Token> = template
        .content
        .iter()
        .flat_map(|token| insert_tag(token, replacement, new_tag_name))
        .collect();
    if content.len() > template.content.len() {
        Some(Template { content })
    } else {
        None
    }
}

fn insert_tag(token: &Token, replacement: &str, new_tag_name: &str) -> Vec<Token> {
    match token {
        Token::Tag(_) => vec![token.clone()],
        Token::Literal(literal) => {
            let mut result = Vec::new();
            let mut pieces = literal.split(replacement).peekable();
            while let Some(piece) = pieces.next() {
                if !piece.is_empty() {
                    result.push(Token::Literal(piece.to_string()));
                }
                if pieces.peek().is_some() {
                    result.push(Token::Tag(new_tag_name.to_string()));
                }
            }
            result
        }
    }
}

/// Get the list of tags in the given template.
pub fn tags_of(template: &Template) -> Vec<Token> {
    template
        .content
        .iter()
        .filter(|token| is_tag(token))
        .cloned()
        .collect()
}

/// Rename tags in a template based on a list of renames.
pub fn tags_rename(renames: &[(&str, &str)], template: &Template) -> Template {
    let renames: HashMap<&str, &str> = renames.iter().copied().collect();
    Template {
        content: template
            .content
            .iter()
            .map(|token| match token {
                Token::Tag(name) => match renames.get(name.as_str()) {
                    Some(new_name) => Token::Tag(new_name.to_string()),
                    None => token.clone(),
                },
                Token::Literal(_) => token.clone(),
            })
            .collect(),
    }
}

/// Check if a template has no more tags inside.
///
/// Returns `true` if the template is final (contains only literals).
pub fn is_final(template: &Template) -> bool {
    template.content.iter().all(is_literal)
}

/// Output the content of the given template as it is, with its tags.
pub fn to_text(template: &Template) -> String {
    template
        .content
        .iter()
        .map(|token| match token {
            Token::Literal(literal) => literal.clone(),
            Token::Tag(name) => format!("{{{{{}}}}}", name),
        })
        .collect()
}

/// Output the content of the given template with all its tags removed.
pub fn to_final_text(template: &Template) -> String {
    template.content.iter().fold(String::new(), |mut acc, token| {
        match token {
            Token::Literal(literal) => acc.push_str(literal),
            Token::Tag(_) => {}
        }
        acc
    })
}

/// Optimize a template content after (many) partial processing rewrites.
///
/// This function merges adjacent literal tokens.
pub fn compress(template: &Template) -> Template {
    let mut content: Vec<Token> = Vec::new();
    for token in &template.content {
        match (content.last_mut(), token) {
            (Some(Token::Literal(last)), Token::Literal(next)) => {
                last.push_str(next);
            }
            _ => content.push(token.clone()),
        }
    }
    Template { content }
}

/// Insert a template into another template by replacing a tag.
///
/// Returns `Some(Template)` if the tag was found and replaced, or `None` otherwise.
pub fn insert_template(
    template: &Template,
    tag: &Token,
    inserted: &Template,
) -> Option<Template> {
    if matches!(tag, Token::Literal(_)) || !template.content.iter().any(|t| t == tag) {
        return None;
    }
    let content = template
        .content
        .iter()
        .flat_map(|token| {
            if token == tag {
                inserted.content.clone()
            } else {
                vec![token.clone()]
            }
        })
        .collect();
    Some(Template { content })
}

fn is_subsequence(needle: &[&Token], haystack: &[Token]) -> bool {
    let mut iter = haystack.iter();
    needle.iter().all(|tag| iter.any(|candidate| *tag == candidate))
}

/// Insert many templates into a template by replacing tags.
///
/// Returns `Some(Template)` if all tags were found and replaced in the exact given order, or `None` otherwise.
pub fn insert_many_templates(template: &Template, pairs: &[(&Token, &Template)]) -> Option<Template> {
    let tag_names: Vec<&Token> = pairs.iter().map(|(tag, _)| *tag).collect();
    let tags = tags_of(template);
    if !is_subsequence(&tag_names, &tags) {
        return None;
    }
    let content = template
        .content
        .iter()
        .flat_map(|token| {
            match pairs.iter().rev().find(|(tag, _)| *tag == token) {
                Some((_, inserted)) => inserted.content.clone(),
                None => vec![token.clone()],
            }
        })
        .collect();
    Some(Template { content })
}