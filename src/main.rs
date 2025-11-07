
pub mod lexer;
pub mod parser;
pub mod utils;
pub mod helper;
pub mod prelude;
pub mod semantics;

pub mod code_gen;
fn main() {
    let mut parser = parser::parser::Parser::new("test.txt");
    let symbols = parser.parse();
    semantics::semantics::Semantics::new(&symbols).validate_semantics();
    let mut generator = code_gen::generator::Generator::new(&symbols, "test");
    generator.generate();
    
}

