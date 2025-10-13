use std::collections::VecDeque;
use crate::lexer::lexer::{SymbolKind, Token};
use crate::parser::types::{OperatorType, Punctuation, Type};


impl Type {
    pub fn from_identifier(identifier: &ASTNode) -> Option<Type> {
        if let ASTNode::Identifier(ident) = identifier {
            Type::from_str(ident)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ASTNode {
    Number(i32),
    String(String),
    Bool(bool),
    Identifier(String),
    Assign { typ: Type, name: Box<ASTNode>, expr: Box<ASTNode> },
    BinaryOperation {
        left: Box<ASTNode>,
        right: Box<ASTNode>,
        operation: Box<ASTNode>,
    },
    Operator(OperatorType),
    Punctuation(Punctuation),
    Empty(), // Empty node for none value kinds (=, +, -, etc)
}
impl ASTNode {
    pub fn from_token_value(token: &Token) -> ASTNode {
        match token.get_kind() {
            SymbolKind::Number => ASTNode::Number(token.get_value().parse().unwrap()),
            SymbolKind::Identifier => ASTNode::Identifier(token.get_value()),
            SymbolKind::Operator => ASTNode::Operator(OperatorType::from_char(token.get_value().chars().next().unwrap()).unwrap()),
            SymbolKind::Punctuation => ASTNode::Punctuation(Punctuation::from_char(token.get_value().chars().next().unwrap()).unwrap()),
            _ => ASTNode::Empty(),
        }
    }
    pub fn create_assign(symbols: &mut VecDeque<SymbolNode>, line: u32) -> Result<ASTNode, String> {
        let typ = Type::from_identifier(&symbols.pop_front().unwrap().get_value())
            .ok_or_else(|| format!("Unknown type at line {}", line))?;
        let name = Box::new(symbols.pop_front().unwrap().get_value());
        let _ = symbols.pop_front(); // Pop '='
        let expr = Box::new(symbols.pop_front().unwrap().value);

        Ok(ASTNode::Assign { typ, name, expr })
    }

    pub fn create_binary_op(symbols: &mut VecDeque<SymbolNode>) -> ASTNode {
        ASTNode::BinaryOperation {
            left: Box::new(symbols.pop_front().unwrap().get_value()),
            operation: Box::new(symbols.pop_front().unwrap().get_value()),
            right: Box::new(symbols.pop_front().unwrap().get_value()),
        }
    }

    pub fn create_expr(symbols: &mut VecDeque<SymbolNode>) -> ASTNode {
        symbols.pop_front().unwrap().get_value()
    }

}
#[derive(Debug, PartialEq, Eq)]
pub struct SymbolNode {
    pub kind: SymbolKind,
    pub value: ASTNode,
}
impl SymbolNode {

    pub fn new(kind: SymbolKind, value: ASTNode) -> SymbolNode {
        SymbolNode { kind, value }
    }
    pub fn get_value(self) -> ASTNode {
        self.value
    }
}