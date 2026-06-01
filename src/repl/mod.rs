use crate::lexer::new;
use crate::parser::Parser;
use crate::parser::ParserTrait;
use crate::parser::new_parser;

const PROMPT: &str = ">> ";

pub fn start_repl() {
    println!("Welcome to the REPL! Type 'exit' to quit.");
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
        for stmt in program.statements {
            println!("{:?}", stmt.token_literal());
        }
    }
}
