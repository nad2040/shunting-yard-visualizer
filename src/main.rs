pub mod lexer;
pub mod parser;
pub mod token;

use lexer::Lexer;
use parser::ShuntingYardParser as SYParser;
// use std::fs::read;
use token::{Loc, Token, TokenValue};

fn main() {
    let lexer: Lexer;

    // lexer = Lexer::new(String::from_utf8(read("example_program.fp").unwrap()).unwrap());
    // lexer.emit();
    // println!();
    // lexer = Lexer::new("123. 123.456 2193\n,391048".to_string());
    // lexer.emit();
    // println!();
    // lexer = Lexer::new("if (x > 3) { print(\"greater\"); } else { print(\"less\") }".to_string());
    // lexer.emit();
    lexer = Lexer::new("x = max(10, 1309, x * 2 + y)".to_string());
    lexer.emit();
    println!();

    let parser: SYParser;

    parser = SYParser::new(lexer.tokens);
    parser.emit();
}
