use crate::ast::tree::*;
use crate::lexer::{ TokenType, Token };

pub fn scan_tokens_until_str(tokens: &Vec<Token>, pos: &mut usize, seperator: &str) -> Vec<Token> {
    let mut expr_tokens = Vec::new();
    while *pos < tokens.len() && tokens[*pos].value != seperator {
        expr_tokens.push(tokens[*pos].clone());
        *pos += 1;
    }

    *pos += 1;

    return expr_tokens;
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
    expr_tokens.push(tokens[*pos].clone());
    *pos += 1;
    if compare_with_incr(tokens, pos, "<") {
        expr_tokens.push(Token { value: String::from("<"), token_type: TokenType::Symbol });
        let mut inner = scan_tokens_until_str(tokens, pos, ">");
        expr_tokens.append(&mut inner);
        expr_tokens.push(Token { value: String::from(">"), token_type: TokenType::Symbol });
    }

    return expr_tokens;
}