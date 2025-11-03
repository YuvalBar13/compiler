use crate::{error, info};
use crate::parser::ast::{ASTNode, SymbolNode};
use crate::parser::types::Type;
use crate::parser::types;
use std::collections::HashMap;

struct Scope {
    variables: HashMap<String, Type>,
}

impl Scope {
    fn new() -> Scope {
        Scope { variables: HashMap::new() }
    }

    fn add_variable(&mut self, name: String, typ: Type) {
        self.variables.insert(name, typ);
    }

    fn get_variable(&self, name: &str) -> Option<&Type> {
        self.variables.get(name)
    }

    fn has_variable(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }
}

struct VariablesTable {
    scopes: Vec<Scope>,
}

impl VariablesTable {
    fn new() -> VariablesTable {
        VariablesTable {
            scopes: vec![Scope::new(), Scope::new()],
            // second scope currently stands for "main" function
        }
    }

    fn add_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    fn remove_scope(&mut self) {
        self.scopes.pop();
    }

    fn find_variable(&self, name: &str) -> Option<&Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(var) = scope.get_variable(name) {
                return Some(var);
            }
        }
        None
    }

    fn add_variable(&mut self, name: String, typ: types::Type) {
        self.scopes.last_mut().unwrap().add_variable(name, typ);
    }

    fn add_global_variable(&mut self, name: String, typ: types::Type) {
        self.scopes.first_mut().unwrap().add_variable(name, typ);
    }
}

pub struct Semantics<'a> {
    variables_table: VariablesTable,
    symbols: &'a [SymbolNode],
}
impl<'a> Semantics<'a> {
    pub fn new(symbols: &'a [SymbolNode]) -> Semantics<'a> {
        Semantics {
            variables_table: VariablesTable::new(),
            symbols,
        }
    }

    pub fn validate_semantics(&mut self) {
        for symbol in self.symbols {
            match &symbol.value {
                ASTNode::Declaration { typ, name } => {
                    self.declaration(typ, name);
                }
                ASTNode::Assign { name, value } => {
                    self.assignment(name, value);
                }
                ASTNode::DeclarationAssignment { typ, name, value } => {
                    self.declaration_assignment(typ, name, value);
                }
                _ => {
                    error!(
                        "semantics failed! {:?} is not a valid semantic",
                        symbol.value
                    );
                }
            }
        }
        info!("semantics passed!");
    }

    fn declaration(&mut self, typ: &Type, name_ast: &Box<ASTNode>) {
        if let ASTNode::Identifier(name) = name_ast.as_ref() {
            if self.variables_table.find_variable(name).is_some() {
                error!("semantics failed! variable {} is already declared", name);
                std::process::exit(1);
            }
            self.variables_table.add_variable(name.clone(), typ.clone());
        } else {
            error!("invalid declaration {:?}", name_ast);
            std::process::exit(1);
        }
    }

    fn assignment(&mut self, name_ast: &Box<ASTNode>, value_ast: &Box<ASTNode>) {
        if let ASTNode::Identifier(name) = name_ast.as_ref() {
            if self.variables_table.find_variable(name).is_none() {
                error!("variable {} is not declared", name);
                std::process::exit(1);
            }
            if let ASTNode::Expr(expr) = value_ast.as_ref() {
                if !Self::validate_expr_type(
                    expr,
                    self.variables_table.find_variable(name).unwrap(),
                ) {
                    error!("type mismatch");
                    std::process::exit(1);
                }
            }
        } else {
            error!("invalid assignment {:?}", name_ast);
            std::process::exit(1);
        }
    }

    fn declaration_assignment(&mut self, typ: &Type, name_ast: &Box<ASTNode>, value_ast: &Box<ASTNode>) {
        self.declaration(typ, name_ast);
        self.assignment(name_ast, value_ast);
    }

    fn validate_expr_type(head: &ASTNode, expected_type: &Type) -> bool {
        match head {
            ASTNode::BinaryOperation { left, right, operation: _ } => {
                Self::validate_expr_type(left, expected_type)
                    && Self::validate_expr_type(right, expected_type)
            }
            _ => {
                if let Some(actual) = head.inferred_type() {
                    if &actual == expected_type {
                        true
                    } else {
                        error!(
                            "mismatched types\n\tExpected: {:?}\n\tGot: {:?}",
                            expected_type, actual
                        );
                        false
                    }
                } else {
                    false
                }
            }
        }
    }
}
