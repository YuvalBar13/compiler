use crate::lexer::{Lexer, SymbolKind, Token};
use crate::parser_rules::RULES;

enum ASTNode {
    Number(i32),
    String(String),
    Bool(bool),
    Identifier(String),
    Assign { name: String, expr: Box<ASTNode> },
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
struct SymbolNode {
    pub kind: SymbolKind,
    value: ASTNode,
}

struct Parser {
    symbols: Vec<SymbolNode>,
    laxer: Lexer,
    first_kind_index: usize,
}

impl Parser {
    fn new(file_name: &str) -> Parser {
        Parser {
            symbols: Vec::new(),
            laxer: Lexer::new(file_name),
            first_kind_index: 0,
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
        }
    }

    fn try_reduce(&mut self) {
        let rule_list = RULES.get(&self.symbols[self.first_kind_index].kind);
        if rule_list.is_none() {
            return;
        }

        let rule_list = rule_list.unwrap();
        for rule in rule_list {
            for symbol in 0..rule.1.len() {
                if self.symbols.len().checked_sub(1 + symbol).is_none() {
                    break;
                }

                if self.symbols[self.symbols.len() - 1 - symbol].kind
                    != rule.1[rule.1.len() - 1 - symbol]
                {
                    break;
                }
            }
            // pop the last symbols and create the new node
        }
    }
}
