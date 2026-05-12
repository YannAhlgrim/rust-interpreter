use crate::token::lookup_ident;

use super::token::ASSIGN;
use super::token::COMMA;
use super::token::EOF;
use super::token::ILLEGAL;
use super::token::INT;
use super::token::LBRACE;
use super::token::LPAREN;
use super::token::PLUS;
use super::token::RBRACE;
use super::token::RPAREN;
use super::token::SEMICOLON;
use super::token::Token;

pub struct Lexer {
    input: String,
    position: Option<i32>,
    read_pos: Option<i32>,
    ch: Option<u8>,
}

pub trait LexerTraits {
    fn read_char(&mut self);
    fn next_token(&mut self) -> Token;
    fn read_identifier(&mut self) -> String;
    fn skip_whitespace(&mut self);
    fn read_number(&mut self) -> String;
}

pub fn new(input_str: String) -> Lexer {
    let mut l: Lexer = Lexer {
        input: input_str,
        position: Some(-1),
        read_pos: Some(0),
        ch: Some(0),
    };
    l.read_char();
    l
}

impl LexerTraits for Lexer {
    fn read_char(&mut self) {
        let read_pos = usize::try_from(self.read_pos.unwrap());
        if read_pos.unwrap() >= self.input.len() {
            self.ch = Some(0);
        } else {
            let bytes = self.input.as_bytes();
            for (i, &item) in bytes.iter().enumerate() {
                if i == read_pos.unwrap() {
                    self.ch = Some(item);
                }
            }
        }
        self.position = self.read_pos;
        let new_read_pos = self.read_pos.unwrap() + 1;
        self.read_pos = Some(new_read_pos);
    }

    fn next_token(&mut self) -> Token {
        #[allow(unused_assignments)]
        let mut tok = Token::default();
        self.skip_whitespace();
        let lit = self.ch.unwrap();
        let c = char::from(lit);

        match c {
            '\0' => tok = new_token(EOF, lit),
            '=' => tok = new_token(ASSIGN, lit),
            ';' => tok = new_token(SEMICOLON, lit),
            '(' => tok = new_token(LPAREN, lit),
            ')' => tok = new_token(RPAREN, lit),
            ',' => tok = new_token(COMMA, lit),
            '+' => tok = new_token(PLUS, lit),
            '{' => tok = new_token(LBRACE, lit),
            '}' => tok = new_token(RBRACE, lit),
            _ => {
                if c.is_alphabetic() {
                    let lit = self.read_identifier();
                    let tok_type = lookup_ident(&lit);
                    return new_token_from_str(tok_type, lit);
                } else if c.is_ascii_digit() {
                    let lit = self.read_number();
                    return new_token_from_str(INT, lit);
                } else {
                    tok = new_token(ILLEGAL, lit);
                }
            }
        }
        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position.unwrap();
        while self.ch.unwrap().is_ascii_alphabetic() {
            self.read_char();
        }
        let read_pos = self.position.unwrap() as usize;
        let position = position as usize;
        let res = &self.input;
        let res = &res[position..read_pos];
        String::from(res)
    }

    fn skip_whitespace(&mut self) {
        while char::from(self.ch.unwrap()) == ' '
            || char::from(self.ch.unwrap()) == '\n'
            || char::from(self.ch.unwrap()) == '\r'
        {
            self.read_char();
        }
    }

    fn read_number(&mut self) -> String {
        let position = self.position.unwrap();
        while self.ch.unwrap().is_ascii_digit() {
            self.read_char();
        }
        let read_pos = self.position.unwrap() as usize;
        let position = position as usize;
        let res = &self.input;
        let res = &res[position..read_pos];
        String::from(res)
    }
}

fn new_token(token_type: &str, ch: u8) -> Token {
    let lit = String::from(char::from(ch));
    let token_type = String::from(token_type);
    Token {
        type_: token_type,
        literal: lit,
    }
}

fn new_token_from_str(token_type: &str, lit: String) -> Token {
    let token_type = String::from(token_type);
    Token {
        type_: token_type,
        literal: lit,
    }
}
