use crate::lexer::{Token};

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
    GOE
}

#[derive(Debug, Clone)]
pub enum Tree {
    Leaf(String),
    Node { op: Op, left: Box<Tree>, right: Box<Tree> },
    Call { func_name: String, args: Vec<Tree> }
}

fn parse_factor(tokens: &[Token], pos: &mut usize) -> Tree {
    if tokens[*pos].value == "-" {
        *pos += 1;
        let right = parse_factor(tokens, pos);
        return Tree::Node {
            op: Op::Sub,
            left: Box::new(Tree::Leaf("0".to_string())),
            right: Box::new(right),
        };
    }

    if tokens[*pos].value == "(" {
        *pos += 1;
        let tree = parse_tree_counter(tokens, pos);
        if *pos < tokens.len() && tokens[*pos].value == ")" {
            *pos += 1;
        }
        return tree;
    }

    let name = tokens[*pos].value.clone();
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
        return Tree::Call { func_name: name, args };
    }

    return Tree::Leaf(name);
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
        if val == "+" || val == "-" {
            let op = if val == "+" { Op::Add } else { Op::Sub };
            *pos += 1;
            let right = parse_mul(tokens, pos);
            left = Tree::Node { op, left: Box::new(left), right: Box::new(right) };
        } else { break; }
    }
    return left;
}

pub fn translate_tree(tree: &Tree) -> String {
    match tree {
        Tree::Leaf(val) => {
            return val.clone();
        },
        Tree::Node { op, left, right } => {
            let op_s = match op {
                Op::Add => "+", Op::Sub => "-",
                Op::Mul => "*", Op::Div => "/",
                Op::Mod => "%",
                Op::Eq => "==", Op::Neq => "!=",
                Op::Less => "<", Op::Greater => ">",
                Op::LOE => "<=", Op::GOE => ">="
            };
            return format!("({} {} {})", translate_tree(left), op_s, translate_tree(right));
        },
        Tree::Call { func_name, args } => {
            let arg_strs: Vec<String> = args.iter().map(|a| translate_tree(a)).collect();
            return format!("{}({})", func_name, arg_strs.join(", "));
        }
    }
}