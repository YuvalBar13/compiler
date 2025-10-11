use crate::parser::ast::ASTNode;
use crate::utils::{MAGENTA, RESET};
pub fn print_ast_tree(node: &ASTNode, prefix: String, is_last: bool) {
    let connector = if is_last { "└── " } else { "├── " };
    let extension = if is_last { "    " } else { "│   " };

    match node {
        ASTNode::Assign { typ, name, expr } => {
            println!("{}{}Assign", prefix, connector);
            let new_prefix = format!("{}{}", prefix, extension);

            println!("{}├── type: {:?}", new_prefix, typ);
            println!("{}├── name:", new_prefix);
            print_ast_tree(name, format!("{}│   ", new_prefix), false);
            println!("{}└── expr:", new_prefix);
            print_ast_tree(expr, format!("{}    ", new_prefix), true);
        }

        ASTNode::BinaryOperation { left, operation, right } => {
            // Print the operation symbol
            if let ASTNode::Operator(op) = operation.as_ref() {
                println!("{}{}{:?}", prefix, connector, op);
            } else {
                println!("{}{}BinaryOp", prefix, connector);
            }

            let new_prefix = format!("{}{}", prefix, extension);
            println!("{}├── left:", new_prefix);
            print_ast_tree(left, format!("{}│   ", new_prefix), false);
            println!("{}└── right:", new_prefix);
            print_ast_tree(right, format!("{}    ", new_prefix), true);
        }

        ASTNode::Number(n) => {
            println!("{}{}Number({})", prefix, connector, n);
        }

        ASTNode::String(s) => {
            println!("{}{}String(\"{}\")", prefix, connector, s);
        }

        ASTNode::Bool(b) => {
            println!("{}{}Bool({})", prefix, connector, b);
        }

        ASTNode::Identifier(id) => {
            println!("{}{}Identifier(\"{}\")", prefix, connector, id);
        }

        ASTNode::Operator(op) => {
            println!("{}{}Operator({:?})", prefix, connector, op);
        }

        ASTNode::Punctuation(p) => {
            println!("{}{}Punctuation({:?})", prefix, connector, p);
        }

        ASTNode::Empty() => {
            println!("{}{}Empty", prefix, connector);
        }
    }
}

// Convenience function to print from the root
pub fn visualize_ast(ast: &Vec<crate::parser::ast::SymbolNode>) {
    println!("{} \n=== AST Visualization ===\n", MAGENTA);
    for (i, symbol_node) in ast.iter().enumerate() {
        let is_last = i == ast.len() - 1;
        println!("SymbolNode {{ kind: {:?} }}", symbol_node.kind);
        print_ast_tree(&symbol_node.value, String::from(""), is_last);
        if !is_last {
            println!();
        }
    }
    println!("\n=== End of AST ===\n {}", RESET );
}

// Alternative: Print a single AST node
pub fn visualize_single_node(node: &ASTNode) {
    println!("\n=== AST Tree ===\n {}", MAGENTA);
    print_ast_tree(node, String::from(""), true);
    println!("\n=== End ===\n {}", RESET);
}