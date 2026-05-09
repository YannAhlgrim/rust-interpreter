pub struct Token {
    pub type_: String,
    pub literal: String,
}

const ILLEGAL: &str = "ILLEGAL";
const EOF: &str = "EOF";
const IDENT: &str = "IDENT";
const INT: &str = "INT";
const ASSIGN: &str = "=";
const PLUS: &str = "+";
const COMMA: &str = ",";
const SEMICOLON: &str = ";";
const LPAREN: &str = "(";
const RPAREN: &str = ")";
const LBRACE: &str = "{";
const RBRACE: &str = "}";
const FUNCTION: &str = "FUNCTION";
const LET: &str = "LET";
