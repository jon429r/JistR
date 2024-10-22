mod ast;
pub mod base_variable;
mod collection;
pub mod compiler;
pub mod function;
mod function_map;
pub mod globals;
pub mod highlighter;
mod node;
pub mod token_type;

mod compilers {
    pub mod collection;
    pub mod conditional;
    pub mod function;
    pub mod loops;
    pub mod operation;
    pub mod variable;
}

mod statement_tokenizer {
    pub mod basic_tokenizer;
    pub mod collection_tokenizer;
    pub mod conditional_tokenizer;
    pub mod function_tokenizer;
    pub mod loop_tokenizer;
    pub mod tests;
    pub mod tokenizer;
    pub mod variable_tokenizer;
}

use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

//use crate::collection::collections::{Array, Dictionary};
use base_variable::variables::VARIABLE_STACK;
use compiler::compilers::route_to_parser;
use globals::MAKE_LOOP;
use node::nodes::match_token_to_node;
use node::nodes::ASTNode;
use statement_tokenizer::tokenizer::tokenizers::tokenize;

use crate::collection::{ARRAY_STACK, DICTIONARY_STACK};
use crate::function::FUNCTION_STACK;
//use lazy_static::lazy_static;
//use std::sync::Mutex;

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

///
///This function prints the array stack for dev purposes
///
fn print_array_stack() {
    let array_stack = ARRAY_STACK.lock().unwrap(); // Lock the mutex
    for array in array_stack.iter() {
        println!("{}", array); // Now we can iterate over the Vec
    }
}

///
///This function prints the dictionary stack for dev purposes
///
fn print_dictionary_stack() {
    let dict_stack = DICTIONARY_STACK.lock().unwrap(); // Lock the mutex
    for dict in dict_stack.iter() {
        println!("{}", dict); // Now we can iterate over the Vec
    }
}

///
///This function prints the function stack for dev purposes
///
fn print_function_stack() {
    let function_stack = FUNCTION_STACK.lock().unwrap(); // Lock the mutex
    for function in function_stack.iter() {
        println!("{}", function); // Now we can iterate over the Vec
    }
}

fn tokenize_input(input: &str) -> Vec<ASTNode> {
    let tokens = tokenize(input.to_string());
    let mut tokenized_expression = Vec::new();

    for parsed_info in tokens {
        let node = match_token_to_node(parsed_info);
        tokenized_expression.push(node);
    }

    tokenized_expression
}

fn parse_tokens(tokens: Vec<ASTNode>) -> Result<(), Box<dyn Error>> {
    let mut tokenized_expression = Vec::new();
    let mut result = true;

    for node in tokens {
        match node {
            ASTNode::SemiColon => {
                result = route_to_parser(&mut tokenized_expression, None)?;
                tokenized_expression.clear(); // Clear after processing
            }
            _ => {
                tokenized_expression.push(node); // Accumulate tokens
            }
        }

        // If needed, handle error states, loops, etc.
        while unsafe { MAKE_LOOP } {
            result = route_to_parser(&mut tokenized_expression, None)?;
        }
    }

    Ok(())
}

///
///This function reads the file and parses it, it was added to support multiple lines of code,
///multiline coding statements and later multiple files
///
// Global var if_else_skip
use crate::globals::IF_ELSE_SKIP;

pub fn parse_lines(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    let mut brace_count = 0;
    let mut bracket_count = 0;
    let mut current_line = String::new();
    let mut finished_lines: Vec<String> = Vec::new();

    for (line_number, line) in lines.iter().enumerate() {
        for ch in line.chars() {
            match ch {
                '{' => {
                    brace_count += 1;
                    current_line.push(ch);
                }
                '}' => {
                    brace_count -= 1;
                    current_line.push(ch);
                    if brace_count < 0 {
                        return Err(format!(
                            "Unmatched closing curly brace at line {}",
                            line_number + 1
                        )
                        .into());
                    }
                    if brace_count == 0 && bracket_count == 0 {
                        finished_lines.push(current_line.clone());
                        current_line.clear();
                    }
                }
                '[' => {
                    bracket_count += 1;
                    current_line.push(ch);
                }
                ']' => {
                    bracket_count -= 1;
                    current_line.push(ch);
                    if bracket_count < 0 {
                        return Err(format!(
                            "Unmatched closing square bracket at line {}",
                            line_number + 1
                        )
                        .into());
                    }
                }
                ';' => {
                    current_line.push(ch);
                    if brace_count == 0 && bracket_count == 0 {
                        finished_lines.push(current_line.clone());
                        current_line.clear();
                    }
                }
                _ => current_line.push(ch),
            }
        }
    }
    return Ok(finished_lines);
}

fn parse_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    let _ast_nodes: Vec<ASTNode> = Vec::new();
    let finished_lines: Vec<String> = parse_lines(file_path).unwrap();
    let mut tokenized_expression = Vec::new();

    for line in finished_lines {
        let tokens = tokenize(line.clone());
        //println!("tokens: {:?}", tokens);
        let mut hasroot = true;
        let mut first_node: ASTNode = ASTNode::None;
        let mut result: bool;

        for (i, parsed_info) in tokens.iter().enumerate() {
            let node = match_token_to_node(parsed_info.clone());
            if i == 0 {
                first_node = node.clone();
            }

            match node {
                ASTNode::SemiColon => {
                    // Check if the expression is valid before processing
                    if tokenized_expression.is_empty() {
                        println!("Syntax error: expression must be more than a semicolon");
                        std::process::exit(1);
                    }

                    // Route to parser only if there are valid tokens
                    match first_node.clone() {
                        ASTNode::While(_) => {
                            result = route_to_parser(&mut tokenized_expression, None)?
                        }
                        ASTNode::If(_) => {
                            result = route_to_parser(&mut tokenized_expression, None)?
                        }
                        ASTNode::Elif(_) => {
                            if unsafe { IF_ELSE_SKIP } {
                                break; // Skip processing if IF_ELSE_SKIP is true
                            } else {
                                result = route_to_parser(&mut tokenized_expression, None)?;

                                while unsafe { MAKE_LOOP } {
                                    result = route_to_parser(&mut tokenized_expression, None)?;
                                }
                            }
                        }
                        ASTNode::Else => {
                            if unsafe { IF_ELSE_SKIP } {
                                unsafe { IF_ELSE_SKIP = false }; // Reset IF_ELSE_SKIP
                                break; // Skip further parsing
                            } else {
                                result = route_to_parser(&mut tokenized_expression, None)?;
                                while unsafe { MAKE_LOOP } {
                                    result = route_to_parser(&mut tokenized_expression, None)?;
                                }
                            }
                        }
                        _ => {
                            result = route_to_parser(&mut tokenized_expression, None)?;
                            while unsafe { MAKE_LOOP } {
                                result = route_to_parser(&mut tokenized_expression, None)?;
                            }
                        }
                    }

                    // check result if Error throw error with line number and exit
                    if !result {
                        println!("Error in parsing line: {}", line);
                        std::process::exit(1);
                    }

                    // Clear tokenized_expression after processing
                    tokenized_expression.clear();
                }
                _ => {
                    hasroot = true; // Mark that we have a valid root
                    tokenized_expression.push(node); // Accumulate tokens
                }
            }
        }
    }

    Ok(())
}

use crossterm::{
    event::{self, KeyCode, KeyEvent},
    terminal::{self, disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
};
use std::io::{self, Write};

fn get_input(
    history: &mut Vec<String>,
    history_index: &mut usize,
) -> Result<String, Box<dyn Error>> {
    let mut current_input = String::new();

    loop {
        // Clear the current line
        print!("\rjist> {}", current_input);
        io::stdout().flush()?;

        // Read the next event
        if event::poll(std::time::Duration::from_millis(500))? {
            if let event::Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Enter => {
                        println!(); // Move to the next line
                        break; // Exit input loop
                    }
                    KeyCode::Up => {
                        if *history_index > 0 {
                            *history_index -= 1;
                            current_input = history[*history_index].clone(); // Load previous command
                        }
                    }
                    KeyCode::Down => {
                        if *history_index < history.len() {
                            *history_index += 1;
                            if *history_index == history.len() {
                                current_input.clear(); // Clear if at the end of history
                            } else {
                                current_input = history[*history_index].clone();
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        current_input.pop(); // Remove last character
                    }
                    KeyCode::Char(c) => {
                        current_input.push(c); // Add character to input
                    }
                    _ => {}
                }
            }
        }

        // Move the cursor to the beginning of the line and clear the line
        print!("\rjist> {}  ", current_input); // Clear the line
        io::stdout().flush()?;
    }

    // Save command to history
    if !current_input.is_empty() {
        history.push(current_input.clone());
        *history_index = history.len(); // Reset history index
    }

    Ok(current_input) // Return the final input
}

fn start_repl() -> Result<(), Box<dyn Error>> {
    let mut history: Vec<String> = Vec::new();
    let mut history_index = 0;

    println!("JistR 0.1.1 (Released: Oct 21 2024)");
    println!("Welcome to the JistR Read-Eval-Print-Loop!");
    println!("Type 'exit();' to exit the REPL");

    loop {
        enable_raw_mode()?;
        println!();
        let input = get_input(&mut history, (&mut history_index).into())?;

        disable_raw_mode()?;
        if input.is_empty() {
            continue;
        }

        if input == "exit();" {
            println!("Exiting REPL... Goodbye!");
            break;
        }

        // Tokenize input
        let tokens = tokenize_input(&input);

        // Parse tokens
        if let Err(e) = parse_tokens(tokens) {
            eprintln!("Error in parsing: {}", e);
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        // Start the REPL if no file is passed
        println!("Starting REPL...");
        start_repl()?; // Propagate errors from REPL if any
        return Ok(());
    }

    // File path provided as an argument
    let file_path = &args[1];

    // Check if the file has the correct extension
    match check_file_extension(file_path.clone()) {
        Ok(true) => {
            // File extension is valid, continue with parsing
        }
        Ok(false) => {
            return Err("File path not valid: Does not have .jist extension".into());
        }
        Err(e) => {
            return Err(format!("Failed to check file extension: {}", e).into());
        }
    }

    // Parse the file and handle any errors
    if let Err(e) = parse_file(file_path) {
        eprintln!("Failed to parse file: {}", e);
        return Err(format!("Error occurred while parsing the file: {}", e).into());
    }

    // After parsing, print the variable stack and other stacks
    println!("\n\nStack:");
    for variable in unsafe { VARIABLE_STACK.iter() } {
        variable.print();
    }

    // Print array, dictionary, and function stacks
    print_array_stack();
    print_dictionary_stack();
    print_function_stack();

    Ok(())
}

#[cfg(test)]
mod main_test {

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
                "Variable Name: a\nVariable Type: Int\nVariable Value: 1",
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
                "Variable Name: a\nVariable Type: String\nVariable Value: Hello World",
            ));
    }

    #[test]
    fn test_bool_variable_declaration() {
        let file_path = "test_files/boolean_variable_declaration.jist";
        let mut cmd = Command::cargo_bin("jist").unwrap();
        cmd.arg(file_path)
            .assert()
            .success()
            .stdout(predicate::str::contains(
                "Variable Name: a\nVariable Type: Bool\nVariable Value: true",
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
                "Variable Name: a\nVariable Type: Char\nVariable Value: a",
            ));
    }

    #[test]
    fn test_float_variable_declaration() {
        let file_path = "test_files/float_variable_declartion.jist";

        let mut cmd = Command::cargo_bin("jist").unwrap();

        cmd.arg(file_path)
            .assert()
            .success()
            .stdout(predicate::str::contains(
                "Variable Name: a\nVariable Type: Float\nVariable Value: 3.141590118408203",
            ));
    }

    #[test]
    fn test_dict_boolean_string_collection_declaration() {
        let file_path = "test_files/dict_boolean_string_collection_declaration.jist";
        let mut cmd = Command::cargo_bin("jist").unwrap();
        cmd.arg(file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            r#"a: Dict<bool, string> = {"true" => true, "false" => false, "true" => not false}"#,
        ));
    }

    #[test]
    fn test_array_boolean_collection_declaration() {
        let file_path = "test_files/array_boolean_collection_declaration.jist";
        let mut cmd = Command::cargo_bin("jist").unwrap();
        cmd.arg(file_path)
            .assert()
            .success()
            .stdout(predicate::str::contains(
                r#"a: Array<bool> = [true, false, true]"#,
            ));
    }

    #[test]
    fn test_array_char_collection_declaration() {
        let file_path = "test_files/array_char_collection_declaration.jist";
        let mut cmd = Command::cargo_bin("jist").unwrap();
        cmd.arg(file_path)
            .assert()
            .success()
            .stdout(predicate::str::contains(r#"a: Array<char> = [a, b, c]"#));
    }

    #[test]
    fn test_array_float_collection_declaration() {
        let file_path = "test_files/array_float_collection_declaration.jist";
        let mut cmd = Command::cargo_bin("jist").unwrap();
        cmd.arg(file_path)
            .assert()
            .success()
            .stdout(predicate::str::contains(
                r#"a: Array<float> = [1.2300000190734863, 2.2300000190734863, 3.2300000190734863]"#,
            ));
    }

    #[test]
    fn test_array_int_collection_declaration() {
        let file_path = "test_files/array_int_collection_declaration.jist";
        let mut cmd = Command::cargo_bin("jist").unwrap();
        cmd.arg(file_path)
            .assert()
            .success()
            .stdout(predicate::str::contains(r#"a: Array<int> = [1, 2, 3]"#));
    }

    #[test]
    fn test_boolean_variable_declaration() {
        let file_path = "test_files/boolean_variable_declaration.jist";
        let mut cmd = Command::cargo_bin("jist").unwrap();
        cmd.arg(file_path)
            .assert()
            .success()
            .stdout(predicate::str::contains(r#"true"#));
    }

    #[test]
    fn test_dict_float_int_collection_declaration() {
        let file_path = "test_files/dict_float_int_collection_declaration.jist";
        let mut cmd = Command::cargo_bin("jist").unwrap();
        cmd.arg(file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            r#"a: Dict<float, int> = {"1.100000023841858" => 1, "2.0999999046325684" => 2, "3.9000000953674316" => 4}"#,
        ));
    }

    #[test]
    fn test_dict_int_char_collection_declaration() {
        let file_path = "test_files/dict_int_char_collection_declaration.jist";
        let mut cmd = Command::cargo_bin("jist").unwrap();
        cmd.arg(file_path)
            .assert()
            .success()
            .stdout(predicate::str::contains(
                r#"a: Dict<int, char> = {"1" => a, "2" => b, "3" => c}"#,
            ));
    }

    #[test]
    fn test_dict_int_string_collection_declaration() {
        let file_path = "test_files/dict_int_string_collection_declaration.jist";
        let mut cmd = Command::cargo_bin("jist").unwrap();
        cmd.arg(file_path)
            .assert()
            .success()
            .stdout(predicate::str::contains(
                r#"a: Dict<int, string> = {"1" => one, "2" => two, "3" => three}"#,
            ));
    }

    #[test]
    fn test_dict_string_float_collection_declaration() {
        let file_path = "test_files/dict_string_float_collection_declaration.jist";
        let mut cmd = Command::cargo_bin("jist").unwrap();
        cmd.arg(file_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            r#"a: Dict<string, float> = {"one" => 1.100000023841858, "two" => 2.0999999046325684, "three" => 3.0999999046325684}"#,
        ));
    }
}
