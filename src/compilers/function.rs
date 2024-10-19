use crate::base_variable::variable::Variable;
use crate::collection::collections::{Array, Dictionary};
use crate::collection::ARRAY_FUNCTIONS;
use crate::function::functions::FunctionTypes;
use crate::node::nodes::ASTNode;
use std::process::exit;

use crate::base_variable::base_types::BaseTypes;
use crate::base_variable::variables::VARIABLE_STACK;
use crate::function::functions::call_function;
use crate::function::functions::Function;
use crate::function::FUNCTION_STACK;
use crate::function_map::FUNCTIONS;
use std::any::Any;

fn add_to_function_stack(func: Function) {
    FUNCTION_STACK.lock().unwrap().push(func);
    //USER_FUNCTIONS.lock().unwrap().push(func);
    // You can still use `dict` after this line because we cloned it
    //println!("dict pushed to stack")
}

fn find_function_in_stack(function_name: &str) -> Function {
    let function_stack = FUNCTION_STACK.lock().unwrap(); // Lock the Mutex, unwrap if the lock is successful

    for function in function_stack.iter() {
        if function_name == function.name {
            return function.clone();
        }
    }

    eprintln!("Function not in user functions");
    exit(1);
}

pub fn parse_function_declaration(expression: &Vec<ASTNode>) -> bool {
    let mut function_name: String = String::new();
    let mut parameters: Vec<Variable> = Vec::new();
    let mut return_type: BaseTypes = BaseTypes::Null;
    let mut function_body: Vec<ASTNode> = Vec::new();

    let mut i = 0;
    while i < expression.len() {
        match &expression[i] {
            ASTNode::Function(f) => {
                // Store function name and return type
                function_name = f.name.clone();
                return_type = BaseTypes::StringWrapper(f.return_type.clone());

                for arg in &f.arguments {
                    let var_type = match arg.2.clone().as_str() {
                        "int" => BaseTypes::Int(0),
                        "float" => BaseTypes::Float(0.0),
                        "string" => BaseTypes::StringWrapper(String::new()),
                        "boolean" => BaseTypes::Bool(false),
                        "char" => BaseTypes::Char('\0'),
                        "null" => BaseTypes::Null,
                        _ => {
                            println!("Syntax Error: Unrecognized type '{}'", arg.2);
                            return false; // Exit if an unrecognized type is found
                        }
                    };

                    let var_value = match arg.2.clone().as_str() {
                        "int" => arg
                            .1
                            .parse::<i32>()
                            .map(BaseTypes::Int)
                            .unwrap_or(BaseTypes::Null),
                        "float" => arg
                            .1
                            .parse::<f64>()
                            .map(BaseTypes::Float)
                            .unwrap_or(BaseTypes::Null),
                        "string" => BaseTypes::StringWrapper(arg.1.clone()),
                        "boolean" => arg
                            .1
                            .parse::<bool>()
                            .map(BaseTypes::Bool)
                            .unwrap_or(BaseTypes::Null),
                        "char" => {
                            if let Some(first_char) = arg.1.chars().next() {
                                BaseTypes::Char(first_char)
                            } else {
                                BaseTypes::Null // Handle empty char case
                            }
                        }
                        "null" => BaseTypes::Null,
                        _ => {
                            println!("Syntax Error: Unrecognized type '{}'", arg.2);
                            return false; // Exit if an unrecognized type is found
                        }
                    };

                    // Create the variable and add it to the parameters
                    let var = Variable::new(
                        arg.0.clone(), // Variable name
                        var_type,      // Variable type
                        var_value,     // Variable value
                    );
                    parameters.push(var);
                }
            }
            ASTNode::LeftCurly => {
                // Now we need to store the function body
                function_body.clear(); // Clear any previous function body
                i += 1; // Move to the next node after '{'

                // Collect nodes until we reach the matching right curly brace
                let mut curly_brace_count = 1; // We've encountered one '{'

                while i < expression.len() {
                    match &expression[i] {
                        ASTNode::LeftCurly => curly_brace_count += 1,
                        ASTNode::RightCurly => {
                            curly_brace_count -= 1;
                            if curly_brace_count == 0 {
                                break; // Found matching '}'
                            }
                        }
                        _ => {}
                    }
                    function_body.push(expression[i].clone());
                    i += 1;
                }

                // After collecting the function body, create the Function object
                let function = Function::new(
                    function_name.clone(),
                    return_type.clone(),
                    parameters.clone(),
                    function_body.clone(),
                );
                // add to FUNCTION_STACK
                println!("Function: {}", function);
                add_to_function_stack(function);
            }
            _ => println!("Unhandled node: {:?}", expression[i]),
        }
        i += 1;
    }
    // Placeholder return value, should likely be more meaningful
    true
}

pub fn parse_function_call(
    expression: &Vec<ASTNode>,
    dot_notation: String,
    array: Option<Array>,
    dictionary: Option<Dictionary>,
    variable: Option<Variable>,
) -> BaseTypes {
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
                    ASTNode::RightParenthesis => {}
                    ASTNode::LeftParenthesis => {
                        parameter_and_value = parse_function_call_arguments(&expression[i + 1..]);
                        break;
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

    match dot_notation.as_str() {
        "dictionary" => {
            return get_function_result(
                function_name,
                &mut parameter_and_value,
                dot_notation,
                None,
                dictionary,
                None,
            );
        }
        "array" => {
            return get_function_result(
                function_name,
                &mut parameter_and_value,
                dot_notation,
                array,
                None,
                None,
            );
        }
        "variable" => {
            return get_function_result(
                function_name,
                &mut parameter_and_value,
                dot_notation,
                None,
                None,
                variable,
            );
        }
        _ => {
            return get_function_result(
                function_name,
                &mut parameter_and_value,
                dot_notation,
                None,
                None,
                None,
            );
        }
    }
    // Handle the result or error
}

pub fn get_function_result(
    function_name: String,
    parameter_and_value: &mut Vec<BaseTypes>,
    dot_notation: String,
    array: Option<Array>,
    dictionary: Option<Dictionary>,
    variable: Option<Variable>,
) -> BaseTypes {
    let std_functions = match FUNCTIONS.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    let array_functions = match ARRAY_FUNCTIONS.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    println!("dot_notation: {}", dot_notation);
    match dot_notation.as_str() {
        "dictionary" => {}
        "array" => {
            if let Some(func) = array_functions.get(function_name.as_str()) {
                //println!("Function call is in ARRAY_FUNCTIONS: {}", function_name);
                // Convert Int to Float for the first two parameters
                for param in parameter_and_value.iter_mut().take(2) {
                    if let BaseTypes::Int(x) = *param {
                        *param = BaseTypes::Float(x as f64);
                    }
                }
                /*
                // Ensure at least two parameters are provided
                if parameter_and_value.len() < 2 {
                    println!(
                "Syntax Error: Not enough parameters supplied to function, {}/2 Provided.",
                parameter_and_value.len()
                    );
                    exit(1);
                }
                */
                let mut params: Vec<Box<dyn Any>> = Vec::new();

                let array_param: Array = array.clone().unwrap();
                println!("Array: {:?}", array_param);

                params.insert(0, Box::new(array_param));

                // Call the function and return the result
                for param in parameter_and_value.iter() {
                    //println!("Parameter: {:?}", param);
                    let boxed_param: Box<dyn Any> = match param {
                        BaseTypes::Int(x) => Box::new(BaseTypes::Int(*x)),
                        BaseTypes::Float(x) => Box::new(BaseTypes::Float(*x)),
                        BaseTypes::StringWrapper(x) => {
                            Box::new(BaseTypes::StringWrapper(x.clone()))
                        }
                        BaseTypes::Bool(x) => Box::new(BaseTypes::Bool(*x)),
                        BaseTypes::Char(x) => Box::new(BaseTypes::Char(*x)),
                        _ => panic!("Unknown parameter type"),
                    };

                    params.push(boxed_param);
                    //add array to params at [0]
                }
                // Create a vector of Box<dyn Any> for parameters
                // Call the function and handle the result
                println!("Params: {:?}", params);

                let result = call_function(func, params);
                // convert the result to the appropriate type
                if result.is::<f64>() {
                    //println!("Result of Function: {:?} of type float", result);
                    return BaseTypes::Float(*result.downcast::<f64>().unwrap());
                }
                if result.is::<i32>() {
                    //println!("Result of Function: {:?} of type int", result);
                    return BaseTypes::Int(*result.downcast::<i32>().unwrap());
                }
                if result.is::<String>() {
                    //println!("Result of Function: {:?} of type string", result);
                    return BaseTypes::StringWrapper(
                        result.downcast::<String>().unwrap().to_string(),
                    );
                }
                if result.is::<bool>() {
                    //println!("Result of Function: {:?} of type bool", result);
                    return BaseTypes::Bool(*result.downcast::<bool>().unwrap());
                } else {
                    return BaseTypes::Null;
                }
            }
            println!("Function call is not in any of the ARRAY_FUNCTIONS.");
        }

        "variable" => {}
        _ => println!("Not calling any function with dot notation"),
    }

    if let Some(func) = std_functions.get(function_name.as_str()) {
        //println!("Function call is in STD_FUNCTIONS: {}", function_name);

        // Convert Int to Float for the first two parameters
        for param in parameter_and_value.iter_mut().take(2) {
            if let BaseTypes::Int(x) = *param {
                *param = BaseTypes::Float(x as f64);
            }
        }
        /*
                // Ensure at least two parameters are provided
                if parameter_and_value.len() < 2 {
                    println!(
                        "Syntax Error: Not enough parameters supplied to function, {}/2 Provided.",
                        parameter_and_value.len()
                    );
                    exit(1);
                }
        */

        let mut params: Vec<Box<dyn Any>> = Vec::new();

        if function_name == "echo" {
            // Check if there's at least one parameter
            if parameter_and_value.len() < 1 {
                println!("Syntax Error: Not enough parameters supplied to function, 0/1 Provided.");
                exit(1);
            }

            // Convert only the first parameter to a String and box it
            let boxed_param: Box<dyn Any> = match &parameter_and_value[0] {
                BaseTypes::Int(x) => Box::new(x.to_string()),
                BaseTypes::Float(x) => Box::new(x.to_string()),
                BaseTypes::StringWrapper(x) => Box::new(x.clone()),
                BaseTypes::Bool(x) => Box::new(x.to_string()),
                BaseTypes::Char(x) => Box::new(x.to_string()),
                _ => panic!("Unknown parameter type"),
            };

            params.push(boxed_param);
        } else {
            // Call the function and return the result
            for param in parameter_and_value.iter() {
                //println!("Parameter: {:?}", param);
                let boxed_param: Box<dyn Any> = match param {
                    BaseTypes::Int(x) => Box::new(*x),
                    BaseTypes::Float(x) => Box::new(*x),
                    BaseTypes::StringWrapper(x) => Box::new(x.clone()),
                    BaseTypes::Bool(x) => Box::new(*x),
                    BaseTypes::Char(x) => Box::new(*x),
                    _ => panic!("Unknown parameter type"),
                };

                params.push(boxed_param);
            }
        }

        // Create a vector of Box<dyn Any> for parameters

        // Call the function and handle the result
        let result = call_function(func, params);
        // convert the result to the appropriate type
        if result.is::<f64>() {
            //println!("Result of Function: {:?} of type float", result);
            return BaseTypes::Float(*result.downcast::<f64>().unwrap());
        }
        if result.is::<i32>() {
            //println!("Result of Function: {:?} of type int", result);
            return BaseTypes::Int(*result.downcast::<i32>().unwrap());
        }
        if result.is::<String>() {
            //println!("Result of Function: {:?} of type string", result);
            return BaseTypes::StringWrapper(result.downcast::<String>().unwrap().to_string());
        }
        if result.is::<bool>() {
            //println!("Result of Function: {:?} of type bool", result);
            return BaseTypes::Bool(*result.downcast::<bool>().unwrap());
        }
        if result.is::<char>() {
            //println!("Result of Function: {:?} of type char", result);
            return BaseTypes::Char(*result.downcast::<char>().unwrap());
        } else {
            return BaseTypes::Null;
        }
    }

    // Function not found

    println!("Function call is not in any of the STD_FUNCTIONS.");

    // Check if the function is a user-defined function
    // If it is, call the function parse the ASTNodes in the body and return result

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
