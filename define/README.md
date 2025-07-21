# Define Macro Implementation

This directory contains the procedural macro implementation for the `alias_macros` crate.

## About Alias Macros

`alias_macros` is a Rust procedural macro crate that allows you to create custom aliases for types, keywords, and complex syntax patterns. Create TypeScript-like interfaces, alias primitive types with descriptive names, or build domain-specific syntax - all with zero runtime cost!

📚 **For complete usage examples, features, and documentation, see the [main README](../README.md).**

## Structure

- `src/lib.rs` - Contains the `define!` procedural macro implementation
- `Cargo.toml` - Crate configuration with proc-macro dependencies

## Implementation Details

The `define!` macro:

1. Parses input in the format `alias = replacement`
2. Creates a `macro_rules!` macro with the alias name
3. Supports two patterns:
   - `()` - Expands to just the replacement
   - `($($tokens:tt)*)` - Expands to replacement followed by the tokens

## Dependencies

- `syn` - For parsing Rust syntax
- `quote` - For generating Rust code
- `proc_macro2` - For token stream manipulation

## Building

```bash
cargo build
```

This crate is used as a dependency by the main project and provides the `define!` macro functionality.
