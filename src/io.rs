use crate::parser::from_text;
use crate::template::to_text;
use crate::types::{Context, Template};

/// Read a template from a file.
pub fn read_template_file(path: &str) -> Result<Template, String> {
    let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    from_text(&content)
}

/// Write a template to a file.
pub fn write_template_file(path: &str, template: &Template) -> Result<(), String> {
    std::fs::write(path, to_text(template)).map_err(|e| e.to_string())
}

/// Read a context from a JSON file.
///
/// Returns `Ok(Some(context))` if the file was read successfully, or `Ok(None)` if the file could not be parsed.
pub fn read_context_file(path: &str) -> Result<Option<Context>, String> {
    let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    match serde_json::from_str(&content) {
        Ok(context) => Ok(Some(context)),
        Err(_) => Ok(None),
    }
}

/// Write a context to a JSON file.
pub fn write_context_file(path: &str, context: &Context) -> Result<(), String> {
    let content = serde_json::to_string_pretty(context).map_err(|e| e.to_string())?;
    std::fs::write(path, content).map_err(|e| e.to_string())
}

/// Initialize a context file with empty values for all variables.
pub fn init_context_file(path: &str, context: &Context) -> Result<(), String> {
    let flattened = Context {
        variables: context
            .variables
            .keys()
            .map(|name| (name.clone(), String::new()))
            .collect(),
    };
    write_context_file(path, &flattened)
}