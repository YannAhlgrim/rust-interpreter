use std::collections::HashMap;

#[derive(Default)]
pub struct Token {
    pub type_: String,
    pub literal: String,
}

pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";
pub const IDENT: &str = "IDENT";
pub const INT: &str = "INT";
pub const ASSIGN: &str = "=";
pub const PLUS: &str = "+";
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";
pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";
pub const FUNCTION: &str = "FUNCTION";
pub const LET: &str = "LET";

pub fn lookup_ident(ident: &str) -> &str {
    match ident {
        "fn" => FUNCTION,
        "let" => LET,
        _ => ILLEGAL,
    }
}
