struct Lexer {
    input: String,
    position: Option<i32>,
    read_pos: Option<i32>,
    ch: Option<i8>,
}

fn new(input_str: String) -> Lexer {
    let l: Lexer = Lexer {
        input: input_str,
        position: None,
        read_pos: None,
        ch: None,
    };
    l
}

fn read_char(l: Lexer) {
    let read_pos = usize::try_from(l.read_pos.unwrap());
    if read_pos >= l.input.len() {
        l.ch = 0;
    }
}
