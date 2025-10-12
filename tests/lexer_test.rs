use compiler::lexer::lexer::{Lexer, SymbolKind};
use std::fs;

const FILE_NAME: &str = "test_input.txt";
#[test]
fn test_basic_tokens() {
    let source = "int x = 1;\ntest = 2 ( 2";
    fs::write(FILE_NAME, source).unwrap();
    let mut lexer = Lexer::new(FILE_NAME);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.get_next_token() {
        tokens.push(token);
    }

    // Expected sequence of token kinds
    let expected = vec![
        SymbolKind::Identifier, // int
        SymbolKind::Whitespace, //
        SymbolKind::Identifier, // x
        SymbolKind::Whitespace, //
        SymbolKind::Operator,   // =
        SymbolKind::Whitespace, //
        SymbolKind::Number,     // 1
        SymbolKind::Punctuation,  // ;
        SymbolKind::Whitespace, //
        SymbolKind::Identifier, // test
        SymbolKind::Whitespace, //
        SymbolKind::Operator,   // =
        SymbolKind::Whitespace, //
        SymbolKind::Number,     // 2
        SymbolKind::Whitespace, //
        SymbolKind::Punctuation,  // (
        SymbolKind::Whitespace, //
        SymbolKind::Number,     // 2
    ];

    let actual: Vec<_> = tokens.iter().map(|t| t.get_kind()).collect();
    assert_eq!(actual, expected, "Token kinds do not match!");
}
