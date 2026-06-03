use crate::token::lookup_ident;
use crate::token::{Token, TokenType};

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
    fn peek_char(&self) -> u8;
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
        let mut tok = Token {
            type_: TokenType::Illegal,
            literal: String::new(),
        };
        self.skip_whitespace();
        let lit = self.ch.unwrap();
        let c = char::from(lit);

        match c {
            '\0' => tok = new_token(TokenType::Eof, lit),
            '=' => {
                if char::from(self.peek_char()) == '=' {
                    self.read_char();
                    let lit = String::from("==");
                    tok = new_token_from_str(TokenType::Eq, lit);
                } else {
                    tok = new_token(TokenType::Assign, lit);
                }
            }
            ';' => tok = new_token(TokenType::Semicolon, lit),
            '(' => tok = new_token(TokenType::Lparen, lit),
            ')' => tok = new_token(TokenType::Rparen, lit),
            ',' => tok = new_token(TokenType::Comma, lit),
            '+' => tok = new_token(TokenType::Plus, lit),
            '-' => tok = new_token(TokenType::Minus, lit),
            '!' => {
                if char::from(self.peek_char()) == '=' {
                    self.read_char();
                    let lit = String::from("!=");
                    tok = new_token_from_str(TokenType::Neq, lit);
                } else {
                    tok = new_token(TokenType::Bang, lit);
                }
            }
            '/' => tok = new_token(TokenType::Slash, lit),
            '*' => tok = new_token(TokenType::Asterisk, lit),
            '<' => tok = new_token(TokenType::Lt, lit),
            '>' => tok = new_token(TokenType::Gt, lit),
            '{' => tok = new_token(TokenType::Lbrace, lit),
            '}' => tok = new_token(TokenType::Rbrace, lit),
            _ => {
                if c.is_alphabetic() {
                    let lit = self.read_identifier();
                    let tok_type = lookup_ident(&lit);
                    return new_token_from_str(tok_type, lit);
                } else if c.is_ascii_digit() {
                    let lit = self.read_number();
                    return new_token_from_str(TokenType::Int, lit);
                } else {
                    tok = new_token(TokenType::Illegal, lit);
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

    fn peek_char(&self) -> u8 {
        let read_pos = self.read_pos.unwrap() as usize;
        if read_pos >= self.input.len() {
            0
        } else {
            let res = self.input.as_bytes();
            res[read_pos]
        }
    }
}

fn new_token(token_type: TokenType, ch: u8) -> Token {
    let lit = String::from(char::from(ch));
    Token {
        type_: token_type,
        literal: lit,
    }
}

fn new_token_from_str(token_type: TokenType, lit: String) -> Token {
    Token {
        type_: token_type,
        literal: lit,
    }
}
