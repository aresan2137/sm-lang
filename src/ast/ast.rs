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
    MethodCall {
        object: Tree,
        method_name: String,
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
    pub name: Vec<Token>,
    pub ret_type: Vec<Token>,
    pub body: Part,
    pub args: Vec<Token>
}

#[derive(Debug)]
pub struct AST {
    pub functions: Vec<FunctionAST>,
    pub glob_inlines: Vec<String>
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

    if t.token_type == TokenType::CppBlock {
        *pos += 1;
        let code = t.value.replace("$$(", "").replace(")$$", "").trim().to_string();
        return Some(Part::CppBlock(code));
    }

    if t.value == "{" {
        *pos += 1;
        let mut inner = Vec::new();
        while *pos < tokens.len() && tokens[*pos].value != "}" {
            if let Some(p) = create_part(tokens, pos, pre_ast, vars) {
                inner.push(p);
            }
        }
        if *pos < tokens.len() { *pos += 1; }
        return Some(Part::Parts { parts: inner });
    }

    if t.value == "ret" {
        *pos += 1;
        if compare_with_incr(tokens, pos, ";") {
            return Some(Part::RetVoid {});
        }
        return Some(Part::Ret { 
            value: tree_with_scaning_until_str(&tokens, pos, ";")
        });
    }

    if t.value == "if" {
        *pos += 1;
        let tree = tree_with_scaning_until_bracket_stack_empty(&tokens, pos);
        return Some(Part::If { value: tree });
    }

    if t.value == "else" {
        *pos += 1;
        return Some(Part::Else {}); 
    }

    if t.value == "while" {
        *pos += 1;
        let tree = tree_with_scaning_until_bracket_stack_empty(&tokens, pos);
        return Some(Part::While { value: tree });
    }

    if t.value == "var" {
        *pos += 1;
        let v_type = parse_type(tokens, pos); 
        let v_name = tokens[*pos].value.clone(); 
        *pos += 1;
        if *pos < tokens.len() && tokens[*pos].value == "=" {
            *pos += 1;
            return Some(Part::Var { var_type: v_type, var_name: v_name, value: tree_with_scaning_until_str(tokens, pos, ";"), is_bodyless: false });
        } else if *pos < tokens.len() && tokens[*pos].value == ";" {            
            *pos += 1;
            return Some(Part::Var { var_type: v_type, var_name: v_name, value: Tree::Leaf(String::from("0")), is_bodyless: true});
        } else {
            panic!("ast: var error at {}", v_name);
        }
    }

    if is_a_function(tokens, *pos) {
        let tree = tree_with_scaning_until_str(tokens, pos, ";");
        match tree {
            Tree::Call { func_name, args } => {
                return Some(Part::FuncCall { func_name, args });
            }
            Tree::MethodCall { object, method_name, args } => {
                return Some(Part::MethodCall { object: *object, method_name, args });
            }
            _ => return None,
        }
    }

    let mut exists = false;
    for v in &vars.vars {
        if v.name == t.value {
            exists = true;
            break;
        }
    }

    if exists {
        *pos += 1;
        return Some(parse_var_change(tokens, pos, &t));
    }

    *pos += 1;
    return None;
}

fn parse_var_change(tokens: &Vec<Token>, pos: &mut usize, t: &Token) -> Part {
    let mut name = t.value.clone();
    
    while *pos < tokens.len() && (tokens[*pos].value == "." || tokens[*pos].value == "[") {
        if tokens[*pos].value == "." {
            name.push('.');
            *pos += 1;
            if *pos < tokens.len() {
                name.push_str(&tokens[*pos].value);
                *pos += 1;
            }
        } else if tokens[*pos].value == "[" {
            name.push('[');
            *pos += 1;
            
            // Zbierz wszystko wewnątrz klamer [ ] (np. indeks, zmienną lub wyrażenie)
            while *pos < tokens.len() && tokens[*pos].value != "]" {
                name.push_str(&tokens[*pos].value);
                *pos += 1;
            }
            
            if *pos < tokens.len() && tokens[*pos].value == "]" {
                name.push(']');
                *pos += 1;
            } else {
                panic!("ast: var_change: expected ']'");
            }
        }
    }

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

fn is_a_function(tokens: &[Token], pos: usize) -> bool {
    let mut i = pos;
    if i >= tokens.len() { return false; }

    i += 1;

    while i < tokens.len() && tokens[i].value == "." {
        i += 1;
        if i < tokens.len() {
            i += 1;
        } else {
            return false;
        }
    }

    if i < tokens.len() && tokens[i].value == "(" {
        return true;
    }

    false
}