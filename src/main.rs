
pub mod lexer;
pub mod parser;
pub mod parser_rules;
pub mod utils;

fn main() {
    let mut parser = parser::Parser::new("test.txt");
    parser.parse();
}

