# Alias Macros 🦀✨

A powerful Rust procedural macro crate that allows you to create aliases for types, keywords, and even complex syntax patterns. Make Rust syntax your own by creating custom aliases for anything!

[![Crates.io](https://img.shields.io/crates/v/alias_macros.svg)](https://crates.io/crates/alias_macros)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)

## 🚀 Features

- **Type Aliases**: Create custom names for existing types (`i64` → `MyInt`)
- **Keyword Aliases**: Alias Rust keywords like `struct` with attributes
- **Struct Aliases**: Create aliases for your custom structs  
- **Zero Runtime Cost**: All expansions happen at compile time
- **Flexible Syntax**: Support for both simple types and complex patterns

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
alias_macros = "0.1.0"
```

Or use cargo to add it:

```bash
cargo add alias_macros
```

## 🔧 Quick Start

```rust
use alias_macros::define;

// Create a type alias
define!(MyInt = i64);

fn main() {
    let number: MyInt!() = 42;
    println!("My number: {}", number);
}
```

## 📚 Usage Examples

### 1. Basic Type Aliases

Perfect for creating more descriptive type names or aliasing complex types:

```rust
use alias_macros::define;

define!(MyInt = i64);
define!(str = String);
define!(char = String);

fn main() {
    let number: MyInt!() = 100;
    let text: char!() = <char!()>::from("I'm a String not a char");
    
    println!("Number: {}", number);
    println!("Text: {}", text);
}
```

### 2. Struct Keyword Aliases

Create TypeScript-like interfaces or custom struct syntax:

```rust
use alias_macros::define;

// Create an "interface" keyword that's actually a struct with Debug derive
define!(interface = #[derive(Debug)]struct);

// Use your new interface keyword
interface!(User {
    id: i64,
    name: String,
});

fn main() {
    let user = User {
        id: 1,
        name: "Alice".to_string(),
    };
    
    println!("User: {:?}", user);
}
```

### 3. Struct Type Aliases

Alias your custom structs for easier instantiation:

```rust
use alias_macros::define;

#[derive(Debug)]
pub struct DataContainer {
    value: String,
}

// Create an alias for your struct
define!(Container = DataContainer);

fn main() {
    // Use the alias to create instances
    let data = Container!({
        value: <String>::from("Hello World")
    });
    
    println!("Data: {:?}", data);
}
```

### 4. Complete Real-World Example

Here's a comprehensive example showing multiple usage patterns:

```rust
use alias_macros::define;

// Type aliases
define!(MyInt = i64);
define!(str = String);
define!(char = String);

// Struct keyword alias
define!(interface = #[derive(Debug)]struct);

// Create a struct using the interface alias
interface!(Nothing {
    num: MyInt!()
});

// Regular struct
#[derive(Debug)]
pub struct something {
    str: str!()
}

// Struct type alias
define!(strignStruct = something);

fn main() {
    // Using type aliases
    let v: MyInt!() = 100;
    let char: char!() = <char!()>::from("I'm a String not a char");

    // Using struct created with interface alias
    let s = Nothing {
        num: 1
    };

    // Using struct type alias
    let a = strignStruct!({
        str: <str!()>::from("Shubh")
    });

    println!("--------------------------------------------------------");
    println!("I'm a struct, but a TS nerd initialized me calling Interface, but I still work: {:?}", s);
    println!("");
    println!("A dumb guy created me with a different name and then aliased me to call strignStruct, anyways I'm: {:?}", a);
    println!("");
    println!("I'm called MyInt, and I'm a i64, Check this out: {}", v);
    println!("");
    println!("People call me char, but secretly I'm a String: {}", char);
    println!("--------------------------------------------------------");
}
```

## 🎯 Common Use Cases

### Language Migration
Coming from TypeScript or other languages? Create familiar syntax:

```rust
define!(number = i64);
define!(string = String);
define!(boolean = bool);
define!(interface = #[derive(Debug, Clone)]struct);
```

### Domain-Specific Languages
Create specialized syntax for your domain:

```rust
// Game development
define!(Entity = u64);
define!(Component = struct);
define!(Health = i32);

// Web development  
define!(UserId = u64);
define!(Email = String);
define!(Token = String);
```

### Code Clarity
Make your code more readable:

```rust
define!(Timestamp = i64);
define!(Price = f64);
define!(Count = usize);
```

## 📝 Syntax Reference

### Basic Syntax
```rust
define!(alias_name = replacement);
```

### Usage Patterns

#### For Types
```rust
// Declaration
define!(MyType = SomeType);

// Usage
let var: MyType!() = value;

// With associated functions
let var: MyType!() = <MyType!()>::from(value);
```

#### For Structs with Keywords
```rust
// Declaration
define!(keyword = #[derive(Debug)]struct);

// Usage
keyword!(StructName {
    field: Type,
});
```

#### For Struct Types
```rust
// Declaration (after defining the struct)
define!(Alias = OriginalStruct);

// Usage
let instance = Alias!({
    field: value
});
```

## ⚙️ How It Works

The `define!` macro creates a `macro_rules!` macro with your specified name:

1. **Compile Time**: `define!(MyInt = i64)` generates a new macro `MyInt!()`
2. **Expansion**: `MyInt!()` expands to `i64`, `MyInt!(args)` expands to `i64 args`
3. **Zero Cost**: Everything is resolved at compile time with no runtime overhead

## 🚨 Important Notes

1. **Associated Functions**: Use `<alias!()>::method()` syntax for calling associated functions
2. **Struct Creation**: For struct aliases, use `Alias!({ field: value })` syntax
3. **Scope**: Aliases are available in the scope where they're defined
4. **Compile Time**: All aliases are resolved at compile time

## 🔧 Building from Source

```bash
git clone https://github.com/shubhiscoding/alias_macros
cd alias_macros
cargo build
cargo test
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🔗 Links

- [Repository](https://github.com/shubhiscoding/alias_macros)
- [Documentation](https://docs.rs/alias_macros)
- [Crates.io](https://crates.io/crates/alias_macros)

---

Made with ❤️ by [Shubh Kesharwani](https://github.com/shubhiscoding)
