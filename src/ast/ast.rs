use crate::lexer::{Token, TokenType};
use crate::var_find::{Vars};
use crate::pre_ast::{PreAST};

use crate::ast::ast_tools::*;
use crate::ast::tree::*;

#[derive(Debug)]
pub enum Part {
    Parts {
        parts: Vec<Part>
    },
    Var {
        var_type: Vec<Token>,
        var_name: String,
        value: Tree,
        is_bodyless: bool
    },
    Ret {
        value: Tree
    },
    RetVoid {

    },
    VarSet {
        var_name: String,
        value: Tree
    },
    VarInc {
        var_name: String,
    },
    VarDec {
        var_name: String,
    },
    FuncCall {
        func_name: String, 
        args: Vec<Tree>
    },
    CppBlock(String),
    If {
        value: Tree
    },
    Else {

    },
    While {
        value: Tree
    }
}

#[derive(Debug)]
pub struct FunctionAST {
    pub name: String,
    pub ret_type: Vec<Token>,
    pub body: Part,
    pub args: Vec<Token>
}

#[derive(Debug)]
pub struct AST {
    pub functions: Vec<FunctionAST>,
    pub glob_inlines: Vec<Token>
}

pub fn parse_ast(pre_ast: &PreAST, vars: &Vars) -> AST {
    let mut final_ast = AST { functions: Vec::new(), glob_inlines: pre_ast.glob_inlines.clone() };

    for pre_func in &pre_ast.functions {
        let mut pos = 0;
        let tokens = &pre_func.body;
        
        let mut body_parts = Vec::new();
        while pos < tokens.len() {
            if let Some(part) = create_part(tokens, &mut pos, pre_ast, vars) {
                body_parts.push(part);
            } else {
                pos += 1;
            }
        }

        final_ast.functions.push(FunctionAST {
            name: pre_func.name.clone(),
            ret_type: pre_func.ret_type.clone(),
            body: Part::Parts { parts: body_parts },
            args: pre_func.args.clone()
        });
    }
    return final_ast;
}

fn create_part(tokens: &Vec<Token>, pos: &mut usize, pre_ast: &PreAST, vars: &Vars) -> Option<Part> {
    if *pos >= tokens.len() { return None; }
    let t = &tokens[*pos];
    *pos += 1;

    if t.value == "{" {
        let mut inner = Vec::new();

        while *pos < tokens.len() && tokens[*pos].value != "}" {
            if let Some(p) = create_part(tokens, pos, pre_ast, vars) {
                inner.push(p);
            }
        }
        if *pos < tokens.len() { *pos += 1; }
        return Some(Part::Parts { parts: inner });
    } else if t.token_type == TokenType::CppBlock {
        let code = t.value.replace("$$(", "").replace(")$$", "").trim().to_string();
        return Some(Part::CppBlock(code));
    } else if t.value == "ret" {
        if compare_with_incr(tokens, pos, ";"){
            return Some(Part::RetVoid {});
        }

        return Some(Part::Ret { 
            value: tree_with_scaning_until_str(&tokens, pos, ";")
        });
    } else if is_a_function(t.value.clone(), pre_ast) {
        *pos -= 1;
        let tree = tree_with_scaning_until_str(tokens, pos, ";");
        if let Tree::Call { func_name, args } = tree {
            return Some(Part::FuncCall { func_name, args });
        }
    } else if t.value == "if" {
        *pos += 1; // (

        let tree = tree_with_scaning_until_str(&tokens, pos, ")");

        return Some(Part::If {
            value: tree
        });
    } else if t.value == "else" {
        return Some(Part::Else {}); 
    } else if t.value == "while" {
        *pos += 1; // (

        let tree = tree_with_scaning_until_str(&tokens, pos, ")");

        return Some(Part::While {
            value: tree
        });
    } else if t.value == "var" {
        let v_type = parse_type(tokens, pos); 
        
        let v_name = tokens[*pos].value.clone(); *pos += 1;
        if *pos < tokens.len() && tokens[*pos].value == "=" {
            *pos += 1;
            
            return Some(Part::Var { var_type: v_type, var_name: v_name, value: tree_with_scaning_until_str(tokens, pos, ";"), is_bodyless: false });
        } else if *pos < tokens.len() && tokens[*pos].value == ";" {            
            *pos += 1;
            return Some(Part::Var { var_type: v_type, var_name: v_name, value: Tree::Leaf(String::from("0")), is_bodyless: true});
        } else {
            panic!("ast: var");
        }
    } else if vars.vars.contains(&t.value) {
        return Some(parse_var_change(tokens, pos, &t));
    }

    return None;
}

fn parse_var_change(tokens: &Vec<Token>, pos: &mut usize, t: &Token) -> Part {
    let name = t.value.clone();

    if tokens[*pos].value == "++" {
        *pos += 2;
        return Part::VarInc { 
            var_name: name
        };
    } else if tokens[*pos].value == "--" {
        *pos += 2;
        return Part::VarDec { 
            var_name: name
        };
    }

    let mut expr_tokens = Vec::new();

    if *pos < tokens.len() && tokens[*pos].value == "=" {

    } else if *pos < tokens.len() && tokens[*pos].value == "+=" {
        expr_tokens.push(Token { value: name.clone(), token_type: TokenType::Symbol });
        expr_tokens.push(Token { value: String::from("+"), token_type: TokenType::Symbol });
        expr_tokens.push(Token { value: String::from("("), token_type: TokenType::Symbol }); 
        
    } else if *pos < tokens.len() && tokens[*pos].value == "-=" {
        expr_tokens.push(Token { value: name.clone(), token_type: TokenType::Symbol });
        expr_tokens.push(Token { value: String::from("-"), token_type: TokenType::Symbol });
        expr_tokens.push(Token { value: String::from("("), token_type: TokenType::Symbol });
    } else if *pos < tokens.len() && tokens[*pos].value == "*=" {
        expr_tokens.push(Token { value: name.clone(), token_type: TokenType::Symbol });
        expr_tokens.push(Token { value: String::from("*"), token_type: TokenType::Symbol });
        expr_tokens.push(Token { value: String::from("("), token_type: TokenType::Symbol });
    } else if *pos < tokens.len() && tokens[*pos].value == "/=" {
        expr_tokens.push(Token { value: name.clone(), token_type: TokenType::Symbol });
        expr_tokens.push(Token { value: String::from("/"), token_type: TokenType::Symbol });
        expr_tokens.push(Token { value: String::from("("), token_type: TokenType::Symbol });
    } else {
        panic!("ast: var_change: unknown operator found: {}", tokens[*pos].value);
    }

    *pos += 1;

    while *pos < tokens.len() && tokens[*pos].value != ";" {
        expr_tokens.push(tokens[*pos].clone());
        *pos += 1;
    }
    if *pos < tokens.len() && tokens[*pos].value == ";" { *pos += 1; }

    expr_tokens.push(Token { value: String::from(")"), token_type: TokenType::Symbol });

    return Part::VarSet { 
        var_name: name, 
        value: parse_tree(&expr_tokens) 
    };    
}

fn is_a_function(stre: String, pre_ast: &PreAST) -> bool {
    for func in &pre_ast.functions {
        if func.name == stre {
            return true;
        }
    }
    return false;
}

