# Glabrous: A template DSL library for Rust [![CI](https://github.com/MichelBoucey/glabrous-rs/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/MichelBoucey/glabrous-rs/actions/workflows/ci.yml)

## 1. Goal

`Glabrous` is a minimalistic Mustache-like syntax, truly logic-less, pure String template DSL library with only one tag type, {{name}}, to be easy and fast.

`Glabrous` is a Rust implementation of [the Haskell library Glabrous](https://hackage.haskell.org/package/glabrous).

## 2. Some features

- A `Template` can be started from a simple `String` or file and tailored to the needs.

- A `Template` can be written to and read from a file.

- A `Template` can be partially filled with a partial `Context` to get a new `Template`.

- One or many `Template`(s) can be inserted at once in another one to get a new `Template`.

- A `Context` can be written to and read from a file through `JSON` serialization.

- See [the full documentation](https://docs.rs/glabrous/latest/glabrous/).

## 3. Basic usage

```rust
use glabrous::{from_text, init_context, set_variables, process};

let template = from_text("Hello, {{name}}!").unwrap();
let context = set_variables(&[("name", "World")], &init_context());
let result = process(&template, &context);
assert_eq!(result, "Hello, World!");
```

## 4. Example

One can find an example demonstrating the full library API in `examples/` which can be run with `cargo run --example pseudo-latin`.
