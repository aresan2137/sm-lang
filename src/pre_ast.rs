use crate::lexer::{Token, TokenType};
use crate::ast::ast_tools::{ parse_name, parse_type };

pub struct Func {
    pub name: Vec<Token>,
    pub ret_type: Vec<Token>,
    pub args: Vec<Token>,
    pub body: Vec<Token>
}

pub struct PreAST {
    pub functions: Vec<Func>,
    pub glob_inlines: Vec<String>
}

pub fn pre_parse(tokens: &Vec<Token>) -> PreAST {
    let mut ast = PreAST { functions: Vec::new(), glob_inlines: Vec::new() };
    let mut i: usize = 0;
    
    while i < tokens.len() {
        if let Some(t) = peek(tokens, i) {
            if t.value == "func" {
                ast.functions.push(parse_function(tokens, &mut i));
            } else if t.token_type == TokenType::CppBlock {
                
                ast.glob_inlines.push(t.value.replace("$$(", "").replace(")$$", "").trim().to_string());
                i += 1;
            } else {
                i += 1;
            }
        } else {
            break;
        }
    }
    return ast;
}

fn parse_function(tokens: &Vec<Token>, i: &mut usize) -> Func {
    expect(tokens, i, TokenType::Key, "func".to_string());
    let ret_type = parse_type(tokens, i);
    let name = parse_name(tokens, i);

    expect(tokens, i, TokenType::Bracket, "(".to_string());
    
    let mut args = Vec::new();
    while let Some(t) = peek(tokens, *i) {
        if t.token_type == TokenType::Bracket && t.value == ")" { break; }
        args.push(next(tokens, i));
    }
    expect(tokens, i, TokenType::Bracket, ")".to_string());

    let mut body = Vec::new();

    if let Some(t) = peek(tokens, *i) {
        if t.value == "{" {
            next(tokens, i);
            let mut depth = 1;
            while depth > 0 {
                if *i >= tokens.len() { break; }
                let t = next(tokens, i);
                if t.token_type == TokenType::Bracket {
                    if t.value == "{" { depth += 1; }
                    else if t.value == "}" { 
                        depth -= 1; 
                        if depth == 0 { break; }
                    }
                }
                body.push(t);
            }
        } else {
            while let Some(_) = peek(tokens, *i) {
                let current = next(tokens, i);
                body.push(current.clone());
                if current.value == ";" { break; }
                if current.value == "func" { break; } 
            }
        }
    }

    return Func { 
        name, 
        ret_type, 
        args, 
        body 
    };
}

fn peek(tokens: &Vec<Token>, i: usize) -> Option<&Token> {
    if i >= tokens.len() { return None; }
    return Some(&tokens[i]);
}

fn next(tokens: &Vec<Token>, i: &mut usize) -> Token {
    if *i >= tokens.len() { panic!("preAST: next(): i out of file"); }
    
    let tok = tokens[*i].clone();
    *i += 1;
    return tok;
}

fn expect(tokens: &Vec<Token>, i: &mut usize, token_type: TokenType, value: String) -> Token {
    let token = next(tokens, i);
    
    if token.token_type != token_type { panic!("preAST: expect(): unsuspected token type. WANT: {:?} GOT: {:?},{:?}", token_type, token.token_type, token.value); }
    if !value.is_empty() && token.value != value { panic!("preAST: expect(): token value different"); }
    
    return token;
}