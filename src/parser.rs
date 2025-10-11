use crate::lexer::{Lexer, SymbolKind, Token};
use crate::parser_rules::RULES;
use std::collections::VecDeque;
use crate::info;

#[derive(Debug)]
enum ASTNode {
    Number(i32),
    String(String),
    Bool(bool),
    Identifier(String),
    Assign { name: Box<ASTNode>, expr: Box<ASTNode> },
    Empty(), // Empty node for none value kinds (=, +, -, etc)
}
impl ASTNode {
    pub fn from_token_value(token: &Token) -> ASTNode {
        match token.get_kind() {
            SymbolKind::Number => ASTNode::Number(token.get_value().parse().unwrap()),
            SymbolKind::Identifier => ASTNode::Identifier(token.get_value()),
            _ => ASTNode::Empty(),
        }
    }
}
#[derive(Debug)]
struct SymbolNode {
    pub kind: SymbolKind,
    value: ASTNode,
}
impl SymbolNode {
    pub fn get_value(self) -> ASTNode {
        self.value
    }
}
pub struct Parser {
    symbols: Vec<SymbolNode>,
    laxer: Lexer,
}

impl Parser {
    pub fn new(file_name: &str) -> Parser {
        Parser {
            symbols: Vec::new(),
            laxer: Lexer::new(file_name),
        }
    }

    pub fn parse(&mut self) {
        while let Some(token) = self.laxer.get_next_token() {
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
        let rule_list = RULES.get(&self.symbols[self.symbols.len() - 1].kind);
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
    
        true
    }

    fn create_ast_node(&mut self, rule: &(SymbolKind, Vec<SymbolKind>)) {
        let mut symbols: VecDeque<SymbolNode> = VecDeque::new();
        for _ in 0..rule.1.len() {
            let symbol = self.symbols.pop().unwrap();
            if symbol.kind == SymbolKind::Operator {
                continue;
            }
            symbols.push_front(symbol);
        }

        match rule.0 {
            SymbolKind::Assign => {
                let assign_node = ASTNode::Assign {
                    name: Box::new(symbols.pop_front().unwrap().get_value()),
                    expr: Box::new(symbols.pop_front().unwrap().value),
                };
                self.symbols.push(SymbolNode {
                    kind: SymbolKind::Assign,
                    value: assign_node,
                });
            }
            SymbolKind::Expr => {
                let expr_node = symbols.pop_front().unwrap().get_value();
                self.symbols.push(SymbolNode {
                    kind: SymbolKind::Expr,
                    value: expr_node,
                });
            }
            _ => {}
        }

    }
}
