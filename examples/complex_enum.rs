use rustalize::AstNode;
use std::str::FromStr;

fn main() {
    let enum_definition = r#"
        pub enum Message {
            Quit,
        }
    "#;

    match AstNode::from_str(enum_definition) {
        Ok(ast) => {
            println!("Parsed AST (Tree Layout):");
            ast.display_tree();
        }
        Err(e) => {
            eprintln!("Failed to parse enum: {}", e);
        }
    }
}
