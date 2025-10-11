use crate::lexer::lexer::{Lexer, SymbolKind, Token};
use super::rules::RULES;
use std::collections::VecDeque;
use crate::{debug, error, info};
use crate::helper::visualize_ast::visualize_ast;
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
}
impl Operator {
    pub fn from_char(char: char) -> Option<Operator> {
        match char {
            '+' => Some(Operator::Add),
            '-' => Some(Operator::Sub),
            '*' => Some(Operator::Mul),
            '/' => Some(Operator::Div),
            '=' => Some(Operator::Equal),
            _ => None,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Integer,
    String,
    Bool,
}
impl Type {
    pub fn from_identifier(identifier: &ASTNode) -> Option<Type> {
        if let ASTNode::Identifier(ident) = identifier {
            match ident.as_str() {
                "int" => Some(Type::Integer),
                "string" => Some(Type::String),
                "bool" => Some(Type::Bool),
                _ => None,
            }
        } else {
            None
        }
    }

}
#[derive(Debug, PartialEq, Eq, Hash,)]
pub enum Punctuation {
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    Semicolon,
}

impl Punctuation {
    pub fn from_char(ch: char) -> Option<Punctuation> {
        match ch {
            '{' => Some(Punctuation::OpenBrace),
            '}' => Some(Punctuation::CloseBrace),
            '(' => Some(Punctuation::OpenParen),
            ')' => Some(Punctuation::CloseParen),
            '[' => Some(Punctuation::OpenBracket),
            ']' => Some(Punctuation::CloseBracket),
            ';' => Some(Punctuation::Semicolon),
            _ => None,
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
    Operator(Operator),
    Punctuation(Punctuation),
    Empty(), // Empty node for none value kinds (=, +, -, etc)
}
impl ASTNode {
    pub fn from_token_value(token: &Token) -> ASTNode {
        match token.get_kind() {
            SymbolKind::Number => ASTNode::Number(token.get_value().parse().unwrap()),
            SymbolKind::Identifier => ASTNode::Identifier(token.get_value()),
            SymbolKind::Operator => ASTNode::Operator(Operator::from_char(token.get_value().chars().next().unwrap()).unwrap()),
            SymbolKind::Punctuation => ASTNode::Punctuation(Punctuation::from_char(token.get_value().chars().next().unwrap()).unwrap()),
            _ => ASTNode::Empty(),
        }
    }
}
#[derive(Debug)]
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
pub struct Parser {
    symbols: Vec<SymbolNode>,
    laxer: Lexer,
    current_line: u32,
}

impl Parser {
    pub fn new(file_name: &str) -> Parser {
        Parser {
            symbols: Vec::new(),
            laxer: Lexer::new(file_name),
            current_line: 1,
        }
    }

    pub fn parse(&mut self) {
        while let Some(token) = self.laxer.get_next_token() {
            self.current_line = token.get_line_number();
            if token.get_kind() == SymbolKind::Whitespace {
                continue;
            }
            self.symbols.push(SymbolNode {
                kind: token.get_kind(),
                value: ASTNode::from_token_value(&token),
            });
            self.try_reduce();
        }
    }

    fn try_reduce(&mut self) {
        let mut last_semicolon = false;
        if self.symbols[self.symbols.len() - 1].kind == SymbolKind::Punctuation {
            let ast_node = &self.symbols[self.symbols.len() - 1].value;
            if let ASTNode::Punctuation(_) = ast_node {
                last_semicolon = true;
            }
        }

        let offset = if last_semicolon { 2 } else { 1 };
        if self.symbols.len().checked_sub(offset).is_none() {
            error!("Error at line {}, expected more symbols", self.current_line);
            std::process::exit(1);
        }
        let rule_list = RULES.get(&self.symbols[self.symbols.len() - offset].kind);
        if rule_list.is_none() {
            return;
        }

        let rule_list = rule_list.unwrap();
        for rule in rule_list {
            if self.check_symbols_in_rule(&rule) {
                self.try_reduce();
            }
        }
    }

    fn check_symbols_in_rule(&mut self, rule: &(SymbolKind, Vec<SymbolKind>)) -> bool {
        for symbol in 1..rule.1.len(){
            if self.symbols.len().checked_sub(1 + symbol).is_none() {
                return false;
            }
            if self.symbols[self.symbols.len() - 1 - symbol].kind
                != rule.1[rule.1.len() - 1 - symbol]
            {
                return false;
            }
        }
    self.create_ast_node(rule);
        info!("{:?}", self.symbols);
        debug!("{:?}", rule);
        visualize_ast(&self.symbols);
        true
    }

    fn create_ast_node(&mut self, rule: &(SymbolKind, Vec<SymbolKind>)) {
        let mut symbols: VecDeque<SymbolNode> = VecDeque::new();
        for _ in 0..rule.1.len() {
            symbols.push_front(self.symbols.pop().unwrap());
        }

        match rule.0 {
            SymbolKind::Assign => {
                let typ = Type::from_identifier(&symbols.pop_front().unwrap().get_value());
                if typ.is_none() {
                    error!("Error at line {}, Unknown type", self.current_line);
                    std::process::exit(1);
                }
                let typ = typ.unwrap();
                let name = Box::new(symbols.pop_front().unwrap().get_value());
                // Pop the '='
                let _ = symbols.pop_front();
                let expr = Box::new(symbols.pop_front().unwrap().value);
                let assign_node = ASTNode::Assign {
                    typ,
                    name,
                    expr,
                };
                info!("{:?}", self.symbols);

                info!("{:?}", self.symbols);
                self.symbols.push(SymbolNode::new(SymbolKind::Assign, assign_node));
            }
            SymbolKind::Expr => {
                let expr_node = symbols.pop_front().unwrap().get_value();
                self.symbols.push(SymbolNode::new(SymbolKind::Expr, expr_node));
            }
            SymbolKind::BinaryOperation => {
                let binary_operation_node = ASTNode::BinaryOperation {
                    left: Box::new(symbols.pop_front().unwrap().get_value()),
                    operation: Box::new(symbols.pop_front().unwrap().get_value()),
                    right: Box::new(symbols.pop_front().unwrap().get_value()),
                };
                self.symbols.push(SymbolNode::new(SymbolKind::BinaryOperation, binary_operation_node));
            }

            _ => {}
        }

    }

}
