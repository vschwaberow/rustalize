use rustalize::AstNode;
use std::str::FromStr;

fn main() {
    let trait_definition = r#"
        pub trait Visualizer {
            fn visualize(&self, data: &[u8]);
            fn process(&self, input: &str) -> String;
        }
    "#;

    match AstNode::from_str(trait_definition) {
        Ok(ast) => {
            println!("Parsed AST:");
            println!("{:#?}", ast);
        }
        Err(e) => {
            eprintln!("Failed to parse trait: {}", e);
        }
    }
}
