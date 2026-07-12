use std::cell::RefCell;
use std::rc::Rc;

use crate::evaluator;
use crate::lexer::new;
use crate::object::Environment;
use crate::parser::Parser;
use crate::parser::new_parser;

const PROMPT: &str = ">> ";

pub fn start_repl() {
    println!("Welcome to the REPL! Type 'exit' to quit.");
    let env = Rc::new(RefCell::new(Environment::new()));
    loop {
        print!("{}", PROMPT);
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        if input == "exit" {
            break;
        }
        let lexer = new(input.to_string());
        let mut parser: Parser = new_parser(lexer);
        let program = parser.parse_program();
        let evaluated = evaluator::eval_program(program, Rc::clone(&env));

        for err in parser.errors() {
            eprintln!("ERROR: {}", err);
        }
        if !evaluated.is_none() {
            println!("{}", evaluated.unwrap().inspect());
        }
    }
}
