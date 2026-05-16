use crate::lexer::{Token};
use crate::ast::ast_tools::*;

#[derive(Debug, Clone)]
pub enum Op { 
    Add, 
    Sub, 
    Mul, 
    Div,
    Mod,
    Eq, 
    Neq, 
    Less, 
    Greater,
    LOE,
    GOE,
    Concat
}

#[derive(Debug, Clone)]
pub enum Tree {
    Leaf(String),
    Node { op: Op, left: Box<Tree>, right: Box<Tree> },
    Call { func_name: String, args: Vec<Tree> },
    MethodCall { object: Box<Tree>, method_name: String, args: Vec<Tree> },
    Array(Vec<Tree>)
}

fn parse_factor(tokens: &[Token], pos: &mut usize) -> Tree {
    if *pos >= tokens.len() {
        return Tree::Leaf(String::new());
    }

    let mut node = if tokens[*pos].value == "[" {
        *pos += 1;
        let mut elements = Vec::new();
        while *pos < tokens.len() && tokens[*pos].value != "]" {
            elements.push(parse_tree_counter(tokens, pos));
            if *pos < tokens.len() && tokens[*pos].value == "," {
                *pos += 1;
            }
        }
        if *pos < tokens.len() && tokens[*pos].value == "]" {
            *pos += 1;
        } else {
            panic!("Parser error: Expected ']'");
        }
        Tree::Array(elements)
    } else if tokens[*pos].value == "-" {
        *pos += 1;
        let right = parse_factor(tokens, pos);
        Tree::Node {
            op: Op::Sub,
            left: Box::new(Tree::Leaf("0".to_string())),
            right: Box::new(right),
        }
    } else if tokens[*pos].value == "(" {
        *pos += 1;
        let tree = parse_tree_counter(tokens, pos);
        if *pos < tokens.len() && tokens[*pos].value == ")" {
            *pos += 1;
        }
        tree
    } else {
        let name_tokens = parse_name(&tokens.to_vec(), pos);
        let name = name_tokens.iter().map(|t| t.value.as_str()).collect::<Vec<_>>().join("");

        if *pos < tokens.len() && tokens[*pos].value == "(" {
            *pos += 1;
            let mut args = Vec::new();
            while *pos < tokens.len() && tokens[*pos].value != ")" {
                args.push(parse_tree_counter(tokens, pos));
                if *pos < tokens.len() && tokens[*pos].value == "," {
                    *pos += 1;
                }
            }
            if *pos < tokens.len() && tokens[*pos].value == ")" {
                *pos += 1;
            }
            Tree::Call { func_name: name, args }
        } else {
            Tree::Leaf(name)
        }
    };

    while *pos < tokens.len() && (tokens[*pos].value == "." || tokens[*pos].value == "[") {
        if tokens[*pos].value == "." {
            *pos += 1;
            if *pos >= tokens.len() {
                panic!("Parser error: Expected method name after '.'");
            }

            let method_name = tokens[*pos].value.clone();
            *pos += 1;

            if *pos < tokens.len() && tokens[*pos].value == "(" {
                *pos += 1;
                let mut args = Vec::new();
                while *pos < tokens.len() && tokens[*pos].value != ")" {
                    args.push(parse_tree_counter(tokens, pos));
                    if *pos < tokens.len() && tokens[*pos].value == "," {
                        *pos += 1;
                    }
                }
                if *pos < tokens.len() && tokens[*pos].value == ")" {
                    *pos += 1;
                }
                
                node = Tree::MethodCall {
                    object: Box::new(node),
                    method_name,
                    args,
                };
            } else {
                node = Tree::MethodCall {
                    object: Box::new(node),
                    method_name,
                    args: Vec::new(),
                };
            }
        } else if tokens[*pos].value == "[" {
            *pos += 1;
            
            let index_tree = parse_tree_counter(tokens, pos);
            
            if *pos < tokens.len() && tokens[*pos].value == "]" {
                *pos += 1;
            } else {
                panic!("Parser error: Expected ']' after array index");
            }

            node = Tree::Leaf(format!("{}[{}]", translate_tree(&node), translate_tree(&index_tree)));
        }
    }

    node
}

fn parse_mul(tokens: &[Token], pos: &mut usize) -> Tree {
    let mut left = parse_factor(tokens, pos);

    while *pos < tokens.len() {
        let val = &tokens[*pos].value;
        if val == "*" || val == "/" || val == "%" {
            let op = if val == "*" { Op::Mul } else if val == "/" { Op::Div } else { Op::Mod };
            *pos += 1;
            let right = parse_factor(tokens, pos);
            left = Tree::Node { op, left: Box::new(left), right: Box::new(right) };
        } else { break; }
    }
    return left;
}

pub fn parse_tree(tokens: &[Token]) -> Tree {
    let mut pos: usize = 0;
    return parse_tree_counter(tokens, &mut pos);
}

pub fn parse_tree_counter(tokens: &[Token], pos: &mut usize) -> Tree {
    let mut left = parse_add_sub(tokens, pos);

    while *pos < tokens.len() {
        let val = &tokens[*pos].value;
        if val == "==" || val == "!=" || val == "<" || val == ">" || val == "<=" || val == ">=" {
            let op = match val.as_str() {
                "==" => Op::Eq,
                "!=" => Op::Neq,
                "<" => Op::Less,
                ">" => Op::Greater,
                "<=" => Op::LOE,
                ">=" => Op::GOE,
                _ => unreachable!()
            };
            *pos += 1;
            let right = parse_add_sub(tokens, pos);
            left = Tree::Node { op, left: Box::new(left), right: Box::new(right) };
        } else { break; }
    }
    return left;
}

fn parse_add_sub(tokens: &[Token], pos: &mut usize) -> Tree {
    let mut left = parse_mul(tokens, pos);

    while *pos < tokens.len() {
        let val = &tokens[*pos].value;
        if val == "+" || val == "-" || val == ".." {
            let op = if val == "+" { Op::Add } else if val == "-" { Op::Sub } else { Op::Concat };
            *pos += 1;
            let right = parse_mul(tokens, pos);
            left = Tree::Node { op, left: Box::new(left), right: Box::new(right) };
        } else { break; }
    }
    return left;
}

pub fn translate_tree(tree: &Tree) -> String {
    match tree {
        Tree::Leaf(val) => val.clone(),
        Tree::Node { op, left, right } => {
            if let Op::Concat = op {
                let l_s = translate_tree(left);
                let r_s = translate_tree(right);

                let left_f = if l_s.starts_with("\"") { l_s } else { format!("std::to_string({})", l_s) };
                let right_f = if r_s.starts_with("\"") { r_s } else { format!("std::to_string({})", r_s) };

                return format!("{} + {}", left_f, right_f);
            }
            
            let op_s = match op {
                Op::Add => "+", Op::Sub => "-",
                Op::Mul => "*", Op::Div => "/",
                Op::Mod => "%", Op::Eq => "==",
                Op::Neq => "!=", Op::Less => "<",
                Op::Greater => ">", Op::LOE => "<=", Op::GOE => ">=",
                _ => ""
            };
            format!("({} {} {})", translate_tree(left), op_s, translate_tree(right))
        },
        Tree::Call { func_name, args } => {
            let arg_strs: Vec<String> = args.iter().map(|a| translate_tree(a)).collect();
            format!("{}({})", func_name, arg_strs.join(", "))
        },
        Tree::MethodCall { object, method_name, args } => {
            let obj_s = translate_tree(object);
            let arg_strs: Vec<String> = args.iter().map(|a| translate_tree(a)).collect();
            
            format!("{}.{}({})", obj_s, method_name, arg_strs.join(", "))
        },
        Tree::Array(elements) => {
            let elem_strs: Vec<String> = elements.iter().map(|e| translate_tree(e)).collect();
            format!("{{ {} }}", elem_strs.join(", "))
        }
    }
}