use crate::types::{Template, Token};

/// Check if a token is a literal.
pub fn is_literal(token: &Token) -> bool {
    matches!(token, Token::Literal(_))
}

/// Check if a token is a tag.
pub fn is_tag(token: &Token) -> bool {
    matches!(token, Token::Tag(_))
}

/// Parse a string into a template.
///
/// This function parses a string containing Mustache-like `{{tag}}` syntax into a `Template`.
pub fn from_text(input: &str) -> Result<Template, String> {
    let bytes = input.as_bytes();
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < bytes.len() {
        let c = bytes[i];
        if c != b'{' && c != b'}' {
            let start = i;
            while i < bytes.len() && bytes[i] != b'{' && bytes[i] != b'}' {
                i += 1;
            }
            tokens.push(Token::Literal(input[start..i].to_string()));
            continue;
        } else if c == b'{' && i + 1 < bytes.len() && bytes[i + 1] == b'{' {
            let mut tag_end = i + 2;
            while tag_end < bytes.len() && bytes[tag_end] != b'{' && bytes[tag_end] != b'}' {
                tag_end += 1;
            }
            if tag_end > i + 2
                && tag_end + 1 < bytes.len()
                && bytes[tag_end] == b'}'
                && bytes[tag_end + 1] == b'}'
            {
                let name = &input[i + 2..tag_end];
                if name.chars().any(char::is_whitespace) {
                    return Err(format!("whitespace in tag at position {i}"));
                }
                tokens.push(Token::Tag(name.to_string()));
                i = tag_end + 2;
                continue;
            }
            return Err(format!("unclosed tag at position {i}"));
        }
        let start = i;
        while i < bytes.len() && (bytes[i] == b'{' || bytes[i] == b'}') {
            i += 1;
        }
        tokens.push(Token::Literal(input[start..i].to_string()));
    }

    Ok(Template { content: tokens })
}