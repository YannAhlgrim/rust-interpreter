use std::rc::Rc;

use crate::token::Token;

pub trait NodeTrait {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
    fn as_any(&self) -> &dyn std::any::Any;
}

#[allow(dead_code)]
pub trait StatementTrait: NodeTrait {
    fn statement_node(&self);
}

pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    StringLiteral(StringLiteral),
    Boolean(Boolean),
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
    IfExpression(IfExpression),
    FunctionLiteral(FunctionLiteral),
    CallExpression(CallExpression),
    ArrayLiteral(ArrayLiteral),
    HashLiteral(HashLiteral),
    IndexExpression(IndexExpression),
}

impl NodeTrait for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(i) => i.token_literal(),
            Expression::IntegerLiteral(i) => i.token_literal(),
            Expression::StringLiteral(i) => i.token_literal(),
            Expression::Boolean(i) => i.token_literal(),
            Expression::PrefixExpression(i) => i.token_literal(),
            Expression::InfixExpression(i) => i.token_literal(),
            Expression::IfExpression(i) => i.token_literal(),
            Expression::FunctionLiteral(i) => i.token_literal(),
            Expression::CallExpression(i) => i.token_literal(),
            Expression::ArrayLiteral(i) => i.token_literal(),
            Expression::HashLiteral(i) => i.token_literal(),
            Expression::IndexExpression(i) => i.token_literal(),
        }
    }

    fn string(&self) -> String {
        match self {
            Expression::Identifier(i) => i.string(),
            Expression::IntegerLiteral(i) => i.string(),
            Expression::StringLiteral(i) => i.string(),
            Expression::Boolean(i) => i.string(),
            Expression::PrefixExpression(i) => i.string(),
            Expression::InfixExpression(i) => i.string(),
            Expression::IfExpression(i) => i.string(),
            Expression::FunctionLiteral(i) => i.string(),
            Expression::CallExpression(i) => i.string(),
            Expression::ArrayLiteral(i) => i.string(),
            Expression::HashLiteral(i) => i.string(),
            Expression::IndexExpression(i) => i.string(),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl StatementTrait for ExpressionStatement {
    fn statement_node(&self) {}
}

#[derive(Debug, Default, Clone)]
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
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
        self.value.to_string()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
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
        self.value.to_string()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: Rc<BlockStatement>,
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Debug)]
pub struct StringLiteral {
    pub token: Token,
    pub value: String,
}

impl NodeTrait for StringLiteral {
    fn token_literal(&self) -> String {
        String::from(&self.token.literal)
    }

    fn string(&self) -> String {
        self.value.clone()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct ArrayLiteral {
    pub token: Token,
    pub elements: Vec<Expression>,
}

impl NodeTrait for ArrayLiteral {
    fn token_literal(&self) -> String {
        String::from(&self.token.literal)
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("[");
        let elements: Vec<String> = self.elements.iter().map(|e| e.string()).collect();
        out.push_str(&elements.join(", "));
        out.push_str("]");
        out
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct HashLiteral {
    pub token: Token,
    pub pairs: Vec<(Expression, Expression)>,
}

impl NodeTrait for HashLiteral {
    fn token_literal(&self) -> String {
        String::from(&self.token.literal)
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("{");
        let pairs: Vec<String> = self
            .pairs
            .iter()
            .map(|(k, v)| format!("{}: {}", k.string(), v.string()))
            .collect();
        out.push_str(&pairs.join(", "));
        out.push_str("}");
        out
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct IndexExpression {
    pub token: Token,
    pub left: Box<Expression>,
    pub index: Box<Expression>,
}

impl NodeTrait for IndexExpression {
    fn token_literal(&self) -> String {
        String::from(&self.token.literal)
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("(");
        out.push_str(&self.left.string());
        out.push_str("[");
        out.push_str(&self.index.string());
        out.push_str("])");
        out
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
