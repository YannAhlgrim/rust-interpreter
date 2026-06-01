use crate::token;

pub trait NodeTrait {
    fn token_literal(&self) -> String;
}

pub trait StatementTrait: NodeTrait {
    fn statement_node(&self);
}

trait ExpressionTrait: NodeTrait {
    fn expression_node(&self);
}
pub struct Expression {
    //
}
impl NodeTrait for Expression {
    fn token_literal(&self) -> String {
        todo!()
    }
}

pub struct Statement {
    //
}
impl NodeTrait for Statement {
    fn token_literal(&self) -> String {
        todo!()
    }
}

impl StatementTrait for Statement {
    fn statement_node(&self) {}
}

pub struct Program {
    pub statements: Vec<Box<dyn StatementTrait>>,
}

impl NodeTrait for Program {
    fn token_literal(&self) -> String {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            String::new()
        }
    }
}

pub struct LetStatement {
    pub token: token::Token,
    pub name: Identifier,
    pub value: Expression,
}

impl NodeTrait for LetStatement {
    fn token_literal(&self) -> String {
        String::from(&self.token.literal)
    }
}

impl StatementTrait for LetStatement {
    fn statement_node(&self) {
        todo!()
    }
}

pub struct Identifier {
    pub token: token::Token,
    pub value: String,
}

impl NodeTrait for Identifier {
    fn token_literal(&self) -> String {
        String::from(&self.token.literal)
    }
}

impl ExpressionTrait for Identifier {
    fn expression_node(&self) {
        todo!()
    }
}
