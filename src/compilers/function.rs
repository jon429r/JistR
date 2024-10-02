use crate::base_variable::variable::Variable;
use crate::node::nodes::ASTNode;
//use std::collections::HashMap;
use std::process::exit;
//use std::sync::MutexGuard;

use crate::base_variable::base_types::BaseTypes;
use crate::base_variable::variables::VARIABLE_STACK;
use crate::function_map::function::Function;
use crate::function_map::{
    STD_FUNCTIONS, STD_FUNCTIONS_DOUBLE, STD_FUNCTIONS_ECHO, STD_FUNCTIONS_SINGLE, USER_FUNCTIONS,
};

pub fn parse_function_declaration(expression: &Vec<ASTNode>) -> bool {
    let mut function_name: String;
    let parameters: Vec<Variable> = Vec::new();
    let mut number_of_curly_braces = 0;
    let return_type: BaseTypes = BaseTypes::Null;

    let mut i = 0;
    while i < expression.len() {
        match &expression[i] {
            ASTNode::Function(f) => {
                function_name = f.name.clone();
                i += 1;
                while i < expression.len() {
                    match &expression[i] {
                        ASTNode::LeftCurly => number_of_curly_braces += 1,
                        ASTNode::RightCurly => {
                            if number_of_curly_braces == 0 {
                                println!("Syntax Error: Unmatched closing curly brace.");
                                return false;
                            } else {
                                number_of_curly_braces -= 1;
                                if number_of_curly_braces == 0 {
                                    if function_name == "None" {
                                        println!("Syntax Error: Missing function name.");
                                        return false;
                                    }
                                    let function = Function::new(
                                        function_name.clone(),
                                        parameters.clone(),
                                        return_type.clone(),
                                        expression.clone(),
                                    );
                                    let mut user_functions = USER_FUNCTIONS.lock().unwrap();
                                    user_functions.insert(function_name.clone(), function);
                                    break;
                                }
                            }
                        }
                        _ => {
                            println!(
                                "Unhandled node in function declaration: {:?}",
                                expression[i]
                            );
                            return false;
                        }
                    }
                    i += 1;
                }
            }
            _ => println!("Unhandled node: {:?}", expression[i]),
        }
        i += 1;
    }
    true
}

pub fn parse_function_call(expression: &Vec<ASTNode>) -> BaseTypes {
    let mut function_name: String = "None".to_string();
    let mut parameter_and_value: Vec<BaseTypes> = Vec::new();
    let mut i = 0;

    match expression.get(i).unwrap() {
        ASTNode::FunctionCall(f) => {
            function_name = f.name.clone();
            i += 1;
            while i < expression.len() {
                match &expression[i] {
                    ASTNode::FunctionCallArguments(_) => {
                        parameter_and_value = parse_function_call_arguments(&expression[i + 1..]);
                    }
                    ASTNode::LeftParenthesis => {
                        parameter_and_value = parse_function_call_arguments(&expression[i + 1..]);
                    }
                    /*ASTNode::VariableCall(v) => {
                                            let mut arg1_value = BaseTypes::StringWrapper(String::new());
                                            let mut arg1_name = String::new();
                                            for var in unsafe { VARIABLE_STACK.iter() } {
                                                if var.name == v.name {
                                                    arg1_value = var.value.clone();
                                                    arg1_name = var.name.clone();
                                                }
                                            }
                                            let arg1 = (arg1_name, arg1_value);
                                            parameter_and_value.push(arg1);
                                        }
                                        ASTNode::Int(n) => {
                                            let arg1 = (String::new(), BaseTypes::Int(n.value.clone()));
                                            parameter_and_value.push(arg1);
                                        }
                    */
                    _ => println!("Unhandled node in function call: {:?}", expression[i]),
                }
                i += 1;
            }
        }
        _ => println!("Unhandled node: {:?}", expression[i]),
    }

    // Handle the result or error
    get_function_result(function_name, &mut parameter_and_value)
}

pub fn get_function_result(
    function_name: String,
    parameter_and_value: &mut Vec<BaseTypes>,
) -> BaseTypes {
    let std_echo = match STD_FUNCTIONS_ECHO.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let std = match STD_FUNCTIONS.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let std_double = match STD_FUNCTIONS_DOUBLE.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let std_single = match STD_FUNCTIONS_SINGLE.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    if let Some(func) = std_double.get(function_name.as_str()) {
        println!(
            "Function call is in STD_FUNCTIONS_DOUBLE: {}",
            function_name
        );

        // Convert Int to Float for the first two parameters
        for param in parameter_and_value.iter_mut().take(2) {
            if let BaseTypes::Int(x) = *param {
                *param = BaseTypes::Float(x as f64);
            }
        }

        // Ensure at least two parameters are provided
        if parameter_and_value.len() < 2 {
            println!(
                "Syntax Error: Not enough parameters supplied to function, {}/2 Provided.",
                parameter_and_value.len()
            );
            exit(1);
        }

        // Call the function and return the result
        let param1: f64 = parameter_and_value[0]
            .clone()
            .try_into()
            .expect("Failed to convert parameter 1 to f64");
        let param2: f64 = parameter_and_value[1]
            .clone()
            .try_into()
            .expect("Failed to convert parameter 2 to f64");

        let result = func(param1, param2);
        println!("Result of Function: {:?}", result);

        return BaseTypes::Float(result);
    }

    // Function is in STD_FUNCTIONS
    if let Some(func) = std.get(function_name.as_str()) {
        println!("Function call is in STD_FUNCTIONS: {}", function_name);

        // Call the function and return the result
        let result = func();
        println!("Result of Function: {}", result);

        return BaseTypes::Float(result);
    }

    // Function is in STD_FUNCTIONS_ECHO
    if let Some(func) = std_echo.get(&function_name.as_str()) {
        println!("Function call is in STD_FUNCTIONS_ECHO: {}", function_name);

        // Ensure at least one parameter is provided
        if parameter_and_value.is_empty() {
            println!("Syntax Error: No parameters supplied to function.");
            exit(1);
        }

        // Call the function and return the result
        let param: BaseTypes = parameter_and_value[0].clone();
        let result = func(param.into());

        return result.into();
    }

    // Function is in STD_FUNCTIONS_SINGLE
    if let Some(func) = std_single.get(&function_name.as_str()) {
        println!(
            "Function call is in STD_FUNCTIONS_SINGLE: {}",
            function_name
        );

        // Ensure at least one parameter is provided
        if parameter_and_value.is_empty() {
            println!("Syntax Error: No parameters supplied to function.");
            exit(1);
        }

        // Convert the first parameter to f64
        let param: f64 = parameter_and_value[0]
            .clone()
            .try_into()
            .expect("Failed to convert parameter to f64");

        // Call the function and return the result
        let result = func(param);
        println!("Result of Function: {}", result);

        return BaseTypes::Float(result);
    }

    // Function not found

    println!("Function call is not in any of the STD_FUNCTIONS.");
    exit(1);
}

fn parse_function_call_arguments(expression: &[ASTNode]) -> Vec<BaseTypes> {
    let mut arguments: Vec<BaseTypes> = Vec::new();
    let mut i = 0;

    while i < expression.len() {
        match &expression[i] {
            ASTNode::VariableCall(v) => {
                // Process variable call, you could push its value from a variable store
                // For now, let's assume variables are stored in VARIABLE_STACK and extract their values
                for var in unsafe { VARIABLE_STACK.iter() } {
                    if var.name == v.name {
                        arguments.push(var.value.clone());
                    }
                }
            }
            ASTNode::Int(n) => {
                // Handle integer argument
                arguments.push(BaseTypes::Int(n.value));
            }
            ASTNode::Float(f) => {
                // Handle float argument
                arguments.push(BaseTypes::Float(f.value.into()));
            }
            ASTNode::String(s) => {
                // Handle string argument
                arguments.push(BaseTypes::StringWrapper(s.value.clone()));
            }
            ASTNode::Bool(b) => {
                // Handle boolean argument
                arguments.push(BaseTypes::Bool(b.value));
            }
            ASTNode::Char(c) => {
                // Handle char argument
                arguments.push(BaseTypes::Char(c.value));
            }
            ASTNode::ArgumentSeparator => {
                // Simply skip over argument separators (commas)
            }
            ASTNode::RightParenthesis => {
                // End of arguments, break out of the loop
                break;
            }
            _ => {
                println!("Unhandled node in arguments: {:?}", expression[i]);
            }
        }
        i += 1;
    }

    // Return the collected arguments
    //println!("@@@@@@@@@@@Arguments: {:?}", arguments);
    arguments
}
