use crate::ast::Program;
use crate::ast::{self};
use crate::object::{self, Null, Object};

const NULL: Null = Null;
const BOOL_TRUE: object::Boolean = object::Boolean { value: true };
const BOOL_FALSE: object::Boolean = object::Boolean { value: false };

fn eval(node: Box<dyn ast::NodeTrait>) -> Option<Box<dyn object::Object>> {
    let node = node.as_ref();
    if let Some(stmt) = node.as_any().downcast_ref::<ast::ExpressionStatement>() {
        return eval_expression(&stmt.expression);
    }
    if let Some(stmt) = node.as_any().downcast_ref::<ast::LetStatement>() {
        return None;
    }
    if let Some(stmt) = node.as_any().downcast_ref::<ast::ReturnStatement>() {
        return eval_expression(&stmt.return_value);
    }
    if let Some(expr) = node.as_any().downcast_ref::<ast::Expression>() {
        return eval_expression(expr);
    }
    if let Some(expr) = node.as_any().downcast_ref::<ast::PrefixExpression>() {
        let right = &eval(expr.right);
        return eval_prefix_expression(expr.operator, right);
    }
    None
}

pub fn eval_program(program: Program) -> Option<Box<dyn object::Object>> {
    let mut res: Option<Box<dyn object::Object>> = None;
    for stmt in program.statements {
        res = eval(stmt)
    }
    res
}

fn eval_expression(expression: &ast::Expression) -> Option<Box<dyn object::Object>> {
    match expression {
        ast::Expression::IntegerLiteral(int) => {
            Some(Box::new(object::Integer { value: int.value }))
        }
        ast::Expression::Boolean(b) => Some(Box::new(native_bool_to_bool_object(b.value))),
        _ => None,
    }
}

fn eval_prefix_expression(
    operator: String,
    right: &Option<Box<dyn object::Object>>,
) -> Option<Box<dyn object::Object>> {
    match operator.as_str() {
        "!" => eval_bang_operator_expression(right),
        _ => Some(Box::new(NULL)),
    }
}

fn eval_bang_operator_expression(
    right: &Option<Box<dyn object::Object>>,
) -> Option<Box<dyn object::Object>> {
    let bool_true = Box::new(BOOL_TRUE);
    let bool_false = Box::new(BOOL_FALSE);
    let null = Box::new(NULL);

    match right..unwrap() {
        bool_true => Some(bool_false),
        bool_false => Some(bool_true),
        null => Some(bool_true),
        _ => Some(bool_false),
    }
}

fn native_bool_to_bool_object(input: bool) -> object::Boolean {
    if input { BOOL_TRUE } else { BOOL_FALSE }
}
