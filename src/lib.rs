use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum AstNode {
    Trait(TraitNode),
    Struct(StructNode),
    Enum(EnumNode),
}

#[derive(Debug, PartialEq)]
pub struct TraitNode {
    pub name: String,
    pub methods: Vec<MethodNode>,
}

#[derive(Debug, PartialEq)]
pub struct StructNode {
    pub name: String,
    pub fields: Vec<FieldNode>,
}

#[derive(Debug, PartialEq)]
pub struct EnumNode {
    pub name: String,
    pub variants: Vec<VariantNode>,
}

#[derive(Debug, PartialEq)]
pub struct MethodNode {
    pub name: String,
    pub params: Vec<ParamNode>,
    pub return_type: Option<Box<TypeNode>>,
}

#[derive(Debug, PartialEq)]
pub struct ParamNode {
    pub name: String,
    pub param_type: Box<TypeNode>,
}

#[derive(Debug, PartialEq)]
pub struct FieldNode {
    pub name: String,
    pub field_type: Box<TypeNode>,
}

#[derive(Debug, PartialEq)]
pub struct VariantNode {
    pub name: String,
    pub associated_data: Option<Box<AstNode>>,
}

#[derive(Debug, PartialEq)]
pub enum TypeNode {
    Simple(String),
    Reference(Box<TypeNode>),
    Generic { name: String, args: Vec<TypeNode> },
}

pub struct Parser;

impl Parser {
    pub fn parse(input: &str) -> Result<AstNode, String> {
        let input = input.trim();
        if input.starts_with("pub trait") {
            Parser::parse_trait(input)
        } else if input.starts_with("pub struct") {
            Parser::parse_struct(input)
        } else if input.starts_with("pub enum") {
            Parser::parse_enum(input)
        } else {
            Err("Unsupported or invalid Rust construct".to_string())
        }
    }

    fn parse_trait(input: &str) -> Result<AstNode, String> {
        let trait_name = input
            .split_whitespace()
            .nth(2)
            .ok_or("Invalid trait definition")?
            .to_string();

        let body_start = input.find('{').ok_or("Missing trait body")?;
        let body_end = input.rfind('}').ok_or("Missing closing brace")?;
        if body_end <= body_start {
            return Err("Invalid trait body".to_string());
        }
        let body_content = &input[body_start + 1..body_end].trim();

        let method_strings: Vec<&str> = body_content
            .split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        let mut methods = Vec::new();
        for method_str in method_strings {
            methods.push(Self::parse_method(method_str)?);
        }

        Ok(AstNode::Trait(TraitNode {
            name: trait_name,
            methods,
        }))
    }

    fn parse_struct(input: &str) -> Result<AstNode, String> {
        let struct_name = input
            .split_whitespace()
            .nth(2)
            .ok_or("Invalid struct definition")?
            .to_string();

        let body_start = input.find('{').ok_or("Missing struct body")?;
        let body_end = input.rfind('}').ok_or("Missing closing brace")?;
        if body_end <= body_start {
            return Err("Invalid struct body".to_string());
        }
        let body_content = &input[body_start + 1..body_end].trim();

        let fields = body_content
            .split(',')
            .map(|field_str| {
                let parts: Vec<&str> = field_str.split(':').collect();
                if parts.len() != 2 {
                    return Err("Invalid field format".to_string());
                }
                Ok(FieldNode {
                    name: parts[0].trim().to_string(),
                    field_type: Box::new(Self::parse_type(parts[1].trim())?),
                })
            })
            .collect::<Result<Vec<FieldNode>, String>>()?;

        Ok(AstNode::Struct(StructNode {
            name: struct_name,
            fields,
        }))
    }

    fn parse_enum(input: &str) -> Result<AstNode, String> {
        let enum_name = input
            .split_whitespace()
            .nth(2)
            .ok_or("Invalid enum definition")?
            .to_string();

        let body_start = input.find('{').ok_or("Missing enum body")?;
        let body_end = input.rfind('}').ok_or("Missing closing brace")?;
        if body_end <= body_start {
            return Err("Invalid enum body".to_string());
        }
        let body_content = &input[body_start + 1..body_end].trim();

        let variant_strings: Vec<&str> = body_content
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        let mut variants = Vec::new();
        for variant_str in variant_strings {
            if variant_str.contains('(') && variant_str.contains(')') {
                // Variant with associated data
                let name = variant_str.split('(').next().unwrap().trim().to_string();
                let data_str = variant_str.split('(').nth(1).unwrap().trim_end_matches(')');
                // For simplicity, assume associated data is a struct
                let associated_ast = Parser::parse(data_str)?;
                variants.push(VariantNode {
                    name,
                    associated_data: Some(Box::new(associated_ast)),
                });
            } else {
                // Simple variant
                variants.push(VariantNode {
                    name: variant_str.to_string(),
                    associated_data: None,
                });
            }
        }

        Ok(AstNode::Enum(EnumNode {
            name: enum_name,
            variants,
        }))
    }

    fn parse_method(input: &str) -> Result<MethodNode, String> {
        let input = input.trim();
        let parts: Vec<&str> = input.split(&['(', ')']).collect();
        if parts.len() < 2 {
            return Err("Invalid method format".to_string());
        }

        let name = parts[0]
            .split_whitespace()
            .nth(1)
            .ok_or("Invalid method name")?
            .to_string();

        let params = Self::parse_params(parts[1])?;

        let return_type = if input.contains("->") {
            let return_str = input
                .split("->")
                .nth(1)
                .unwrap()
                .trim()
                .trim_end_matches(';')
                .to_string();
            Some(Box::new(Self::parse_type(&return_str)?))
        } else {
            None
        };

        Ok(MethodNode {
            name,
            params,
            return_type,
        })
    }

    fn parse_params(input: &str) -> Result<Vec<ParamNode>, String> {
        if input.trim().is_empty() {
            return Ok(Vec::new());
        }

        input
            .split(',')
            .map(|param| {
                let param = param.trim();
                if param == "&self" {
                    Ok(ParamNode {
                        name: "&self".to_string(),
                        param_type: Box::new(TypeNode::Reference(Box::new(TypeNode::Simple("self".to_string())))),
                    })
                } else if param == "self" {
                    Ok(ParamNode {
                        name: "self".to_string(),
                        param_type: Box::new(TypeNode::Simple("self".to_string())),
                    })
                } else {
                    let parts: Vec<&str> = param.split(':').collect();
                    if parts.len() != 2 {
                        return Err("Invalid parameter format".to_string());
                    }
                    Ok(ParamNode {
                        name: parts[0].trim().to_string(),
                        param_type: Box::new(Self::parse_type(parts[1].trim())?),
                    })
                }
            })
            .collect()
    }

    fn parse_type(input: &str) -> Result<TypeNode, String> {
        if input.starts_with('&') {
            let inner = input.trim_start_matches('&').trim();
            let inner_type = Self::parse_type(inner)?;
            Ok(TypeNode::Reference(Box::new(inner_type)))
        } else if input.starts_with('[') && input.ends_with(']') {
            let inner_str = &input[1..input.len()-1].trim();
            let inner_type = Self::parse_type(inner_str)?;
            Ok(TypeNode::Generic {
                name: "[]".to_string(),
                args: vec![inner_type],
            })
        } else if input.contains('<') && input.contains('>') {
            let name = input.split('<').next().unwrap().trim().to_string();
            let args_str = input
                .split('<')
                .nth(1)
                .unwrap()
                .trim_end_matches('>')
                .trim();
            let args: Result<Vec<TypeNode>, String> = args_str
                .split(',')
                .map(|arg| Self::parse_type(arg.trim()))
                .collect();
            Ok(TypeNode::Generic { name, args: args? })
        } else {
            Ok(TypeNode::Simple(input.to_string()))
        }
    }

    fn parse_tuple_variant(input: &str) -> Result<AstNode, String> {
        let fields: Vec<FieldNode> = input
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .enumerate()
            .map(|(i, s)| -> Result<FieldNode, String> {
                Ok(FieldNode {
                    name: format!("{}", i),
                    field_type: Box::new(Self::parse_type(s)?),
                })
            })
            .collect::<Result<Vec<FieldNode>, String>>()?;

        Ok(AstNode::Struct(StructNode {
            name: "".to_string(),
            fields,
        }))
    }
}

impl FromStr for AstNode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Parser::parse(s)
    }
}

// Tree Display Implementation with Recursive Traversal
impl AstNode {
    pub fn display_tree(&self) {
        self.display_tree_internal("");
    }

    fn display_tree_internal(&self, prefix: &str) {
        match self {
            AstNode::Trait(trait_node) => {
                println!("{}- Trait: {}", prefix, trait_node.name);
                let len = trait_node.methods.len();
                for (i, method) in trait_node.methods.iter().enumerate() {
                    let is_last = i == len - 1;
                    let branch = if is_last { "└──" } else { "├──" };
                    let new_prefix = format!("{}{} ", prefix, branch);
                    method.display_tree_internal(&new_prefix, is_last);
                }
            }
            AstNode::Struct(struct_node) => {
                println!("{}- Struct: {}", prefix, struct_node.name);
                let len = struct_node.fields.len();
                for (i, field) in struct_node.fields.iter().enumerate() {
                    let is_last = i == len - 1;
                    let branch = if is_last { "└──" } else { "├──" };
                    let new_prefix = format!("{}{} ", prefix, branch);
                    field.display_tree_internal(&new_prefix, is_last);
                }
            }
            AstNode::Enum(enum_node) => {
                println!("{}- Enum: {}", prefix, enum_node.name);
                let len = enum_node.variants.len();
                for (i, variant) in enum_node.variants.iter().enumerate() {
                    let is_last = i == len - 1;
                    let branch = if is_last { "└──" } else { "├──" };
                    let new_prefix = format!("{}{} ", prefix, branch);
                    variant.display_tree_internal(&new_prefix, is_last);
                }
            }
        }
    }
}

impl MethodNode {
    fn display_tree_internal(&self, prefix: &str, is_last: bool) {
        let _ = is_last;
        println!("{}Method: {}", prefix, self.name);
        let len = self.params.len();
        for (i, param) in self.params.iter().enumerate() {
            let is_last_param = i == len - 1;
            let branch = if is_last_param {
                "└──"
            } else {
                "├──"
            };
            let param_prefix = format!("{}{} ", prefix, branch);
            param.display_tree_internal(&param_prefix, is_last_param);
        }
        if let Some(return_type) = &self.return_type {
            let branch = if len == 0 { "└──" } else { "├──" };
            let return_prefix = format!("{}{} ", prefix, branch);
            println!("{}Return Type: {}", return_prefix, return_type.display());
        }
    }
}

impl FieldNode {
    fn display_tree_internal(&self, prefix: &str, _is_last: bool) {
        println!(
            "{}Field: {}: {}",
            prefix,
            self.name,
            self.field_type.display()
        );
    }
}

impl VariantNode {
    fn display_tree_internal(&self, prefix: &str, _is_last: bool) {
        println!("{}Variant: {}", prefix, self.name);
        if let Some(associated_data) = &self.associated_data {
            // Recursively display the associated AstNode
            associated_data.display_tree_internal(&format!("{}    ", prefix));
        }
    }
}

impl TypeNode {
    fn display(&self) -> String {
        match self {
            TypeNode::Simple(name) => name.clone(),
            TypeNode::Reference(inner) => format!("&{}", inner.display()),
            TypeNode::Generic { name, args } => {
                let args_display: Vec<String> = args.iter().map(|arg| arg.display()).collect();
                format!("{}<{}>", name, args_display.join(", "))
            }
        }
    }
}

impl ParamNode {
    fn display_tree_internal(&self, prefix: &str, _is_last: bool) {
        println!(
            "{}Param: {}: {}",
            prefix,
            self.name,
            self.param_type.display()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_visualizer_trait() {
        let inputs = vec![
            r#"pub trait Visualizer {
                fn visualize(&self, data: &[u8]);
                fn process(&self, input: &str) -> String;
            }"#,
            r#"
            pub trait Visualizer {
                fn visualize(&self, data: &[u8]);
                fn process(&self, input: &str) -> String;
            }
            "#,
            "pub trait Visualizer { fn visualize(&self, data: &[u8]); fn process(&self, input: &str) -> String; }",
        ];

        for input in inputs {
            let expected = AstNode::Trait(TraitNode {
                name: "Visualizer".to_string(),
                methods: vec![
                    MethodNode {
                        name: "visualize".to_string(),
                        params: vec![
                            ParamNode {
                                name: "&self".to_string(),
                                param_type: Box::new(TypeNode::Reference(Box::new(TypeNode::Simple("self".to_string())))),
                            },
                            ParamNode {
                                name: "data".to_string(),
                                param_type: Box::new(TypeNode::Generic {
                                    name: "[]".to_string(),
                                    args: vec![TypeNode::Simple("u8".to_string())],
                                }),
                            },
                        ],
                        return_type: None,
                    },
                    MethodNode {
                        name: "process".to_string(),
                        params: vec![
                            ParamNode {
                                name: "&self".to_string(),
                                param_type: Box::new(TypeNode::Reference(Box::new(TypeNode::Simple("self".to_string())))),
                            },
                            ParamNode {
                                name: "input".to_string(),
                                param_type: Box::new(TypeNode::Reference(Box::new(TypeNode::Simple("str".to_string())))),
                            },
                        ],
                        return_type: Some(Box::new(TypeNode::Simple("String".to_string()))),
                    },
                ],
            });

            assert_eq!(input.parse::<AstNode>().unwrap(), expected);
        }
    }

    #[test]
    fn test_parse_struct() {
        let input = r#"
            pub struct Point {
                x: f64,
                y: f64,
                label: String,
            }
        "#;

        let expected = AstNode::Struct(StructNode {
            name: "Point".to_string(),
            fields: vec![
                FieldNode {
                    name: "x".to_string(),
                    field_type: Box::new(TypeNode::Simple("f64".to_string())),
                },
                FieldNode {
                    name: "y".to_string(),
                    field_type: Box::new(TypeNode::Simple("f64".to_string())),
                },
                FieldNode {
                    name: "label".to_string(),
                    field_type: Box::new(TypeNode::Simple("String".to_string())),
                },
            ],
        });

        assert_eq!(input.parse::<AstNode>().unwrap(), expected);
    }

    #[test]
    fn test_parse_enum() {
        let input = r#"
            pub enum Color {
                Red,
                Green,
                Blue,
            }
        "#;

        let expected = AstNode::Enum(EnumNode {
            name: "Color".to_string(),
            variants: vec![
                VariantNode {
                    name: "Red".to_string(),
                    associated_data: None,
                },
                VariantNode {
                    name: "Green".to_string(),
                    associated_data: None,
                },
                VariantNode {
                    name: "Blue".to_string(),
                    associated_data: None,
                },
            ],
        });

        assert_eq!(input.parse::<AstNode>().unwrap(), expected);
    }

    #[test]
    fn test_parse_enum_with_associated_data() {
        let input = r#"
            pub enum Message {
                Quit,
                Move { x: i32, y: i32 },
                Write(String),
                ChangeColor(i32, i32, i32),
            }
        "#;

        let expected = AstNode::Enum(EnumNode {
            name: "Message".to_string(),
            variants: vec![
                VariantNode {
                    name: "Quit".to_string(),
                    associated_data: None,
                },
                VariantNode {
                    name: "Move".to_string(),
                    associated_data: Some(Box::new(AstNode::Struct(StructNode {
                        name: "".to_string(), // Anonymous struct
                        fields: vec![
                            FieldNode {
                                name: "x".to_string(),
                                field_type: Box::new(TypeNode::Simple("i32".to_string())),
                            },
                            FieldNode {
                                name: "y".to_string(),
                                field_type: Box::new(TypeNode::Simple("i32".to_string())),
                            },
                        ],
                    }))),
                },
                VariantNode {
                    name: "Write".to_string(),
                    associated_data: Some(Box::new(AstNode::Struct(StructNode {
                        name: "".to_string(), // Tuple struct equivalent
                        fields: vec![FieldNode {
                            name: "0".to_string(),
                            field_type: Box::new(TypeNode::Simple("String".to_string())),
                        }],
                    }))),
                },
                VariantNode {
                    name: "ChangeColor".to_string(),
                    associated_data: Some(Box::new(AstNode::Struct(StructNode {
                        name: "".to_string(), // Tuple struct equivalent
                        fields: vec![
                            FieldNode {
                                name: "0".to_string(),
                                field_type: Box::new(TypeNode::Simple("i32".to_string())),
                            },
                            FieldNode {
                                name: "1".to_string(),
                                field_type: Box::new(TypeNode::Simple("i32".to_string())),
                            },
                            FieldNode {
                                name: "2".to_string(),
                                field_type: Box::new(TypeNode::Simple("i32".to_string())),
                            },
                        ],
                    }))),
                },
            ],
        });

        assert_eq!(input.parse::<AstNode>().unwrap(), expected);
    }

    #[test]
    fn test_invalid_input() {
        let input = "fn standalone_function() {}";
        assert!(input.parse::<AstNode>().is_err());
    }

    #[test]
    fn test_parse_struct_with_invalid_field() {
        let input = r#"
            pub struct InvalidStruct {
                x f64, // Missing colon
                y: f64,
            }
        "#;

        assert!(input.parse::<AstNode>().is_err());
    }

    #[test]
    fn test_parse_trait_with_invalid_method() {
        let input = r#"
            pub trait InvalidTrait {
                fn invalid_method(&self data: &[u8]); // Missing comma
            }
        "#;

        assert!(input.parse::<AstNode>().is_err());
    }
}