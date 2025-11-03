
pub mod lexer;
pub mod parser;
pub mod utils;
pub mod helper;
pub mod prelude;
pub mod semantics;

fn main() {
    let mut parser = parser::parser::Parser::new("test.txt");
    let symbols = parser.parse();
    semantics::semantics::Semantics::new(&symbols).validate_semantics();
}

