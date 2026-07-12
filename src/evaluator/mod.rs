use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::{self, NodeTrait};
use crate::object::{self, Environment, Hashable, Null, Object, ObjectType};

const NULL: Null = Null;
const BOOL_TRUE: object::Boolean = object::Boolean { value: true };
const BOOL_FALSE: object::Boolean = object::Boolean { value: false };

pub fn eval_program(
    program: ast::Program,
    env: Rc<RefCell<Environment>>,
) -> Option<Box<dyn Object>> {
    let mut result: Option<Box<dyn Object>> = None;

    for stmt in program.statements {
        result = eval(stmt.as_ref(), Rc::clone(&env));

        if let Some(ref obj) = result {
            if obj.object_type() == ObjectType::RETURN_VALUE {
                return Some(unwrap_return_value(obj.clone_box()));
            }
            if obj.object_type() == ObjectType::ERROR {
                return result;
            }
        }
    }

    result
}

fn eval(node: &dyn NodeTrait, env: Rc<RefCell<Environment>>) -> Option<Box<dyn Object>> {
    if let Some(stmt) = node.as_any().downcast_ref::<ast::ExpressionStatement>() {
        return eval_expression(&stmt.expression, env);
    }
    if let Some(stmt) = node.as_any().downcast_ref::<ast::LetStatement>() {
        let value = eval_expression(&stmt.value, Rc::clone(&env))?;
        if is_error(value.as_ref()) {
            return Some(value);
        }
        env.borrow_mut().set(&stmt.name.value, value);
        return Some(Box::new(NULL));
    }
    if let Some(stmt) = node.as_any().downcast_ref::<ast::ReturnStatement>() {
        let value = eval_expression(&stmt.return_value, Rc::clone(&env))?;
        if is_error(value.as_ref()) {
            return Some(value);
        }
        return Some(Box::new(object::ReturnValue { value }));
    }
    if let Some(block) = node.as_any().downcast_ref::<ast::BlockStatement>() {
        return eval_block_statement(block, env);
    }

    None
}

fn eval_block_statement(
    block: &ast::BlockStatement,
    env: Rc<RefCell<Environment>>,
) -> Option<Box<dyn Object>> {
    let mut result: Option<Box<dyn Object>> = None;

    for stmt in &block.statements {
        result = eval(stmt.as_ref(), Rc::clone(&env));

        if let Some(ref obj) = result {
            let obj_type = obj.object_type();
            if obj_type == ObjectType::RETURN_VALUE || obj_type == ObjectType::ERROR {
                return result;
            }
        }
    }

    result
}

fn eval_expression(
    expression: &ast::Expression,
    env: Rc<RefCell<Environment>>,
) -> Option<Box<dyn Object>> {
    match expression {
        ast::Expression::Identifier(ident) => eval_identifier(ident, env),
        ast::Expression::IntegerLiteral(int) => {
            Some(Box::new(object::Integer { value: int.value }))
        }
        ast::Expression::StringLiteral(s) => Some(Box::new(object::StringObj {
            value: s.value.clone(),
        })),
        ast::Expression::Boolean(b) => Some(Box::new(native_bool_to_bool_object(b.value))),
        ast::Expression::PrefixExpression(expr) => {
            let right = eval_expression(expr.right.as_ref(), Rc::clone(&env))?;
            if is_error(right.as_ref()) {
                return Some(right);
            }
            eval_prefix_expression(&expr.operator, right)
        }
        ast::Expression::InfixExpression(expr) => {
            let left = eval_expression(expr.left.as_ref(), Rc::clone(&env))?;
            if is_error(left.as_ref()) {
                return Some(left);
            }
            let right = eval_expression(expr.right.as_ref(), Rc::clone(&env))?;
            if is_error(right.as_ref()) {
                return Some(right);
            }
            eval_infix_expression(&expr.operator, left, right)
        }
        ast::Expression::IfExpression(expr) => eval_if_expression(expr, env),
        ast::Expression::FunctionLiteral(func) => Some(Box::new(object::Function {
            parameters: func.parameters.clone(),
            body: Rc::clone(&func.body),
            env: Rc::clone(&env),
        })),
        ast::Expression::CallExpression(call) => {
            let function = eval_expression(call.function.as_ref(), Rc::clone(&env))?;
            if is_error(function.as_ref()) {
                return Some(function);
            }
            let args = eval_expressions(&call.arguments, Rc::clone(&env))?;
            if args.len() == 1 && is_error(args[0].as_ref()) {
                return Some(args.into_iter().next().unwrap());
            }
            apply_function(function, args)
        }
        ast::Expression::ArrayLiteral(arr) => {
            let elements = eval_expressions(&arr.elements, Rc::clone(&env))?;
            if elements.len() == 1 && is_error(elements[0].as_ref()) {
                return Some(elements.into_iter().next().unwrap());
            }
            Some(Box::new(object::Array { elements }))
        }
        ast::Expression::HashLiteral(hash) => eval_hash_literal(hash, env),
        ast::Expression::IndexExpression(idx) => {
            let left = eval_expression(idx.left.as_ref(), Rc::clone(&env))?;
            if is_error(left.as_ref()) {
                return Some(left);
            }
            let index = eval_expression(idx.index.as_ref(), Rc::clone(&env))?;
            if is_error(index.as_ref()) {
                return Some(index);
            }
            Some(eval_index_expression(left, index))
        }
    }
}

fn eval_identifier(
    ident: &ast::Identifier,
    env: Rc<RefCell<Environment>>,
) -> Option<Box<dyn Object>> {
    if let Some(value) = env.borrow().get(&ident.value) {
        return Some(value);
    }
    Some(new_error(format!("identifier not found: {}", ident.value)))
}

fn eval_expressions(
    expressions: &[ast::Expression],
    env: Rc<RefCell<Environment>>,
) -> Option<Vec<Box<dyn Object>>> {
    let mut result = Vec::new();

    for expr in expressions {
        let evaluated = eval_expression(expr, Rc::clone(&env))?;
        if is_error(evaluated.as_ref()) {
            return Some(vec![evaluated]);
        }
        result.push(evaluated);
    }

    Some(result)
}

fn apply_function(
    function: Box<dyn Object>,
    args: Vec<Box<dyn Object>>,
) -> Option<Box<dyn Object>> {
    if let Some(func) = function.as_any().downcast_ref::<object::Function>() {
        let extended_env = extend_function_env(func, args);
        let evaluated = eval_block_statement(&func.body, Rc::clone(&extended_env));

        if let Some(obj) = evaluated {
            if obj.object_type() == ObjectType::RETURN_VALUE {
                return Some(unwrap_return_value(obj));
            }
            return Some(obj);
        }
        return Some(Box::new(NULL));
    }

    if let Some(builtin) = function.as_any().downcast_ref::<object::Builtin>() {
        return Some((builtin.func)(args));
    }

    Some(new_error(format!(
        "not a function: {:?}",
        function.object_type()
    )))
}

fn extend_function_env(
    func: &object::Function,
    args: Vec<Box<dyn Object>>,
) -> Rc<RefCell<Environment>> {
    let mut env = Environment::from_outer(Rc::clone(&func.env));

    for (i, param) in func.parameters.iter().enumerate() {
        if let Some(arg) = args.get(i) {
            env.set(&param.value, arg.clone_box());
        }
    }

    Rc::new(RefCell::new(env))
}

fn eval_prefix_expression(
    operator: &str,
    right: Box<dyn Object>,
) -> Option<Box<dyn Object>> {
    match operator {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_prefix_operator_expression(right),
        _ => Some(new_error(format!("unknown operator: {}{:?}", operator, right.object_type()))),
    }
}

fn eval_bang_operator_expression(right: Box<dyn Object>) -> Option<Box<dyn Object>> {
    match right.object_type() {
        ObjectType::BOOLEAN => {
            if let Some(b) = right.as_any().downcast_ref::<object::Boolean>() {
                Some(Box::new(native_bool_to_bool_object(!b.value)))
            } else {
                Some(Box::new(NULL))
            }
        }
        ObjectType::NULL => Some(Box::new(BOOL_TRUE)),
        _ => Some(Box::new(BOOL_FALSE)),
    }
}

fn eval_minus_prefix_operator_expression(right: Box<dyn Object>) -> Option<Box<dyn Object>> {
    if let Some(int) = right.as_any().downcast_ref::<object::Integer>() {
        return Some(Box::new(object::Integer { value: -int.value }));
    }
    Some(new_error(format!(
        "unknown operator: -{:?}",
        right.object_type()
    )))
}

fn eval_infix_expression(
    operator: &str,
    left: Box<dyn Object>,
    right: Box<dyn Object>,
) -> Option<Box<dyn Object>> {
    if left.object_type() == ObjectType::INTEGER && right.object_type() == ObjectType::INTEGER {
        return eval_integer_infix_expression(operator, left, right);
    }

    if left.object_type() == ObjectType::BOOLEAN && right.object_type() == ObjectType::BOOLEAN {
        return eval_boolean_infix_expression(operator, left, right);
    }

    if left.object_type() == ObjectType::STRING && right.object_type() == ObjectType::STRING {
        return eval_string_infix_expression(operator, left, right);
    }

    if left.object_type() != right.object_type() {
        return Some(new_error(format!(
            "type mismatch: {:?} {} {:?}",
            left.object_type(),
            operator,
            right.object_type()
        )));
    }

    Some(new_error(format!(
        "unknown operator: {:?} {} {:?}",
        left.object_type(),
        operator,
        right.object_type()
    )))
}

fn eval_integer_infix_expression(
    operator: &str,
    left: Box<dyn Object>,
    right: Box<dyn Object>,
) -> Option<Box<dyn Object>> {
    let left_val = left.as_any().downcast_ref::<object::Integer>().unwrap().value;
    let right_val = right
        .as_any()
        .downcast_ref::<object::Integer>()
        .unwrap()
        .value;

    match operator {
        "+" => Some(Box::new(object::Integer {
            value: left_val + right_val,
        })),
        "-" => Some(Box::new(object::Integer {
            value: left_val - right_val,
        })),
        "*" => Some(Box::new(object::Integer {
            value: left_val * right_val,
        })),
        "/" => Some(Box::new(object::Integer {
            value: left_val / right_val,
        })),
        "<" => Some(Box::new(native_bool_to_bool_object(left_val < right_val))),
        ">" => Some(Box::new(native_bool_to_bool_object(left_val > right_val))),
        "==" => Some(Box::new(native_bool_to_bool_object(left_val == right_val))),
        "!=" => Some(Box::new(native_bool_to_bool_object(left_val != right_val))),
        _ => Some(new_error(format!(
            "unknown operator: {:?} {} {:?}",
            left.object_type(),
            operator,
            right.object_type()
        ))),
    }
}

fn eval_string_infix_expression(
    operator: &str,
    left: Box<dyn Object>,
    right: Box<dyn Object>,
) -> Option<Box<dyn Object>> {
    let left_val = left
        .as_any()
        .downcast_ref::<object::StringObj>()
        .unwrap()
        .value
        .clone();
    let right_val = right
        .as_any()
        .downcast_ref::<object::StringObj>()
        .unwrap()
        .value
        .clone();

    match operator {
        "+" => Some(Box::new(object::StringObj {
            value: format!("{}{}", left_val, right_val),
        })),
        _ => Some(new_error(format!(
            "unknown operator: {:?} {} {:?}",
            left.object_type(),
            operator,
            right.object_type()
        ))),
    }
}

fn eval_boolean_infix_expression(
    operator: &str,
    left: Box<dyn Object>,
    right: Box<dyn Object>,
) -> Option<Box<dyn Object>> {
    let left_val = left.as_any().downcast_ref::<object::Boolean>().unwrap().value;
    let right_val = right
        .as_any()
        .downcast_ref::<object::Boolean>()
        .unwrap()
        .value;

    match operator {
        "==" => Some(Box::new(native_bool_to_bool_object(left_val == right_val))),
        "!=" => Some(Box::new(native_bool_to_bool_object(left_val != right_val))),
        _ => Some(new_error(format!(
            "unknown operator: {:?} {} {:?}",
            left.object_type(),
            operator,
            right.object_type()
        ))),
    }
}

fn eval_hash_literal(
    hash: &ast::HashLiteral,
    env: Rc<RefCell<Environment>>,
) -> Option<Box<dyn Object>> {
    let mut pairs = HashMap::new();

    for (k, v) in &hash.pairs {
        let key = eval_expression(k, Rc::clone(&env))?;
        if is_error(key.as_ref()) {
            return Some(key);
        }
        let value = eval_expression(v, Rc::clone(&env))?;
        if is_error(value.as_ref()) {
            return Some(value);
        }

        let hash_key = match key.object_type() {
            ObjectType::INTEGER => {
                key.as_any()
                    .downcast_ref::<object::Integer>()
                    .unwrap()
                    .hash_key()
            }
            ObjectType::BOOLEAN => {
                key.as_any()
                    .downcast_ref::<object::Boolean>()
                    .unwrap()
                    .hash_key()
            }
            ObjectType::STRING => {
                key.as_any()
                    .downcast_ref::<object::StringObj>()
                    .unwrap()
                    .hash_key()
            }
            _ => return Some(new_error(format!("unusable as hash key: {:?}", key.object_type()))),
        };

        pairs.insert(
            hash_key,
            object::HashPair {
                key: key.clone_box(),
                value: value.clone_box(),
            },
        );
    }

    Some(Box::new(object::Hash { pairs }))
}

fn eval_index_expression(left: Box<dyn Object>, index: Box<dyn Object>) -> Box<dyn Object> {
    match (left.object_type(), index.object_type()) {
        (ObjectType::ARRAY, ObjectType::INTEGER) => eval_array_index_expression(left, index),
        (ObjectType::HASH, _) => eval_hash_index_expression(left, index),
        (ObjectType::STRING, ObjectType::INTEGER) => eval_string_index_expression(left, index),
        _ => new_error(format!(
            "index operator not supported: {:?}[{:?}]",
            left.object_type(),
            index.object_type()
        )),
    }
}

fn eval_array_index_expression(
    array: Box<dyn Object>,
    index: Box<dyn Object>,
) -> Box<dyn Object> {
    let arr = array.as_any().downcast_ref::<object::Array>().unwrap();
    let idx = index.as_any().downcast_ref::<object::Integer>().unwrap().value;
    let max = arr.elements.len() as i64 - 1;
    if idx < 0 || idx > max {
        return Box::new(NULL);
    }
    arr.elements[idx as usize].clone_box()
}

fn eval_hash_index_expression(
    hash: Box<dyn Object>,
    index: Box<dyn Object>,
) -> Box<dyn Object> {
    let hash_obj = hash.as_any().downcast_ref::<object::Hash>().unwrap();

    let key = match index.object_type() {
        ObjectType::INTEGER => index
            .as_any()
            .downcast_ref::<object::Integer>()
            .unwrap()
            .hash_key(),
        ObjectType::BOOLEAN => index
            .as_any()
            .downcast_ref::<object::Boolean>()
            .unwrap()
            .hash_key(),
        ObjectType::STRING => index
            .as_any()
            .downcast_ref::<object::StringObj>()
            .unwrap()
            .hash_key(),
        _ => return new_error(format!("unusable as hash key: {:?}", index.object_type())),
    };

    match hash_obj.pairs.get(&key) {
        Some(pair) => pair.value.clone_box(),
        None => Box::new(NULL),
    }
}

fn eval_string_index_expression(
    string: Box<dyn Object>,
    index: Box<dyn Object>,
) -> Box<dyn Object> {
    let s = string.as_any().downcast_ref::<object::StringObj>().unwrap();
    let idx = index.as_any().downcast_ref::<object::Integer>().unwrap().value;
    let max = s.value.len() as i64 - 1;
    if idx < 0 || idx > max {
        return Box::new(NULL);
    }
    Box::new(object::StringObj {
        value: s.value.chars().nth(idx as usize).unwrap().to_string(),
    })
}

fn eval_if_expression(
    expr: &ast::IfExpression,
    env: Rc<RefCell<Environment>>,
) -> Option<Box<dyn Object>> {
    let condition = eval_expression(expr.condition.as_ref(), Rc::clone(&env))?;
    if is_error(condition.as_ref()) {
        return Some(condition);
    }

    if is_truthy(condition.as_ref()) {
        eval_block_statement(&expr.consequence, env)
    } else if let Some(ref alternative) = expr.alternative {
        eval_block_statement(alternative, env)
    } else {
        Some(Box::new(NULL))
    }
}

fn is_truthy(obj: &dyn Object) -> bool {
    match obj.object_type() {
        ObjectType::NULL => false,
        ObjectType::BOOLEAN => {
            let b = obj.as_any().downcast_ref::<object::Boolean>().unwrap();
            b.value
        }
        _ => true,
    }
}

fn unwrap_return_value(obj: Box<dyn Object>) -> Box<dyn Object> {
    if let Some(ret) = obj.as_any().downcast_ref::<object::ReturnValue>() {
        return ret.value.clone_box();
    }
    obj
}

fn is_error(obj: &dyn Object) -> bool {
    obj.object_type() == ObjectType::ERROR
}

fn new_error(message: String) -> Box<dyn Object> {
    Box::new(object::Error { message })
}

fn native_bool_to_bool_object(input: bool) -> object::Boolean {
    if input { BOOL_TRUE } else { BOOL_FALSE }
}

#[cfg(test)]
mod tests;
