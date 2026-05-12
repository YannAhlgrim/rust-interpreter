use crate::lexer::LexerTraits;
use crate::lexer::new;

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
        let mut lexer = new(input.to_string());
        loop {
            let token = lexer.next_token();
            if token.type_ == "EOF" {
                break;
            }
            println!("Token: {:?}, Literal: {:?}", token.type_, token.literal);
        }
    }
}
