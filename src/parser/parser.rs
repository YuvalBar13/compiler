use crate::lexer::lexer::{Lexer, SymbolKind};
use super::rules::RULES;
use std::collections::VecDeque;
use crate::{error, info};
use crate::helper::visualize_ast::visualize_ast;
use super::ast::{ASTNode, SymbolNode};
pub struct Parser {
    symbols: Vec<SymbolNode>,
    lexer: Lexer,
    current_line: u32,
}

impl Parser {
    pub fn new(file_name: &str) -> Parser {
        Parser {
            symbols: Vec::new(),
            lexer: Lexer::new(file_name),
            current_line: 1,
        }
    }

    pub fn parse(&mut self) {
        while let Some(token) = self.lexer.get_next_token() {
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
        info!("asdf");
        visualize_ast(&self.symbols);
        true
    }

    fn create_ast_node(&mut self, rule: &(SymbolKind, Vec<SymbolKind>)) {
        let mut symbols: VecDeque<SymbolNode> = VecDeque::new();
        for _ in 0..rule.1.len() {
            symbols.push_front(self.symbols.pop().unwrap());
        }

        let node = match rule.0 {
            SymbolKind::Assign => {
                match ASTNode::create_assign(&mut symbols, self.current_line) {
                    Ok(node) => node,
                    Err(msg) => {
                        error!("{}", msg);
                        std::process::exit(1);
                    }
                }
            }
            SymbolKind::Expr => ASTNode::create_expr(&mut symbols),
            SymbolKind::BinaryOperation => ASTNode::create_binary_op(&mut symbols),
            _ => return,
        };

        self.symbols.push(SymbolNode::new(rule.0, node));
    }

}
