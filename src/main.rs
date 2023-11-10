mod ast;
mod lexer;
mod parser;

use std::io::{self, Write};

fn input(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");
    input
}

fn main() {
    loop {
        let expr = input("Enter a math expression: ");
        let mut my_lexer = lexer::Lexer::new(&expr);
        println!("{:#?}", my_lexer.get_tokens())
    }
}
