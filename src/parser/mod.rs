use crate::{
    ast::{self},
    lexer::{self, LexerTraits},
    token::{self, EOF},
};

pub struct Parser {
    lexer: lexer::Lexer,
    cur_token: token::Token,
    peek_token: token::Token,
    errors: Vec<String>,
}

pub trait ParserTrait {
    fn next_token(&mut self);
    fn parse_program(&mut self) -> ast::Program;
    fn parse_statement(&mut self) -> Option<Box<dyn ast::StatementTrait>>;
    fn parse_let_statement(&mut self) -> Option<Box<dyn ast::StatementTrait>>;

    fn cur_token_is(&self, t: String) -> bool;
    fn peek_token_is(&self, t: String) -> bool;
    fn expect_peek(&mut self, t: String) -> bool;
    fn errors(&self) -> &Vec<String>;
    fn peek_error(&mut self, t: String);
}

pub fn new_parser(l: lexer::Lexer) -> Parser {
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
        errors: Vec::new(),
    };
    p.next_token();
    p.next_token();
    p
}

impl ParserTrait for Parser {
    fn next_token(&mut self) {
        self.cur_token = token::Token {
            literal: String::from(&self.peek_token.literal),
            type_: String::from(&self.peek_token.type_),
        };
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program {
            statements: Vec::new(),
        };
        while &self.cur_token.type_ != EOF {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Box<dyn ast::StatementTrait>> {
        match self.cur_token.type_.as_str() {
            token::LET => self.parse_let_statement(),
            _ => None,
        }
    }

    fn cur_token_is(&self, t: String) -> bool {
        self.cur_token.type_ == t
    }

    fn peek_token_is(&self, t: String) -> bool {
        self.peek_token.type_ == t
    }

    fn expect_peek(&mut self, t: String) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            false
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn ast::StatementTrait>> {
        let mut stmt = ast::LetStatement {
            token: token::Token {
                literal: String::from(&self.cur_token.literal),
                type_: String::from(&self.cur_token.type_),
            },
            name: ast::Identifier {
                token: token::Token {
                    literal: String::new(),
                    type_: String::new(),
                },
                value: String::new(),
            },
            value: ast::Expression {},
        };

        if !self.expect_peek(token::IDENT.to_string()) {
            return None;
        }

        stmt.name = ast::Identifier {
            token: token::Token {
                literal: String::from(&self.cur_token.literal),
                type_: String::from(&self.cur_token.type_),
            },
            value: String::from(&self.cur_token.literal),
        };

        if !self.expect_peek(token::ASSIGN.to_string()) {
            return None;
        }

        while !self.cur_token_is(token::SEMICOLON.to_string()) {
            self.next_token();
        }

        Some(Box::from(stmt))
    }

    fn errors(&self) -> &Vec<String> {
        &self.errors
    }

    fn peek_error(&mut self, t: String) {
        let msg = format!(
            "expected next token to be {}, got {} instead",
            t, self.peek_token.type_
        );
        self.errors.push(msg);
    }
}
