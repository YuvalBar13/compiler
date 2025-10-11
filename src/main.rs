
pub mod lexer;
pub mod parser;
pub mod utils;

fn main() {
    let mut parser = parser::parser::Parser::new("test.txt");
    parser.parse();
}

