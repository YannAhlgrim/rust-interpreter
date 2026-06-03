use crate::token::Token;

pub trait NodeTrait {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

pub trait StatementTrait: NodeTrait {
    fn statement_node(&self);
}

pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
    Boolean(Boolean),
    BlockStatement(BlockStatement),
    IfExpression(IfExpression),
    FunctionLiteral(FunctionLiteral),
    CallExpression(CallExpression),
}

impl NodeTrait for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(i) => i.token_literal(),
            Expression::IntegerLiteral(i) => i.token_literal(),
            Expression::PrefixExpression(i) => i.token_literal(),
            Expression::InfixExpression(i) => i.token_literal(),
            Expression::Boolean(i) => i.token_literal(),
            Expression::BlockStatement(i) => i.token_literal(),
            Expression::IfExpression(i) => i.token_literal(),
            Expression::FunctionLiteral(i) => i.token_literal(),
            Expression::CallExpression(i) => i.token_literal(),
        }
    }

    fn string(&self) -> String {
        match self {
            Expression::Identifier(i) => i.string(),
            Expression::IntegerLiteral(i) => i.string(),
            Expression::PrefixExpression(i) => i.string(),
            Expression::InfixExpression(i) => i.string(),
            Expression::Boolean(i) => i.string(),
            Expression::BlockStatement(i) => i.string(),
            Expression::IfExpression(i) => i.string(),
            Expression::FunctionLiteral(i) => i.string(),
            Expression::CallExpression(i) => i.string(),
        }
    }
}

pub struct Statement;
impl NodeTrait for Statement {
    fn token_literal(&self) -> String {
        todo!()
    }

    fn string(&self) -> String {
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

    fn string(&self) -> String {
        let mut out = String::new();
        for stmt in &self.statements {
            out.push_str(&stmt.string());
        }
        out
    }
}

pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression,
}

pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Expression,
}

pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Expression,
}

impl NodeTrait for LetStatement {
    fn token_literal(&self) -> String {
        String::from(&self.token.literal)
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push_str(" ");
        out.push_str(&self.name.string());
        out.push_str(" = ");
        out.push_str(&self.value.string());
        out.push_str(";");
        out
    }
}

impl StatementTrait for LetStatement {
    fn statement_node(&self) {}
}

impl NodeTrait for ReturnStatement {
    fn token_literal(&self) -> String {
        String::from(&self.token.literal)
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push_str(" ");
        out.push_str(&self.return_value.string());
        out.push_str(";");
        out
    }
}

impl StatementTrait for ReturnStatement {
    fn statement_node(&self) {}
}

impl NodeTrait for ExpressionStatement {
    fn token_literal(&self) -> String {
        String::from(&self.token.literal)
    }

    fn string(&self) -> String {
        self.expression.string()
    }
}

impl StatementTrait for ExpressionStatement {
    fn statement_node(&self) {}
}

#[derive(Debug, Default)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl NodeTrait for Identifier {
    fn token_literal(&self) -> String {
        String::from(&self.token.literal)
    }

    fn string(&self) -> String {
        self.value.clone()
    }
}

#[derive(Debug)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl NodeTrait for IntegerLiteral {
    fn token_literal(&self) -> String {
        String::from(&self.token.literal)
    }

    fn string(&self) -> String {
        self.token.literal.clone()
    }
}

pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<Expression>,
}

impl NodeTrait for PrefixExpression {
    fn token_literal(&self) -> String {
        String::from(&self.token.literal)
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.operator);
        out.push_str(&self.right.string());
        out
    }
}

pub struct InfixExpression {
    pub token: Token,
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}

impl NodeTrait for InfixExpression {
    fn token_literal(&self) -> String {
        String::from(&self.token.literal)
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("(");
        out.push_str(&self.left.string());
        out.push_str(" ");
        out.push_str(&self.operator);
        out.push_str(" ");
        out.push_str(&self.right.string());
        out.push_str(")");
        out
    }
}

pub struct Boolean {
    pub token: Token,
    pub value: bool,
}

impl NodeTrait for Boolean {
    fn token_literal(&self) -> String {
        String::from(&self.token.literal)
    }

    fn string(&self) -> String {
        self.token.literal.clone()
    }
}

pub struct IfExpression {
    pub token: Token,
    pub condition: Box<Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl NodeTrait for IfExpression {
    fn token_literal(&self) -> String {
        String::from(&self.token.literal)
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("if");
        out.push_str(&self.condition.string());
        out.push_str(" ");
        out.push_str(&self.consequence.string());
        if let Some(alt) = &self.alternative {
            out.push_str("else ");
            out.push_str(&alt.string());
        }
        out
    }
}

pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Box<dyn StatementTrait>>,
}

impl NodeTrait for BlockStatement {
    fn token_literal(&self) -> String {
        String::from(&self.token.literal)
    }

    fn string(&self) -> String {
        let mut out = String::new();
        for stmt in &self.statements {
            out.push_str(&stmt.string());
        }
        out
    }
}

pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

impl NodeTrait for FunctionLiteral {
    fn token_literal(&self) -> String {
        String::from(&self.token.literal)
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push_str("(");
        let params: Vec<String> = self.parameters.iter().map(|p| p.string()).collect();
        out.push_str(&params.join(", "));
        out.push_str(") ");
        out.push_str(&self.body.string());
        out
    }
}

pub struct CallExpression {
    pub token: Token,
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
}

impl NodeTrait for CallExpression {
    fn token_literal(&self) -> String {
        String::from(&self.token.literal)
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.function.string());
        out.push_str("(");
        let args: Vec<String> = self.arguments.iter().map(|a| a.string()).collect();
        out.push_str(&args.join(", "));
        out.push_str(")");
        out
    }
}
