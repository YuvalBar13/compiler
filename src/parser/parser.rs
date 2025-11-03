use crate::lexer::lexer::{Lexer, SymbolKind};
use super::rules::RULES;
use std::collections::VecDeque;
use crate::helper::visualize_ast::visualize_ast;
use super::ast::{ASTNode, SymbolNode};

pub struct Parser {
    lexer: Lexer,
    current_line: u32,
}

impl Parser {
    pub fn new(file_name: &str) -> Parser {
        Parser {
            lexer: Lexer::new(file_name),
            current_line: 1,
        }
    }

    /// Parses the input and returns the list of top-level AST symbols
    pub fn parse(&mut self) -> Vec<SymbolNode> {
        let mut symbols: Vec<SymbolNode> = Vec::new();

        while let Some(token) = self.lexer.get_next_token() {
            self.current_line = token.get_line_number();

            if token.get_kind() == SymbolKind::Whitespace {
                continue;
            }

            symbols.push(SymbolNode {
                kind: token.get_kind(),
                value: ASTNode::from_token_value(&token),
            });

            self.try_reduce(&mut symbols);
        }

        // Return the completed AST/symbol list after parsing
        symbols
    }

    fn try_reduce(&mut self, symbols: &mut Vec<SymbolNode>) {
        let Some(rule_list) = RULES.get(&symbols[symbols.len() - 1].kind) else {
            return;
        };

        for rule in rule_list {
            if self.check_symbols_in_rule(symbols, rule) {
                self.try_reduce(symbols);
            }
        }
    }

    fn check_symbols_in_rule(
        &mut self,
        symbols: &mut Vec<SymbolNode>,
        rule: &(SymbolKind, Vec<SymbolKind>),
    ) -> bool {
        for i in 1..rule.1.len() {
            if symbols.len().checked_sub(1 + i).is_none() {
                return false;
            }

            if symbols[symbols.len() - 1 - i].kind != rule.1[rule.1.len() - 1 - i] {
                return false;
            }
        }

        self.create_ast_node(symbols, rule);
        visualize_ast(symbols);
        true
    }

    fn create_ast_node(&mut self, symbols: &mut Vec<SymbolNode>, rule: &(SymbolKind, Vec<SymbolKind>)) {
        let mut matched: VecDeque<SymbolNode> = VecDeque::new();
        for _ in 0..rule.1.len() {
            matched.push_front(symbols.pop().unwrap());
        }

        let node = match rule.0 {
            SymbolKind::Assign => ASTNode::create_assign(&mut matched),
            SymbolKind::Expr => ASTNode::create_expr(&mut matched),
            SymbolKind::BinaryOperation => ASTNode::create_binary_op(&mut matched),
            SymbolKind::Declaration => ASTNode::create_declaration(&mut matched, self.current_line),
            SymbolKind::DeclarationAssignment => {
                ASTNode::create_declaration_assignment(&mut matched, self.current_line)
            }
            _ => return,
        };

        symbols.push(SymbolNode::new(rule.0, node));
    }
}