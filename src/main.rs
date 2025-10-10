
pub mod lexer;
pub mod parser;
mod parser_rules;

fn main() {
    let mut lexer = lexer::Lexer::new("test.txt");

    for _ in 0..18 {

        let next = lexer.get_next_token();
        if next.is_none() {
            break;
        }
        println!("{:?}", next);

    }
}

