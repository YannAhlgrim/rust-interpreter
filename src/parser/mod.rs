use crate::{
    ast,
    lexer::{self, LexerTraits},
    token,
};

pub struct Parser {
    lexer: lexer::Lexer,
    cur_token: token::Token,
    peek_token: token::Token,
}

trait PaserTrait {
    fn next_token(&mut self);
    fn parse_program(&self) -> ast::Program;
}

pub fn new(l: lexer::Lexer) -> Parser {
    let mut p: Parser = Parser {
        lexer: l,
        cur_token: token::Token {
            literal: String::new(),
            type_: String::new(),
        },
        peek_token: token::Token {
            literal: String::new(),
            type_: String::new(),
        },
    };
    p.next_token();
    p.next_token();
    p
}

impl PaserTrait for Parser {
    fn next_token(&mut self) {
        self.cur_token = token::Token {
            literal: String::from(&self.peek_token.literal),
            type_: String::from(&self.peek_token.type_),
        };
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(&self) -> ast::Program {
        todo!()
    }
}
