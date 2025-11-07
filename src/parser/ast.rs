use std::collections::VecDeque;
use crate::error;
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
    Char(char),
    Identifier(String),

    Declaration {
        typ: Type,
        name: Box<ASTNode>,
    },
    DeclarationAssignment {
        typ: Type,
        name: Box<ASTNode>,
        value: Box<ASTNode>,
    },
    Assign {
        name: Box<ASTNode>,
        value: Box<ASTNode>,
    },
    BinaryOperation {
        left: Box<ASTNode>,
        right: Box<ASTNode>,
        operation: Box<ASTNode>,
    },
    Operator(OperatorType),
    Punctuation(Punctuation),
    Expr(Box<ASTNode>),
    Empty(), //
}
use std::fmt;

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTNode::Number(n) => write!(f, "{}", n),
            ASTNode::String(s) => write!(f, "\"{}\"", s),
            ASTNode::Bool(b) => write!(f, "{}", b),
            ASTNode::Char(c) => write!(f, "'{}'", c),
            ASTNode::Identifier(name) => write!(f, "{}", name),

            ASTNode::Declaration { typ, name } => {
                write!(f, "Declaration({:?}, {})", typ, name)
            }
            ASTNode::DeclarationAssignment { typ, name, value } => {
                write!(f, "DeclarationAssignment({:?}, {}, {})", typ, name, value)
            }
            ASTNode::Assign { name, value } => write!(f, "Assign({}, {})", name, value),
            ASTNode::BinaryOperation { left, right, operation } => {
                write!(f, "BinaryOperation({}, {}, {})", left, operation, right)
            }
            ASTNode::Operator(op) => write!(f, "{:?}", op),
            ASTNode::Punctuation(p) => write!(f, "{:?}", p),
            ASTNode::Expr(expr) => write!(f, "Expr({})", expr),
            ASTNode::Empty() => write!(f, "Empty"),
        }
    }
}


impl ASTNode {
    pub fn from_token_value(token: &Token) -> ASTNode {
        match token.get_kind() {
            SymbolKind::Number => ASTNode::Number(token.get_value().parse().unwrap()),
            SymbolKind::Identifier => ASTNode::Identifier(token.get_value()),
            SymbolKind::String => ASTNode::String(token.get_value()),
            SymbolKind::Bool => ASTNode::Bool(token.get_value() == "true".to_string()),
            SymbolKind::Char => ASTNode::Char(token.get_value().chars().next().unwrap()),
            SymbolKind::Operator => ASTNode::Operator(OperatorType::from_char(token.get_value().chars().next().unwrap()).unwrap()),
            SymbolKind::Punctuation => ASTNode::Punctuation(Punctuation::from_char(token.get_value().chars().next().unwrap()).unwrap()),
            _ => ASTNode::Empty(),
        }
    }
    pub fn create_assign(symbols: &mut VecDeque<SymbolNode>, ) -> ASTNode {
        let name = Box::new(symbols.pop_front().unwrap().get_value());
        let _ = symbols.pop_front(); // Pop '='
        let value = Box::new(symbols.pop_front().unwrap().value);

        ASTNode::Assign {name, value }
    }

    pub fn create_binary_op(symbols: &mut VecDeque<SymbolNode>) -> ASTNode {
        ASTNode::BinaryOperation {
            left: Box::new(symbols.pop_front().unwrap().get_value()),
            operation: Box::new(symbols.pop_front().unwrap().get_value()),
            right: Box::new(symbols.pop_front().unwrap().get_value()),
        }
    }

    pub fn create_expr(symbols: &mut VecDeque<SymbolNode>) -> ASTNode {
        ASTNode::Expr(Box::new(symbols.pop_front().unwrap().get_value()))
    }

    pub fn create_declaration(symbols: &mut VecDeque<SymbolNode>, line: u32) -> ASTNode {
        let typ = Self::get_type(symbols, line);
        let name = Box::new(symbols.pop_front().unwrap().get_value());

        ASTNode::Declaration {typ, name,}
    }

    pub fn create_declaration_assignment(symbols: &mut VecDeque<SymbolNode>, line: u32) -> ASTNode {
        let typ = Self::get_type(symbols, line);
        let name = Box::new(symbols.pop_front().unwrap().get_value());

        let _ = symbols.pop_front(); // Pop '='
        let value = Box::new(symbols.pop_front().unwrap().value);

        ASTNode::DeclarationAssignment {typ, name, value}
    }

    fn get_type(symbols: &mut VecDeque<SymbolNode>, line_number: u32) -> Type
    {
        let token = symbols.pop_front().unwrap().get_value();
        let typ = Type::from_identifier(&token);
        if typ.is_none() {
            if let ASTNode::Identifier(wrong_type) = token{
                error!("Error at line: {} Unknown type {:?}", line_number, wrong_type);
                std::process::exit(1);
            }
        }
        typ.unwrap()
    }

    pub fn inferred_type(&self) -> Option<Type> {
        match self {
            ASTNode::Number(_) => Some(Type::Integer),
            ASTNode::String(_) => Some(Type::String),
            ASTNode::Bool(_) => Some(Type::Bool),
            ASTNode::Char(_) => Some(Type::Char),
            _ => None,
        }
    }


}

impl ASTNode {
    pub fn as_number(&self) -> Option<i32> {
        if let ASTNode::Number(n) = self {
            Some(*n)
        } else {
            None
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        if let ASTNode::String(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let ASTNode::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    pub fn as_identifier(&self) -> Option<&str> {
        if let ASTNode::Identifier(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn as_declaration(&self) -> Option<(&Type, &ASTNode)> {
        if let ASTNode::Declaration { typ, name } = self {
            Some((typ, name.as_ref()))
        } else {
            None
        }
    }

    pub fn as_declaration_assignment(&self) -> Option<(&Type, &ASTNode, &ASTNode)> {
        if let ASTNode::DeclarationAssignment { typ, name, value } = self {
            Some((typ, name.as_ref(), value.as_ref()))
        } else {
            None
        }
    }

    pub fn as_assign(&self) -> Option<(&ASTNode, &ASTNode)> {
        if let ASTNode::Assign { name, value } = self {
            Some((name.as_ref(), value.as_ref()))
        } else {
            None
        }
    }

    pub fn as_binary_operation(&self) -> Option<(&ASTNode, &ASTNode, &ASTNode)> {
        if let ASTNode::BinaryOperation { left, right, operation } = self {
            Some((left.as_ref(), right.as_ref(), operation.as_ref()))
        } else {
            None
        }
    }

    pub fn as_operator(&self) -> Option<&OperatorType> {
        if let ASTNode::Operator(op) = self {
            Some(op)
        } else {
            None
        }
    }

    pub fn as_punctuation(&self) -> Option<&Punctuation> {
        if let ASTNode::Punctuation(p) = self {
            Some(p)
        } else {
            None
        }
    }

    pub fn as_expr(&self) -> Option<&ASTNode> {
        if let ASTNode::Expr(inner) = self {
            Some(inner.as_ref())
        } else {
            None
        }
    }

    pub fn as_empty(&self) -> Option<()> {
        if let ASTNode::Empty() = self {
            Some(())
        } else {
            None
        }
    }
}


#[derive(Debug, PartialEq, Eq)]
pub struct SymbolNode {
    pub kind: SymbolKind,
    pub value: ASTNode,
}
impl SymbolNode {

    pub fn new(kind: SymbolKind, value: ASTNode) -> SymbolNode {
        SymbolNode { kind, value, }
    }
    pub fn get_value(self) -> ASTNode {
        self.value
    }
}