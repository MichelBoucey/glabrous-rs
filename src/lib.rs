//! # Glabrous
//!
//! A minimalistic Mustache-like syntax, truly logic-less, pure `String` template library.
//!
//! This is a Rust implementation of [the haskell library Glabrous](https://hackage.haskell.org/package/glabrous).
//!
//! ## Synopsis
//!
//! This crate provides a simple template engine using the `{{name}}` tag syntax only.
//!
//! - [`Template`]: The core template type.
//! - [`Context`]: A collection of variables for template processing.
//! - [`process()`]: The main function to render a template with a context.
//!
//! ## Usage
//!
//! ```rust
//! use glabrous::{from_text, init_context, set_variables, process};
//!
//! let template = from_text("Hello, {{name}}!").unwrap();
//! let context = set_variables(&[("name", "World")], &init_context());
//! let result = process(&template, &context);
//! assert_eq!(result, "Hello, World!");
//! ```

pub mod context;
pub mod io;
pub mod parser;
pub mod process;
pub mod template;
pub mod types;

pub use types::{Context, ProcessResult, Template, Token};
pub use parser::{from_text, is_literal, is_tag};
pub use context::{
    delete_variables, from_list, from_tags_list, from_template, init_context, is_set, join,
    set_variables, unset_context, variables_of,
};
pub use template::{
    add_tag, compress, insert_many_templates, insert_template, is_final, tags_of, tags_rename,
    to_final_text, to_text,
};
pub use process::{partial_process, partial_process_result, process, process_with_default};
pub use io::{
    init_context_file, read_context_file, read_template_file, write_context_file,
    write_template_file,
};
