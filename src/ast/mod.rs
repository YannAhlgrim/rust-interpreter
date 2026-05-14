trait NodeTrait {
    fn token_literal(&self) -> String;
}

trait StatementTrait: NodeTrait {
    fn statement_node(&self);
}

trait ExpressionTrait: NodeTrait {
    fn expression_node(&self);
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

struct Program {
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
