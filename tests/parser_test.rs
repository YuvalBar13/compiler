use std::fs;
use compiler::lexer::lexer::SymbolKind;
use compiler::parser::ast::{ASTNode, SymbolNode};
use compiler::parser::types::{OperatorType, Type};
use compiler::parser::parser::Parser;


const FILE_NAME: &str = "test_parser.txt";

#[test]
fn test_basic_parser() {
    let source = "int x = 2 * 2 + 2;\nint y = 1;";
    fs::write(FILE_NAME, source).unwrap();

    let mut parser = Parser::new(FILE_NAME);
    parser.parse();

    let symbols = parser.get_symbols();
    let expected = vec![
        SymbolNode::new(
            SymbolKind::Assign,
            ASTNode::Assign {
                typ: Type::Integer,
                name: Box::new(ASTNode::Identifier("x".into())),
                value: Box::new(ASTNode::BinaryOperation {
                    left: Box::new(ASTNode::BinaryOperation {
                        left: Box::new(ASTNode::Number(2)),
                        right: Box::new(ASTNode::Number(2)),
                        operation: Box::new(ASTNode::Operator(OperatorType::Mul)),
                    }),
                    right: Box::new(ASTNode::Number(2)),
                    operation: Box::new(ASTNode::Operator(OperatorType::Add)),
                }),
            },
        ),
        SymbolNode::new(
            SymbolKind::Assign,
            ASTNode::Assign {
                typ: Type::Integer,
                name: Box::new(ASTNode::Identifier("y".into())),
                value: Box::new(ASTNode::Number(1)),
            },
        ),
    ];

    assert_eq!(*symbols, expected, "Parsed output does not match expected AST");




}