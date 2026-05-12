mod lexer;
mod token;

use crate::lexer::LexerTraits;
use lexer::new;

fn main() {
    let input = "let five = 5;
    let ten = 10;
    let add = fn(x, y) {
        x + y;
    };
    let result = add(five, ten);";

    let mut l = new(input.to_string());

    for (i, tok) in (0..36).map(|_| l.next_token()).enumerate() {
        println!("{}: {:?}", tok.type_, tok.literal);
    }
}
