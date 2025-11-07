use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

use std::collections::HashMap;
use crate::parser::ast::{ASTNode, SymbolNode};
use crate::parser::types::Type;

#[derive(Debug)]
struct LocalVar {
    offset: i32, // offset from rbp (negative)
    size: i32,   // size in bytes
}

#[derive(Debug)]
struct FunctionContext {
    locals: HashMap<String, LocalVar>,
    current_stack_size: i32,
}

impl FunctionContext {
    fn new() -> Self {
        Self {
            locals: HashMap::new(),
            current_stack_size: 0,
        }
    }

    fn add_local(&mut self, name: String, size: i32) {
        self.current_stack_size += size;
        let offset = -self.current_stack_size;
        self.locals.insert(name, LocalVar { offset, size });
    }

    fn get_local(&self, name: &str) -> Option<&LocalVar> {
        self.locals.get(name)
    }
}


pub struct Generator<'a> {
    symbols: &'a [SymbolNode],
    data_file: File,
    code_file: File,
    function_context: FunctionContext,
}

impl<'a> Generator<'a> {
    pub fn new(symbols: &'a [SymbolNode], file_name: &str) -> Generator<'a> {
        let data_file = OpenOptions::new()
            .write(true)      // allow writing
            .create(true)     // create if it doesn't exist
            .truncate(true)   // overwrite existing content
            .open(format!("{}.data", file_name)).expect("Could not open output file");

        let code_file = OpenOptions::new()
            .write(true)      // allow writing
            .create(true)     // create if it doesn't exist
            .truncate(true)   // overwrite existing content
            .open(format!("{}.code", file_name)).expect("Could not open output file");

        Generator { symbols, data_file, code_file, function_context: FunctionContext::new() }
    }

    pub fn write_code_line(&mut self, line: &str) {
        writeln!(self.code_file, "{}", line).expect("Failed to write line to code file");
    }

    pub fn write_data_line(&mut self, line: &str) {
        writeln!(self.data_file, "{}", line).expect("Failed to write line to data file");
    }

    pub fn generate(&mut self) {
            for symbol in self.symbols {
                match &symbol.value {
                    ASTNode::DeclarationAssignment { typ, name, value } => {}
                    ASTNode::Declaration { typ, name } => {self.handle_declaration(typ, name);}
                    ASTNode::Assign { .. } => {}
                    _ => {}


                }
            }
    }

    pub fn handle_assignment(&self, symbol: &SymbolNode) {

    }

    pub fn handle_declaration(&mut self, typ: &Type, name: &ASTNode) {
        if let Some(variable_name) = name.as_identifier() {
            println!("Variable name: {}", variable_name);
            self.function_context.add_local(variable_name.to_string(), typ.to_size_asm());
            if let Some(local) = self.function_context.get_local(variable_name) {
                let offset = local.offset;
                // write rax into local variable at [rbp + offset]
                self.write_code_line("mov rax, 0");
                self.write_code_line(&format!("mov [rbp{}], rax", offset));
            }

        }

    }


    pub fn handle_declaration_assignment(&self,  typ: &Type, name: &ASTNode) {

    }

    pub fn handle_assign(&self, symbol: &SymbolNode) {

    }

}