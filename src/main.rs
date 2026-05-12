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
    let result = add(five, ten);
    !-/*5; 
    5 < 10 > 5;

    if (5 < 10) {
        return true;
    } else {
        return false;
    }

    10 == 10;
    10 != 9;";

    let mut l = new(input.to_string());

    for (_i, tok) in (0..80).map(|_| l.next_token()).enumerate() {
        println!("{}: {:?}", tok.type_, tok.literal);
    }
}
