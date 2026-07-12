use std::rc::Rc;

use crate::{
    ast::{self},
    lexer::{self, LexerTraits},
    token::{self, TokenType},
};
use std::collections::HashMap;

type PrefixParseFn = fn(&mut Parser) -> Option<ast::Expression>;
type InfixParseFn = fn(&mut Parser, ast::Expression) -> Option<ast::Expression>;

pub struct Parser {
    lexer: lexer::Lexer,
    cur_token: token::Token,
    peek_token: token::Token,
    errors: Vec<String>,
    prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,
    infix_parse_fns: HashMap<TokenType, InfixParseFn>,
}

pub fn new_parser(l: lexer::Lexer) -> Parser {
    let mut p = Parser {
        lexer: l,
        cur_token: token::Token {
            literal: String::new(),
            type_: TokenType::Illegal,
        },
        peek_token: token::Token {
            literal: String::new(),
            type_: TokenType::Illegal,
        },
        errors: Vec::new(),
        prefix_parse_fns: HashMap::new(),
        infix_parse_fns: HashMap::new(),
    };
    p.register_prefix(TokenType::Ident, Parser::parse_identifier);
    p.register_prefix(TokenType::Int, Parser::parse_integer_literal);
    p.register_prefix(TokenType::Bang, Parser::parse_prefix_expression);
    p.register_prefix(TokenType::Minus, Parser::parse_prefix_expression);
    p.register_infix(TokenType::Plus, Parser::parse_infix_expression);
    p.register_infix(TokenType::Minus, Parser::parse_infix_expression);
    p.register_infix(TokenType::Slash, Parser::parse_infix_expression);
    p.register_infix(TokenType::Asterisk, Parser::parse_infix_expression);
    p.register_infix(TokenType::Eq, Parser::parse_infix_expression);
    p.register_infix(TokenType::Neq, Parser::parse_infix_expression);
    p.register_infix(TokenType::Lt, Parser::parse_infix_expression);
    p.register_infix(TokenType::Gt, Parser::parse_infix_expression);
    p.register_prefix(TokenType::True, Parser::parse_boolean);
    p.register_prefix(TokenType::False, Parser::parse_boolean);
    p.register_prefix(TokenType::Lparen, Parser::parse_grouped_expression);
    p.register_prefix(TokenType::If, Parser::parse_if_expression);
    p.register_prefix(TokenType::Function, Parser::parse_function_literal);
    p.register_infix(TokenType::Lparen, Parser::parse_call_expression);
    p.next_token();
    p.next_token();
    p
}

impl Parser {
    fn register_prefix(&mut self, t: TokenType, f: PrefixParseFn) {
        self.prefix_parse_fns.insert(t, f);
    }

    fn register_infix(&mut self, t: TokenType, f: InfixParseFn) {
        self.infix_parse_fns.insert(t, f);
    }

    fn parse_identifier(&mut self) -> Option<ast::Expression> {
        Some(ast::Expression::Identifier(ast::Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        }))
    }

    fn parse_integer_literal(&mut self) -> Option<ast::Expression> {
        let value: i64 = self.cur_token.literal.parse().ok()?;
        Some(ast::Expression::IntegerLiteral(ast::IntegerLiteral {
            token: self.cur_token.clone(),
            value,
        }))
    }

    fn no_prefix_parse_fn_error(&mut self) {
        let msg = format!(
            "no prefix parse function for {} found",
            self.cur_token.type_
        );
        self.errors.push(msg);
    }

    fn peek_precedence(&self) -> Precedence {
        match self.peek_token.type_ {
            TokenType::Eq | TokenType::Neq => Precedence::Equals,
            TokenType::Lt | TokenType::Gt => Precedence::Lessgreater,
            TokenType::Plus | TokenType::Minus => Precedence::Sum,
            TokenType::Slash | TokenType::Asterisk => Precedence::Product,
            TokenType::Lparen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }

    fn cur_precedence(&self) -> Precedence {
        match self.cur_token.type_ {
            TokenType::Eq | TokenType::Neq => Precedence::Equals,
            TokenType::Lt | TokenType::Gt => Precedence::Lessgreater,
            TokenType::Plus | TokenType::Minus => Precedence::Sum,
            TokenType::Slash | TokenType::Asterisk => Precedence::Product,
            TokenType::Lparen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }

    fn next_token(&mut self) {
        self.cur_token = token::Token {
            literal: String::from(&self.peek_token.literal),
            type_: self.peek_token.type_,
        };
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program {
            statements: Vec::new(),
        };
        while self.cur_token.type_ != TokenType::Eof {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Box<dyn ast::StatementTrait>> {
        match self.cur_token.type_ {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn cur_token_is(&self, t: TokenType) -> bool {
        self.cur_token.type_ == t
    }

    fn peek_token_is(&self, t: TokenType) -> bool {
        self.peek_token.type_ == t
    }

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            self.peek_error(t);
            false
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn ast::StatementTrait>> {
        let mut stmt = ast::LetStatement {
            token: token::Token {
                literal: String::from(&self.cur_token.literal),
                type_: self.cur_token.type_.clone(),
            },
            name: ast::Identifier {
                token: token::Token {
                    literal: String::new(),
                    type_: TokenType::Illegal,
                },
                value: String::new(),
            },
            value: ast::Expression::Identifier(ast::Identifier {
                token: token::Token {
                    literal: String::new(),
                    type_: TokenType::Illegal,
                },
                value: String::new(),
            }),
        };

        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        stmt.name = ast::Identifier {
            token: token::Token {
                literal: String::from(&self.cur_token.literal),
                type_: self.cur_token.type_.clone(),
            },
            value: String::from(&self.cur_token.literal),
        };

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        self.next_token();

        stmt.value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Box::new(stmt))
    }

    fn parse_return_statement(&mut self) -> Option<Box<dyn ast::StatementTrait>> {
        let mut stmt = ast::ReturnStatement {
            token: token::Token {
                literal: String::from(&self.cur_token.literal),
                type_: self.cur_token.type_.clone(),
            },
            return_value: ast::Expression::Identifier(ast::Identifier {
                token: token::Token {
                    literal: String::new(),
                    type_: TokenType::Illegal,
                },
                value: String::new(),
            }),
        };

        self.next_token();

        stmt.return_value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Box::new(stmt))
    }

    fn parse_expression_statement(&mut self) -> Option<Box<dyn ast::StatementTrait>> {
        let stmt = ast::ExpressionStatement {
            token: token::Token {
                literal: String::from(&self.cur_token.literal),
                type_: self.cur_token.type_,
            },
            expression: self.parse_expression(Precedence::Lowest)?,
        };

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Box::new(stmt))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<ast::Expression> {
        let prefix = self.prefix_parse_fns.get(&self.cur_token.type_).copied();
        let mut left = match prefix {
            Some(f) => f(self)?,
            None => {
                self.no_prefix_parse_fn_error();
                return None;
            }
        };
        while !self.peek_token_is(TokenType::Semicolon) && precedence < self.peek_precedence() {
            let infix = self.infix_parse_fns.get(&self.peek_token.type_).copied();
            match infix {
                Some(f) => {
                    self.next_token();
                    left = f(self, left)?;
                }
                None => return Some(left),
            }
        }
        Some(left)
    }

    fn parse_prefix_expression(&mut self) -> Option<ast::Expression> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();
        self.next_token();
        let right = self.parse_expression(Precedence::Prefix)?;
        Some(ast::Expression::PrefixExpression(ast::PrefixExpression {
            token,
            operator,
            right: Box::new(right),
        }))
    }

    fn parse_infix_expression(&mut self, left: ast::Expression) -> Option<ast::Expression> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();
        let precedence = self.cur_precedence();
        self.next_token();
        let right = self.parse_expression(precedence)?;
        Some(ast::Expression::InfixExpression(ast::InfixExpression {
            token,
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }))
    }

    fn parse_boolean(&mut self) -> Option<ast::Expression> {
        Some(ast::Expression::Boolean(ast::Boolean {
            token: self.cur_token.clone(),
            value: self.cur_token_is(TokenType::True),
        }))
    }

    fn parse_grouped_expression(&mut self) -> Option<ast::Expression> {
        self.next_token();
        let exp = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_peek(TokenType::Rparen) {
            return None;
        }
        Some(exp)
    }

    fn parse_if_expression(&mut self) -> Option<ast::Expression> {
        let mut expression = ast::IfExpression {
            token: self.cur_token.clone(),
            condition: Box::new(ast::Expression::Identifier(ast::Identifier {
                token: token::Token {
                    literal: String::new(),
                    type_: TokenType::Illegal,
                },
                value: String::new(),
            })),
            consequence: ast::BlockStatement {
                token: token::Token {
                    literal: String::new(),
                    type_: TokenType::Illegal,
                },
                statements: Vec::new(),
            },
            alternative: None,
        };
        if !self.expect_peek(TokenType::Lparen) {
            return None;
        }
        self.next_token();
        expression.condition = Box::new(self.parse_expression(Precedence::Lowest)?);
        if !self.expect_peek(TokenType::Rparen) {
            return None;
        }
        if !self.expect_peek(TokenType::Lbrace) {
            return None;
        }
        expression.consequence = self.parse_block_statement()?;

        if self.peek_token_is(TokenType::Else) {
            self.next_token();
            if !self.expect_peek(TokenType::Lbrace) {
                return None;
            }
            expression.alternative = Some(self.parse_block_statement()?);
        }

        return Some(ast::Expression::IfExpression(expression));
    }

    fn parse_block_statement(&mut self) -> Option<ast::BlockStatement> {
        let mut block = ast::BlockStatement {
            token: self.cur_token.clone(),
            statements: Vec::new(),
        };
        self.next_token();
        while !self.cur_token_is(TokenType::Rbrace) && !self.cur_token_is(TokenType::Eof) {
            if let Some(stmt) = self.parse_statement() {
                block.statements.push(stmt);
            }
            self.next_token();
        }
        Some(block)
    }

    fn parse_function_literal(&mut self) -> Option<ast::Expression> {
        let token = self.cur_token.clone();
        if !self.expect_peek(TokenType::Lparen) {
            return None;
        }
        let parameters = self.parse_function_parameters()?;
        if !self.expect_peek(TokenType::Lbrace) {
            return None;
        }
        let body = Rc::new(self.parse_block_statement()?);
        Some(ast::Expression::FunctionLiteral(ast::FunctionLiteral {
            token,
            parameters,
            body,
        }))
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<ast::Identifier>> {
        let mut identifiers = Vec::new();
        if self.peek_token_is(TokenType::Rparen) {
            self.next_token();
            return Some(identifiers);
        }
        self.next_token();
        identifiers.push(ast::Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        });
        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
            identifiers.push(ast::Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            });
        }
        if !self.expect_peek(TokenType::Rparen) {
            return None;
        }
        Some(identifiers)
    }

    fn parse_call_expression(&mut self, function: ast::Expression) -> Option<ast::Expression> {
        let token = self.cur_token.clone();
        let arguments = self.parse_call_arguments()?;
        Some(ast::Expression::CallExpression(ast::CallExpression {
            token,
            function: Box::new(function),
            arguments,
        }))
    }

    fn parse_call_arguments(&mut self) -> Option<Vec<ast::Expression>> {
        let mut args = Vec::new();
        if self.peek_token_is(TokenType::Rparen) {
            self.next_token();
            return Some(args);
        }
        self.next_token();
        args.push(self.parse_expression(Precedence::Lowest)?);
        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
            args.push(self.parse_expression(Precedence::Lowest)?);
        }
        if !self.expect_peek(TokenType::Rparen) {
            return None;
        }
        Some(args)
    }

    pub fn errors(&self) -> &Vec<String> {
        &self.errors
    }

    fn peek_error(&mut self, t: TokenType) {
        let msg = format!(
            "expected next token to be {}, got {} instead",
            t, self.peek_token.type_
        );
        self.errors.push(msg);
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Equals,
    Lessgreater,
    Sum,
    Product,
    Prefix,
    Call,
}
