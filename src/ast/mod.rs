use crate::token;

trait NodeTrait {
    fn token_literal(&self) -> String;
}

trait StatementTrait: NodeTrait {
    fn statement_node(&self);
}

trait ExpressionTrait: NodeTrait {
    fn expression_node(&self);
}
struct Expression {
    //
}
impl NodeTrait for Expression {
    fn token_literal(&self) -> String {
        todo!()
    }
}

struct Statement {
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
    statements: Vec<Statement>,
}

impl NodeTrait for Program {
    fn token_literal(&self) -> String {
        if self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            String::new()
        }
    }
}

struct LetStatement {
    token: token::Token,
    name: Identifier,
    value: Expression,
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

struct Identifier {
    token: token::Token,
    value: String,
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
