# Rustalizer

Rustalizer is a powerful tool designed to help developers analyze complex Rust code structures. It provides a simple Abstract Syntax Tree (AST) parser that can break down Rust traits, structs, and enums into a more manageable and visually comprehensible format.

## Features

- Parse complex Rust code structures including traits, structs, and enums
- Generate an Abstract Syntax Tree (AST) representation of the parsed code
- Display the AST in an easy-to-read, hierarchical tree format
- Support for advanced Rust features like generic types and references
- Handle associated data in enum variants
- Assist in understanding and visualizing intricate Rust code architectures

## Why Rustalizer?

Understanding complex code structures can be challenging, especially in large projects. Rustalizer aims to simplify this process by providing a clear, hierarchical view of your Rust code's structure. This can be particularly useful for:

- Code review and analysis
- Documentation generation
- Learning and teaching Rust's advanced features
- Refactoring and architectural planning

## Usage

To use Rustalizer in your Rust project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
rustalizer = { git = "https://github.com/vschwaberow/rustalizer.git" }
```

Then, you can use the `rustalizer` crate in your Rust code. Here's an example of how to parse and display a Rust trait using Rustalizer:

```rust
use rustalizer::AstNode;
use std::str::FromStr;

fn main() {
    let complex_struct = r#"
        pub struct ComplexStruct<T, U> {
            data: Vec<T>,
            processor: Box<dyn Fn(T) -> U>,
            metadata: Option<String>,
        }
    "#;

    match AstNode::from_str(complex_struct) {
        Ok(ast) => {
            println!("Analyzed structure:");
            ast.display_tree();
        }
        Err(e) => println!("Error analyzing structure: {}", e),
    }
}
```

This example demonstrates how to parse a complex Rust structure and display its AST. You can extend this example to handle other types of Rust code structures as well.

## Contributing

We welcome contributions to Rustalizer! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request on our GitHub repository.

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.
