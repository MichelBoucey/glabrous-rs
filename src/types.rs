use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Represents a token in a template, either a literal string or a tag.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Token {
    /// A template tag, e.g., `{{name}}`.
    Tag(String),
    /// A literal string in the template.
    Literal(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Tag(name) => write!(f, "{{{{{}}}}}", name),
            Token::Literal(literal) => write!(f, "{}", literal),
        }
    }
}

/// A template containing a sequence of tokens.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Template {
    /// The sequence of tokens in the template.
    pub content: Vec<Token>,
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for token in &self.content {
            write!(f, "{}", token)?;
        }
        Ok(())
    }
}

/// A context containing variables for template processing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Context {
    /// A map of variable names to their values.
    pub variables: HashMap<String, String>,
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for (i, (name, value)) in self.variables.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", name, value)?;
        }
        write!(f, "}}")
    }
}

/// The result of a partial processing operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessResult {
    /// The processing is complete, and the result is a string.
    Final(String),
    /// The processing is partial, and the result is a new template and context.
    Partial { template: Template, context: Context },
}
