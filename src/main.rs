
pub mod lexer;
pub mod parser;
pub mod utils;
pub mod helper;
pub mod prelude;

fn main() {
    let mut parser = parser::parser::Parser::new("test.txt");
    parser.parse();
}

