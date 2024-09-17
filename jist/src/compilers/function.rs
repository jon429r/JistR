use crate::base_variables::base_types::BaseTypes;
use crate::base_variables::variable::Variable;
use crate::node::node::{ASTNode, VariableCallNode};

use crate::base_variables::variables::VARIABLE_STACK;
use crate::function_map::function::Function;
use crate::function_map::{
    STD_FUNCTIONS, STD_FUNCTIONS_DOUBLE, STD_FUNCTIONS_ECHO, STD_FUNCTIONS_SINGLE,
};

pub fn parse_function_declaration_or_call(expression: &Vec<ASTNode>) -> bool {
    //println!("Parsing function.");

    let mut function_name: Option<String> = None;
    let parameters: Vec<Variable> = Vec::new();
    let mut number_of_curly_braces = 0;
    let mut return_type: Option<BaseTypes> = None;
    let mut parsing_function_call = false;
    let mut parameter_and_value: Vec<(String, BaseTypes)> = Vec::new();

    let mut i = 0;
    while i < expression.len() {
        match &expression[i] {
            ASTNode::Function(f) => {
                // Handle function declaration
                // TODO: We need to tell the parser to continue passing expressions here until we
                // reach the end of the function's ending }.
                function_name = Some(f.name.clone());
                //println!("Function name: {}", f.name);

                // Start looking for function body or arguments
                i += 1;
                while i < expression.len() {
                    match &expression[i] {
                        ASTNode::FunctionArguments(f) => {
                            //println!("Function arguments: {}", f.value);
                            //add f to parameters
                            //f: BaseTypes = f.value.clone().into();
                            //parameters.push(f.clone());
                            // TODO create a variable object and add it to parameters vector
                        }
                        ASTNode::ArgumentSeparator => {
                            continue;
                        }
                        ASTNode::ReturnTypeAssignment(r) => {
                            //println!("Return type: {:?}", r.value);
                            return_type = Some(r.value.clone().into());
                        }
                        ASTNode::LeftCurly => {
                            //println!("Function body start.");
                            number_of_curly_braces += 1;
                        }
                        ASTNode::RightCurly => {
                            //println!("Function body end.");
                            if number_of_curly_braces == 0 {
                                println!("Syntax Error: Unmatched curly braces.");
                                return false;
                            } else {
                                number_of_curly_braces -= 1;
                                if number_of_curly_braces == 0 {
                                    //println!("End of function declaration.");
                                    // TODO create new function object and add it to function maps
                                    let function = Function::new(
                                        function_name.as_ref().unwrap().clone(),
                                        parameters.clone(),
                                        return_type.clone().unwrap(),
                                        expression.clone(),
                                    );
                                    break;
                                }
                                break; // End of function declaration
                            }
                        }
                        _ => {
                            println!(
                                "Unhandled node in function declaration: {:?}",
                                expression[i]
                            );
                        }
                    }
                    i += 1;
                }
            }
            ASTNode::FunctionCall(f) => {
                // Handle function call

                parsing_function_call = true;
                function_name = Some(f.name.clone());
                let mut number_of_parentheses = 0;
                //println!("Function call: {}", f.name);

                i += 1;
                while i < expression.len() {
                    match &expression[i] {
                        ASTNode::FunctionCallArguments(_) => {
                            //println!("Parsing function call arguments.");
                            parse_function_call_arguments(&expression[i + 1..]);
                        }
                        /*
                         * TODO Make functions able to take expressions like 1+1 and (1+1), hjk
                         * by making a temp varible declaration and set the alue of expression to
                         * the value of a variable to pass
                         */
                        ASTNode::LeftParenthesis => {
                            if number_of_parentheses == 0 {
                                //println!("Function call starting (.");
                                number_of_parentheses += 1;
                            } else {
                                // TODO do what is within the LeftParenthesis
                            }
                            //println!("Function call starting (.");
                        }
                        ASTNode::RightParenthesis => {
                            //println!("Function call ending ).");
                            if number_of_parentheses == 0 {
                                println!("Syntax Error: Unmatched parenthesis.");
                                return false;
                            } else {
                                number_of_parentheses -= 1;
                                if number_of_parentheses == 0 {
                                    //println!("End of function call.");
                                    break;
                                }
                            }
                            break; // End of function call
                        }
                        ASTNode::SemiColon => {
                            //println!("End of function.");
                            return true;
                        }
                        ASTNode::VariableCall(v) => {
                            //println!("Function argument: {}", v.name);
                            let mut _arg_name = ASTNode::VariableCall(VariableCallNode {
                                name: v.name.clone(),
                            });
                            //var stack for var with this name
                            let mut arg1_value = BaseTypes::StringWrapper(String::new()); // Initialize with default value
                            let mut arg1_name = String::new(); // Initialize with default value
                            for var in unsafe { VARIABLE_STACK.iter() } {
                                if var.name == v.name {
                                    arg1_value = var.value.clone();
                                    //print!("Value: {:?}", arg1_value);
                                    arg1_name = var.name.clone();
                                }
                            }

                            let arg1 = (arg1_name, arg1_value);
                            parameter_and_value.push(arg1);
                        }
                        ASTNode::ArgumentSeparator => {
                            //println!("Comma separator between arguments.");
                        }
                        ASTNode::String(s) => {
                            //println!("Function argument: {}", s.value);
                            let arg1 = (String::new(), BaseTypes::StringWrapper(s.value.clone()));
                            parameter_and_value.push(arg1);
                        }
                        ASTNode::Int(n) => {
                            //println!("Function argument: {}", n.value);
                            let arg1 = (String::new(), BaseTypes::Int(n.value.clone()));
                            parameter_and_value.push(arg1);
                        }
                        ASTNode::Char(c) => {
                            //println!("Function argument: {}", c.value);
                            let arg1 = (String::new(), BaseTypes::Char(c.value.clone()));
                            parameter_and_value.push(arg1);
                        }
                        ASTNode::Bool(b) => {
                            //println!("Function argument: {}", b.value);
                            let arg1 = (String::new(), BaseTypes::Bool(b.value.clone()));
                            parameter_and_value.push(arg1);
                        }
                        ASTNode::Float(
                            f,
                        ) => {
                            //println!("Function argument: {}", f.value);
                            let arg1 = (String::new(), BaseTypes::Float(f.value.clone().into()));
                            parameter_and_value.push(arg1);
                        }
                        _ => {
                            println!("Unhandled node in function call: {:?}", expression[i]);
                        }
                    }
                    i += 1;
                }
            }
            _ => {
                println!("Unhandled node: {:?}", expression[i]);
            }
        }
        i += 1;
    }

    if function_name.is_none() {
        println!("Syntax Error: Function is incomplete.");
        return false;
    }

    //println!("Successfully parsed function call.");
    //check that function is in our 4 function maps

    let std_double = STD_FUNCTIONS_DOUBLE.lock().unwrap();
    let std_single = STD_FUNCTIONS_SINGLE.lock().unwrap();
    let std = STD_FUNCTIONS.lock().unwrap();
    let std_echo = STD_FUNCTIONS_ECHO.lock().unwrap();

    if parsing_function_call {
        if std_double.contains_key(function_name.as_ref().unwrap().as_str()) {
            //println!("Function call is in STD_FUNCTIONS_DOUBLE.");
            //call function with parameters
            if let Some(func) = std_double.get(function_name.as_ref().unwrap().as_str()) {
                let result = func(
                    parameter_and_value[0].1.clone().try_into().unwrap(),
                    parameter_and_value[1].1.clone().try_into().unwrap(),
                );
                println!("Result of Function: {}", result); // Output: 8
            }
        } else if std.contains_key(function_name.as_ref().unwrap().as_str()) {
            //println!("Function call is in STD_FUNCTIONS.");
            if let Some(func) = std.get(function_name.as_ref().unwrap().as_str()) {
                let result = func();
                //println!("Result of Function: {}", result); // Output: 8
            }
        } else if std_echo.contains_key(function_name.as_ref().unwrap().as_str()) {
            //println!("Function call is in STD_FUNCTIONS_ECHO.");

            if let Some(func) = std_echo.get(function_name.as_ref().unwrap().as_str()) {
                if parameter_and_value.len() == 0 {
                    println!("Syntax Error: No parameters supplied to function.");
                    let result = func(String::new());
                    return true;
                }
                let param: String = match parameter_and_value[0].1.clone().to_string().try_into() {
                    Ok(value) => value,
                    Err(_) => {
                        //println!("Failed to convert parameter to String.");
                        String::new()
                    }
                };
                /*print!(
                    "Parameter: {}", param
                );
                */
                let result = func(param);
                //print!("Result of Function: ");
                //return result;
            }
        } else if std_single.contains_key(function_name.as_ref().unwrap().as_str()) {
            //println!("Function call is in STD_FUNCTIONS_SINGLE.");
            if let Some(func) = std_echo.get(function_name.as_ref().unwrap().as_str()) {
                if parameter_and_value.len() == 0 {
                    println!("Syntax Error: No parameters supplied to function.");
                    let result = func(String::new());
                    return true;
                }
                let param: String = match parameter_and_value[0].1.clone().try_into() {
                    Ok(value) => value,
                    Err(_) => {
                        println!("Failed to convert parameter to String.");
                        String::new()
                    }
                };
                let result = func(param);
                //return result;
            }
        } else {
            println!("Function call is not in any of the STD_FUNCTIONS.");
        }
    } else {
        // now we need to create a function object and add it to the function map
        //print!("Creating function object.");
    }

    // now call compile function with parameters if supllie

    true
}

fn parse_function_call_arguments(expression: &[ASTNode]) {
    //println!("*****Processing function call arguments.");

    let mut i = 0;
    while i < expression.len() {
        match &expression[i] {
            ASTNode::VariableCall(v) => {
                //println!("Function argument: {}", v.name);
            }
            ASTNode::ArgumentSeparator => {
                //println!("Comma separator between arguments.");
            }
            ASTNode::RightParenthesis => {
                //println!("End of function call arguments.");
                break;
            }
            _ => {
                println!("Unhandled node in arguments: {:?}", expression[i]);
            }
        }
        i += 1;
    }
}
