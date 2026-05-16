use crate::ast::tree::*;
use crate::lexer::Token;

pub fn scan_tokens_until_str(tokens: &Vec<Token>, pos: &mut usize, seperator: &str) -> Vec<Token> {
    let mut expr_tokens = Vec::new();
    while *pos < tokens.len() && tokens[*pos].value != seperator {
        expr_tokens.push(tokens[*pos].clone());
        *pos += 1;
    }

    *pos += 1;

    return expr_tokens;
}

pub fn scan_tokens_until_bracket_stack_empty(tokens: &Vec<Token>, pos: &mut usize) -> Vec<Token> {
    if *pos >= tokens.len() || tokens[*pos].value != "(" {
        panic!("Parser error: Expected '(' at token {}", *pos);
    }

    let mut expr_tokens = Vec::new();
    let mut bracket_stack = Vec::new();

    bracket_stack.push("(");
    *pos += 1; 

    while *pos < tokens.len() {
        let t = &tokens[*pos];

        if t.value == "(" {
            bracket_stack.push("(");
        } else if t.value == ")" {
            bracket_stack.pop();
            
            if bracket_stack.is_empty() {
                *pos += 1;
                return expr_tokens;
            }
        }

        expr_tokens.push(t.clone());
        *pos += 1;
    }

    panic!("Parser error: Unclosed parenthesis");
}

pub fn tree_with_scaning_until_bracket_stack_empty(tokens: &Vec<Token>, pos: &mut usize) -> Tree {
    return parse_tree(&&scan_tokens_until_bracket_stack_empty(tokens, pos));
}

pub fn tree_with_scaning_until_str(tokens: &Vec<Token>, pos: &mut usize, seperator: &str) -> Tree {
    return parse_tree(&scan_tokens_until_str(tokens, pos, seperator));
}

pub fn compare_with_incr(tokens: &Vec<Token>, pos: &mut usize, value: &str) -> bool {
    if tokens[*pos].value == value {
        *pos += 1;
        return true;
    }
    return false; 
}

pub fn parse_type(tokens: &Vec<Token>, pos: &mut usize) -> Vec<Token> {
    let mut expr_tokens = Vec::new();

    let mut base_type = parse_name(tokens, pos);
    
    if base_type.is_empty() && *pos < tokens.len() {
        expr_tokens.push(tokens[*pos].clone());
        *pos += 1;
    } else {
        expr_tokens.append(&mut base_type);
    }

    if *pos < tokens.len() && tokens[*pos].value == "<" {
        expr_tokens.push(tokens[*pos].clone());
        *pos += 1;

        while *pos < tokens.len() && tokens[*pos].value != ">" {
            let mut inner_type = parse_type(tokens, pos);
            expr_tokens.append(&mut inner_type);

            if *pos < tokens.len() && tokens[*pos].value == "," {
                expr_tokens.push(tokens[*pos].clone());
                *pos += 1;
            }
        }

        if *pos < tokens.len() && tokens[*pos].value == ">" {
            expr_tokens.push(tokens[*pos].clone());
            *pos += 1;
        }
    }
    return expr_tokens;
}

pub fn parse_name(tokens: &Vec<Token>, pos: &mut usize) -> Vec<Token> {
    let mut name_tokens = Vec::new();

    name_tokens.push(tokens[*pos].clone());
    *pos += 1;


    while *pos + 1 < tokens.len() {
        if tokens[*pos].value == "::" {
            name_tokens.push(tokens[*pos].clone());
            name_tokens.push(tokens[*pos + 1].clone());
            *pos += 2;
        } else {
            break;
        }
    }

    return name_tokens;
}

