use crate::lexer::lexer::SymbolKind;
use once_cell::sync::Lazy;
use std::collections::HashMap;

/*
structure:
Map by last symbol kind of the rule -> gets a list of all the rules that start with this symbol kind
-> every item in the list is a tuple of the LHS and RHS of the rule
 */
pub static RULES: Lazy<HashMap<SymbolKind, Vec<(SymbolKind, Vec<SymbolKind>)>>> = Lazy::new(|| {
    let mut m = HashMap::new();

    m.insert(
        SymbolKind::Expr,
        vec![(
            SymbolKind::BinaryOperation,
            vec![SymbolKind::Expr, SymbolKind::Operator, SymbolKind::Expr],
        )],
    );

    m.insert(
        SymbolKind::Number,
        vec![(SymbolKind::Expr, vec![SymbolKind::Number])],
    );
    m.insert(
        SymbolKind::Bool,
        vec![(SymbolKind::Expr, vec![SymbolKind::Bool])],
    );
    m.insert(
        SymbolKind::String,
        vec![(SymbolKind::Expr, vec![SymbolKind::String])],
    );
    m.insert(
        SymbolKind::Char,
        vec![(SymbolKind::Expr, vec![SymbolKind::Char])],
    );
    m.insert(
        SymbolKind::BinaryOperation,
        vec![(SymbolKind::Expr, vec![SymbolKind::BinaryOperation])],
    );
    m.insert(
        SymbolKind::Punctuation,
        vec![

            (
                SymbolKind::DeclarationAssignment,
                vec![
                    SymbolKind::Identifier,
                    SymbolKind::Identifier,
                    SymbolKind::Operator,
                    SymbolKind::Expr,
                    SymbolKind::Punctuation,
                ],
            ),
            (
                SymbolKind::Declaration,
                vec![
                    SymbolKind::Identifier,
                    SymbolKind::Identifier,
                    SymbolKind::Punctuation,
                ],
            ),
            (
                SymbolKind::Assign,
                vec![
                    SymbolKind::Identifier,
                    SymbolKind::Operator,
                    SymbolKind::Expr,
                    SymbolKind::Punctuation,
                ],
            ),
        ],
    );

    m
});
