use super::token;
use super::token::Token;

struct Lexer {
    input: String,
    position: Option<i32>,
    read_pos: Option<i32>,
    ch: Option<u8>,
}

trait LexerTraits {
    fn read_char(&mut self);
    fn next_token(&self) -> Token;
}

fn new(input_str: String) -> Lexer {
    let mut l: Lexer = Lexer {
        input: input_str,
        position: Some(0),
        read_pos: Some(1),
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

    fn next_token(&self) -> Token {
        let tok = Token {
           type_: String::from(""),
            literal: 
        }

        match self.ch {}
    }
}

fn new_token(token_type: String, ch: &u8) {
        let lit = String::from_utf8_lossy(ch);
        let token = Token {
            type_: token_type,
            literal: lit
        };
    }
