use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::{self, NodeTrait};

#[derive(Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum ObjectType {
    INTEGER,
    BOOLEAN,
    NULL,
    RETURN_VALUE,
    ERROR,
    FUNCTION,
}

pub trait Object {
    fn object_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
    fn clone_box(&self) -> Box<dyn Object>;
    fn as_any(&self) -> &dyn std::any::Any;
}

pub struct Integer {
    pub value: i64,
}

impl Object for Integer {
    fn object_type(&self) -> ObjectType {
        ObjectType::INTEGER
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }

    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(Integer { value: self.value })
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct Boolean {
    pub value: bool,
}

impl Object for Boolean {
    fn object_type(&self) -> ObjectType {
        ObjectType::BOOLEAN
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }

    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(Boolean { value: self.value })
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct Null;

impl Object for Null {
    fn object_type(&self) -> ObjectType {
        ObjectType::NULL
    }

    fn inspect(&self) -> String {
        "null".to_string()
    }

    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(Null)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct ReturnValue {
    pub value: Box<dyn Object>,
}

impl Object for ReturnValue {
    fn object_type(&self) -> ObjectType {
        ObjectType::RETURN_VALUE
    }

    fn inspect(&self) -> String {
        self.value.inspect()
    }

    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(ReturnValue {
            value: self.value.clone_box(),
        })
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct Error {
    pub message: String,
}

impl Object for Error {
    fn object_type(&self) -> ObjectType {
        ObjectType::ERROR
    }

    fn inspect(&self) -> String {
        format!("ERROR: {}", self.message)
    }

    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(Error {
            message: self.message.clone(),
        })
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct Function {
    pub parameters: Vec<ast::Identifier>,
    pub body: Rc<ast::BlockStatement>,
    pub env: Rc<RefCell<Environment>>,
}

impl Object for Function {
    fn object_type(&self) -> ObjectType {
        ObjectType::FUNCTION
    }

    fn inspect(&self) -> String {
        let params: Vec<String> = self.parameters.iter().map(|p| p.string()).collect();
        format!(
            "fn({}) {{\n{}\n}}",
            params.join(", "),
            self.body.string()
        )
    }

    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(Function {
            parameters: self.parameters.clone(),
            body: Rc::clone(&self.body),
            env: Rc::clone(&self.env),
        })
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Default)]
pub struct Environment {
    store: HashMap<String, Box<dyn Object>>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn from_outer(outer: Rc<RefCell<Environment>>) -> Self {
        Self {
            store: HashMap::new(),
            outer: Some(outer),
        }
    }

    pub fn get(&self, name: &str) -> Option<Box<dyn Object>> {
        if let Some(value) = self.store.get(name) {
            return Some(value.clone_box());
        }
        if let Some(ref outer) = self.outer {
            return outer.borrow().get(name);
        }
        None
    }

    pub fn set(&mut self, name: &str, value: Box<dyn Object>) {
        self.store.insert(name.to_string(), value);
    }
}
