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
    STRING,
    ARRAY,
    HASH,
    BUILTIN,
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

pub struct StringObj {
    pub value: String,
}

impl Object for StringObj {
    fn object_type(&self) -> ObjectType {
        ObjectType::STRING
    }

    fn inspect(&self) -> String {
        self.value.clone()
    }

    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(StringObj {
            value: self.value.clone(),
        })
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct Array {
    pub elements: Vec<Box<dyn Object>>,
}

impl Object for Array {
    fn object_type(&self) -> ObjectType {
        ObjectType::ARRAY
    }

    fn inspect(&self) -> String {
        let elements: Vec<String> = self.elements.iter().map(|e| e.inspect()).collect();
        format!("[{}]", elements.join(", "))
    }

    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(Array {
            elements: self.elements.iter().map(|e| e.clone_box()).collect(),
        })
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum HashKey {
    Integer(i64),
    Boolean(bool),
    String(String),
}

pub trait Hashable {
    fn hash_key(&self) -> HashKey;
}

impl Hashable for Integer {
    fn hash_key(&self) -> HashKey {
        HashKey::Integer(self.value)
    }
}

impl Hashable for Boolean {
    fn hash_key(&self) -> HashKey {
        HashKey::Boolean(self.value)
    }
}

impl Hashable for StringObj {
    fn hash_key(&self) -> HashKey {
        HashKey::String(self.value.clone())
    }
}

pub struct HashPair {
    pub key: Box<dyn Object>,
    pub value: Box<dyn Object>,
}

pub struct Hash {
    pub pairs: HashMap<HashKey, HashPair>,
}

impl Object for Hash {
    fn object_type(&self) -> ObjectType {
        ObjectType::HASH
    }

    fn inspect(&self) -> String {
        let mut pairs: Vec<String> = self
            .pairs
            .values()
            .map(|pair| format!("{}: {}", pair.key.inspect(), pair.value.inspect()))
            .collect();
        pairs.sort();
        format!("{{{}}}", pairs.join(", "))
    }

    fn clone_box(&self) -> Box<dyn Object> {
        let mut new_pairs = HashMap::new();
        for (k, v) in &self.pairs {
            new_pairs.insert(
                k.clone(),
                HashPair {
                    key: v.key.clone_box(),
                    value: v.value.clone_box(),
                },
            );
        }
        Box::new(Hash { pairs: new_pairs })
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub type BuiltinFunction = fn(Vec<Box<dyn Object>>) -> Box<dyn Object>;

pub struct Builtin {
    pub func: BuiltinFunction,
}

impl Object for Builtin {
    fn object_type(&self) -> ObjectType {
        ObjectType::BUILTIN
    }

    fn inspect(&self) -> String {
        "builtin function".to_string()
    }

    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(Builtin { func: self.func })
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
        let mut env = Self {
            store: HashMap::new(),
            outer: None,
        };
        env.register_builtins();
        env
    }

    fn register_builtins(&mut self) {
        self.set("len", Box::new(Builtin { func: builtin_len }));
        self.set("first", Box::new(Builtin { func: builtin_first }));
        self.set("last", Box::new(Builtin { func: builtin_last }));
        self.set("rest", Box::new(Builtin { func: builtin_rest }));
        self.set("push", Box::new(Builtin { func: builtin_push }));
        self.set("puts", Box::new(Builtin { func: builtin_puts }));
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

fn builtin_len(args: Vec<Box<dyn Object>>) -> Box<dyn Object> {
    if args.len() != 1 {
        return Box::new(Error {
            message: format!("wrong number of arguments. got={}, want=1", args.len()),
        });
    }
    if let Some(s) = args[0].as_any().downcast_ref::<StringObj>() {
        return Box::new(Integer {
            value: s.value.len() as i64,
        });
    }
    if let Some(arr) = args[0].as_any().downcast_ref::<Array>() {
        return Box::new(Integer {
            value: arr.elements.len() as i64,
        });
    }
    Box::new(Error {
        message: format!("argument to `len` not supported, got {:?}", args[0].object_type()),
    })
}

fn builtin_first(args: Vec<Box<dyn Object>>) -> Box<dyn Object> {
    if args.len() != 1 {
        return Box::new(Error {
            message: format!("wrong number of arguments. got={}, want=1", args.len()),
        });
    }
    if let Some(arr) = args[0].as_any().downcast_ref::<Array>() {
        if !arr.elements.is_empty() {
            return arr.elements[0].clone_box();
        }
        return Box::new(Null);
    }
    Box::new(Error {
        message: format!(
            "argument to `first` must be ARRAY, got {:?}",
            args[0].object_type()
        ),
    })
}

fn builtin_last(args: Vec<Box<dyn Object>>) -> Box<dyn Object> {
    if args.len() != 1 {
        return Box::new(Error {
            message: format!("wrong number of arguments. got={}, want=1", args.len()),
        });
    }
    if let Some(arr) = args[0].as_any().downcast_ref::<Array>() {
        if let Some(last) = arr.elements.last() {
            return last.clone_box();
        }
        return Box::new(Null);
    }
    Box::new(Error {
        message: format!(
            "argument to `last` must be ARRAY, got {:?}",
            args[0].object_type()
        ),
    })
}

fn builtin_rest(args: Vec<Box<dyn Object>>) -> Box<dyn Object> {
    if args.len() != 1 {
        return Box::new(Error {
            message: format!("wrong number of arguments. got={}, want=1", args.len()),
        });
    }
    if let Some(arr) = args[0].as_any().downcast_ref::<Array>() {
        if arr.elements.is_empty() {
            return Box::new(Null);
        }
        let mut new_elements = Vec::new();
        for (i, el) in arr.elements.iter().enumerate() {
            if i != 0 {
                new_elements.push(el.clone_box());
            }
        }
        return Box::new(Array {
            elements: new_elements,
        });
    }
    Box::new(Error {
        message: format!(
            "argument to `rest` must be ARRAY, got {:?}",
            args[0].object_type()
        ),
    })
}

fn builtin_push(args: Vec<Box<dyn Object>>) -> Box<dyn Object> {
    if args.len() != 2 {
        return Box::new(Error {
            message: format!("wrong number of arguments. got={}, want=2", args.len()),
        });
    }
    if let Some(arr) = args[0].as_any().downcast_ref::<Array>() {
        let mut new_elements: Vec<Box<dyn Object>> =
            arr.elements.iter().map(|e| e.clone_box()).collect();
        new_elements.push(args[1].clone_box());
        return Box::new(Array {
            elements: new_elements,
        });
    }
    Box::new(Error {
        message: format!(
            "argument to `push` must be ARRAY, got {:?}",
            args[0].object_type()
        ),
    })
}

fn builtin_puts(args: Vec<Box<dyn Object>>) -> Box<dyn Object> {
    for arg in args {
        println!("{}", arg.inspect());
    }
    Box::new(Null)
}
