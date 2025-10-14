
pub mod lexer;
pub mod parser;
pub mod utils;
pub mod helper;
pub mod prelude;
pub mod semantics;

fn main() {
    // let mut parser = parser::parser::Parser::new("test.txt");
    // parser.parse();

    semantics::semantics::Semantics::new().validate_semantics();
}

