use std::collections::HashMap;

use crate::template::tags_of;
use crate::types::{Context, Template, Token};

/// Build an empty context.
pub fn init_context() -> Context {
    Context {
        variables: HashMap::new(),
    }
}

/// Build a context from a list of tag-value pairs.
pub fn from_list(pairs: &[(&str, &str)]) -> Context {
    Context {
        variables: pairs
            .iter()
            .map(|(name, value)| (name.to_string(), value.to_string()))
            .collect(),
    }
}

/// Build an unset context from a list of tag names.
///
/// All variables will have empty values.
pub fn from_tags_list(tags: &[&str]) -> Context {
    let pairs: Vec<(&str, &str)> = tags.iter().map(|tag| (*tag, "")).collect();
    from_list(&pairs)
}

/// Build an unset ad hoc context from the given template.
pub fn from_template(template: &Template) -> Context {
    Context {
        variables: tags_of(template)
            .into_iter()
            .map(|token| match token {
                Token::Tag(name) => (name, String::new()),
                Token::Literal(_) => {
                    unreachable!("tags_of only returns Tag tokens")
                }
            })
            .collect(),
    }
}

/// Populate with variables and/or update variables in the given context.
pub fn set_variables(pairs: &[(&str, &str)], context: &Context) -> Context {
    let mut variables = context.variables.clone();
    for (name, value) in pairs {
        variables.insert(name.to_string(), value.to_string());
    }
    Context { variables }
}

/// Delete variables from a context by their names.
pub fn delete_variables(names: &[&str], context: &Context) -> Context {
    let mut variables = context.variables.clone();
    for name in names {
        variables.remove(*name);
    }
    Context { variables }
}

/// Get the list of the given context's variables.
pub fn variables_of(context: &Context) -> Vec<String> {
    context.variables.keys().cloned().collect()
}

/// Returns `true` if all variables of the given context are not empty.
pub fn is_set(context: &Context) -> bool {
    context.variables.values().all(|value| !value.is_empty())
}

/// Build `Some` a (sub)context made of unset variables of the given context, or `None`.
pub fn unset_context(context: &Context) -> Option<Context> {
    let variables: HashMap<String, String> = context
        .variables
        .iter()
        .filter(|(_, value)| value.is_empty())
        .map(|(name, value)| (name.clone(), value.clone()))
        .collect();
    if variables.is_empty() {
        None
    } else {
        Some(Context { variables })
    }
}

/// Join two contexts if they don't share variable names.
///
/// Returns `Ok` with the merged context if successful, or `Err` with the intersection context if there are conflicts.
pub fn join(first: &Context, second: &Context) -> Result<Context, Context> {
    let intersection: HashMap<String, String> = first
        .variables
        .iter()
        .filter(|(name, _)| second.variables.contains_key(*name))
        .map(|(name, value)| (name.clone(), value.clone()))
        .collect();
    if intersection.is_empty() {
        let mut variables = first.variables.clone();
        variables.extend(second.variables.clone());
        Ok(Context { variables })
    } else {
        Err(Context {
            variables: intersection,
        })
    }
}

pub(crate) fn to_text_with_context(
    tag_default: impl Fn(&str) -> String,
    context: &Context,
    tokens: &[Token],
) -> String {
    tokens
        .iter()
        .map(|token| match token {
            Token::Literal(literal) => literal.clone(),
            Token::Tag(name) => context
                .variables
                .get(name)
                .cloned()
                .unwrap_or_else(|| tag_default(name)),
        })
        .collect()
}