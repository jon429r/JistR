mod ast;
pub mod base_variable;
mod collection;
pub mod compiler;
mod function_map;
mod node;
pub mod token_type;
//mod tokenizer;

mod compilers {
    pub mod collection;
    pub mod function;
    pub mod operation;
    pub mod variable;
}

mod statement_tokenizer {
    pub mod basic_tokenizer;
    pub mod collection_tokenizer;
    pub mod function_tokenizer;
    pub mod tests;
    pub mod tokenizer;
    pub mod variable_tokenizer;
}
use crate::statement_tokenizer::tokenizer::tokenizers::ParseInfo;

use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process::exit;

use base_variable::variables::VARIABLE_STACK;
use compiler::compilers::route_to_parser;
use node::nodes::match_token_to_node;
use node::nodes::ASTNode;
use statement_tokenizer::tokenizer::tokenizers::tokenize;

///
/// This function checks if the file extension is valid. IE: .jist
///
fn check_file_extension(file_path: String) -> Result<bool, Box<dyn Error>> {
    let ext = Path::new(&file_path).extension().and_then(OsStr::to_str);
    let valid_ext = "jist";
    if ext == Some(valid_ext) {
        Ok(true)
    } else {
        Err("Invalid file extension".into())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("No file path provided".into());
    }
    let file_path = &args[1];
    match check_file_extension(file_path.to_owned()) {
        Ok(true) => {
            //println!("File path is valid");
        }
        Err(_) => {
            return Err("File path not valid: Does not have extension .jist".into());
        }
        _ => {
            return Err("Some error occurred".into());
        }
    }

    // Read the file contents
    let contents = fs::read_to_string(file_path)?;
    //println!("{}", contents);

    // Tokenize each line and collect AST nodes
    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    let _ast_nodes: Vec<ASTNode> = Vec::new();

    for line in lines {
        let tokens = tokenize(line);
        //println!("Tokens {:?}", tokens);
        //
        let mut hasroot = false;
        let mut tokenized_expression = Vec::new();
        for parsed_info in tokens {
            let node = match_token_to_node(parsed_info);
            //println!("Node: {:?}", node);
            match node {
                ASTNode::SemiColon => {
                    if !hasroot {
                        //throw syntax error
                        print!("Syntax error expression must be more than semicolon");
                        exit(1);
                    } else {
                        //send to compiler.rs
                        route_to_parser(&mut tokenized_expression);
                    }
                }
                _ => {
                    if !hasroot {
                        hasroot = true;
                        tokenized_expression.push(node);
                    } else {
                        tokenized_expression.push(node);
                    }
                }
            }
        }

        // Display the collected AST nodes
        /*
        for node in &ast_nodes {
            let indent = " ".repeat(4);
            match node {
                ASTNode::Bool(b) => println!("{}BoolNode: Value: {}", indent, b.value),
                ASTNode::Variable(v) => println!("{}VariableNode: Type: {}, Value: {}", indent, v.var_type, v.value),
                ASTNode::Int(n) => println!("{}IntNode: Value: {}", indent, n.value),
                ASTNode::Operator(o) => println!("{}OperatorNode: Operator: {}", indent, o.operator),
                ASTNode::Function(f) => println!("{}FunctionNode: Name: {}", indent, f.name),
                ASTNode::String(s) => println!("{}StringNode: Value: {}", indent, s.value),
                ASTNode::Char(c) => println!("{}CharNode: Value: {}", indent, c.value),
                ASTNode::Assignment(a) => println!("{}AssignmentNode: Value: {}", indent, a.value),
                ASTNode::VarTypeAssignment(v) => println!("{}VarTypeAssignmentNode: Value: {}", indent, v.value),
                ASTNode::FunctionCall(f) => println!("{}FunctionCallNode: Value: {}", indent, f.name),
                ASTNode::VariableCall(v) => println!("{}VariableCallNode: Value: {}", indent, v.name),
                ASTNode::VariableType(v) => println!("{}VariableTypeNode: Value: {}", indent, v.value),
                ASTNode::VariableValue(v) => println!("{}VariableValueNode: Value: {}", indent, v.value),
                ASTNode::FunctionArguments(f) => println!("{}FunctionArgumentsNode: Value: {}", indent, f.value),
                ASTNode::AssignmentOperator(a) => println!("{}AssignmentOperatorNode: Value: {}", indent, a.operator),
                ASTNode::ReturnTypeAssignment(r) => println!("{}ReturnTypeAssignmentNode: Value: {}", indent, r.value),
                ASTNode::Comment(c) => println!("{}CommentNode: Value: {}", indent, c.value),
                ASTNode::SemiColon => println!("{}SemicolonNode", indent),
                ASTNode::LeftParenthesis => println!("{}LeftParenthesisNode", indent),
                ASTNode::RightParenthesis => println!("{}RightParenthesisNode", indent),
                ASTNode::ArgumentSeparator => println!("{}ArgumentSeparatorNode", indent),
                ASTNode::LeftCurly => println!("{}LeftCurlyNode", indent),
                ASTNode::RightCurly => println!("{}RightCurlyNode", indent),
                ASTNode::None => println!("{}NoneNode", indent),
            }
        }*/
    }
    //print variable stack
    println!("\nVariable stack:");
    for variable in unsafe { VARIABLE_STACK.iter() } {
        variable.print();
    }
    Ok(())
}

#[cfg(test)]
mod main_test {
    use assert_cmd::Command;

    #[test]
    fn test_check_file_extension() {
        let file_path = "test.jist";
        let result = super::check_file_extension(file_path.to_string());
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_check_file_extension_invalid() {
        let file_path = "test.txt";
        let result = super::check_file_extension(file_path.to_string());
        assert_eq!(result.is_err(), true);
    }
}

#[cfg(test)]
mod test_input_output {
    use assert_cmd::Command;
    use predicates::prelude::*;

    // Pass in a file path and check if the output is correct
    #[test]
    fn test_int_variable_declarations() {
        let file_path = "test_files/int_variable_declaration.jist";

        // Run the program and check the output
        let mut cmd = Command::cargo_bin("jist").unwrap();

        cmd.arg(file_path)
            .assert()
            .success() // Asserting that the command was successful
            .stdout(predicate::str::contains(
                "Variable Name: a\nVariable Type: Int(0)\nVariable Value: Int(1)",
            ));
    }

    #[test]
    fn test_string_variable_declaration() {
        let file_path = "test_files/string_variable_declaration.jist";

        let mut cmd = Command::cargo_bin("jist").unwrap();

        cmd.arg(file_path)
            .assert()
            .success()
            .stdout(predicate::str::contains(
                "Variable info: a, StringWrapper(\"\\\"Hello World\\\"\"), StringWrapper(\"\")",
            ));
    }
    /*
        #[test]
        fn test_bool_variable_declaration() {
            let file_path = "test_files/boolean_variable_declaration.jist";
            let mut cmd = Command::cargo_bin("jist").unwrap();
            cmd.arg(file_path)
                .assert()
                .success()
                .stdout(predicate::str::contains(
                    "Variable Name: a\nVariable Type: Bool(0)\nVariable Value: Bool(true)",
                ));
        }

        #[test]
        fn test_char_variable_declaration() {
            let file_path = "test_files/char_variable_declaration.jist";
            let mut cmd = Command::cargo_bin("jist").unwrap();
            cmd.arg(file_path)
                .assert()
                .success()
                .stdout(predicate::str::contains(
                    "Variable info: a, CharWrapper('a'), CharWrapper('')",
                ));
        }
    */
    #[test]
    fn test_float_variable_declaration() {
        let file_path = "test_files/float_variable_declaration.jist";

        let mut cmd = Command::cargo_bin("jist").unwrap();

        cmd.arg(file_path)
            .assert()
            .success()
            .stdout(predicate::str::contains(
                "Variable info: a, Float(3.141590118408203), Float(0.0)",
            ));
    }
}
