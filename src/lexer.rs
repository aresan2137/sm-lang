use std::{fs, process::exit};
use std::process::Command;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TokenType {
	Key,
    CppBlock,
    Number,
    String,
    Scope,
    Symbol,
    Assign,
    Member,
    Bracket,
    Semicolon,
    Comma,
    Value
}

#[derive(Deserialize, Debug, Clone)]
pub struct Token {
    pub value: String,
    #[serde(rename = "type")]
    pub token_type: TokenType,
}

pub fn tokenize(file_path: &str) -> Vec<Token> {
    let lexer_data = Command::new("python").arg("assets/lexer.py").arg(file_path).output().expect("lexer error");

    if !lexer_data.status.success() {
        let err = String::from_utf8_lossy(&lexer_data.stderr);
        panic!("lexer: error: {}", err);
    } 
    
    let lexer_output =  fs::read_to_string("assets/in.json").expect("in.json does't exist");

    return serde_json::from_str(&lexer_output).expect("lexer: error: json problem");
}