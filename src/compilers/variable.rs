use std::process::exit;

use rand::seq::index;

use crate::base_variable::base_types::BaseTypes;
use crate::base_variable::variable::Variable;
use crate::base_variable::variables::VARIABLE_STACK;
use crate::compilers::function::parse_function_call;
use crate::node::nodes::ASTNode;
use crate::node::nodes::VariableCallNode;
use crate::node::nodes::{IntNode, OperatorNode};
use crate::statement_tokenizer::variable_tokenizer;

///
///This Function takes in an ASTNode and returns a tuple of the variable name and its value
///
pub fn parse_variable_call(node: &ASTNode) -> (String, BaseTypes) {
    match node {
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
            //parameter_and_value.push(arg1);
            return arg1;
        }
        _ => {
            println!("Syntax Error: Expected a variable call.");
            exit(1)
        }
    }
}

///
/// This function is called when the first token in the expression is a variable
/// can be used to set the value of the variable to something else
///

pub fn compile_variable_call(exp_stack: &mut Vec<ASTNode>) -> bool {
    // Ensure there is at least one node in the stack
    if exp_stack.is_empty() {
        println!("Syntax Error: Empty expression stack.");
        return false;
    }

    // Find the first variable call node
    let index = exp_stack
        .iter()
        .position(|node| matches!(node, ASTNode::VariableCall(_)))
        .unwrap_or_else(|| {
            println!("Syntax Error: Expected a variable call.");
            exit(1);
        });

    let first_node = &exp_stack[index];

    // Match against the first node to see if it is a variable call
    if let ASTNode::VariableCall(v) = first_node {
        let mut variable = Variable::new(String::new(), BaseTypes::Null, BaseTypes::Null);

        // Search for the variable in the global VARIABLE_STACK
        for var in unsafe { &VARIABLE_STACK } {
            if var.name == v.name {
                variable = var.clone();
                println!(
                    "Variable found: Name = {}, Value = {:?}, Type = {:?}",
                    variable.name, variable.value, variable.var_type
                );
                break; // Break after finding the variable
            }
        }

        // If variable wasn't found, return false
        if variable.name.is_empty() {
            println!("Syntax Error: Variable '{}' not found in stack.", v.name);
            return false;
        }

        // Process further if there are additional nodes in exp_stack
        for node in &exp_stack[index + 1..] {
            match node {
                ASTNode::AssignmentOperator(_) => {
                    let mut operation_stack = exp_stack[index + 1..].to_vec();
                    operation(&mut operation_stack);
                    // Set the value to the variable based on the operation result
                    if let Some(next_node) = operation_stack.first() {
                        let value: BaseTypes = next_node.into();
                        variable.set_value(value);
                    }
                    println!(
                        "Processed assignment operator with updated variable: {:?}",
                        variable
                    );
                    return true;
                }
                ASTNode::Operator(o) => match o.operator.as_str() {
                    "++" => {
                        variable.increment();
                        println!("Incremented variable: {:?}", variable);
                        return true;
                    }
                    "--" => {
                        variable.decrement();
                        println!("Decremented variable: {:?}", variable);
                        return true;
                    }
                    _ => {
                        println!("Syntax Error: Unrecognized operator '{}'", o.operator);
                        return false;
                    }
                },
                _ => {
                    // Handle additional argument processing here
                    let value: BaseTypes = node.into();
                    variable.set_value(value.clone());
                    println!("Set variable value to: {:?}", value);
                }
            }
        }

        return true; // Successfully processed the variable call
    } else {
        println!(
            "Syntax Error: Expected a variable call but found {:?}",
            first_node
        );
        return false; // Return false if the first node is not a variable call
    }
}

///
///This function takes in a mutable reference to a vector of ASTNodes and parses the variable
///declaration returning end after parsing the variable declaration
///
pub fn parse_variable_declaration(exp_stack: &mut Vec<ASTNode>) -> bool {
    let mut var_name: Option<String> = None;
    let mut var_type: Option<BaseTypes> = None;
    let mut assignment_operator: Option<String> = None;
    let mut inside_assignment = false;
    let mut value: BaseTypes = BaseTypes::Null;
    let mut var_value = ASTNode::Int(IntNode { value: 0 });
    let mut first: Option<ASTNode> = Option::None;
    let mut parenthesis: bool = false;
    let mut variable_call_values: Vec<(String, BaseTypes)> = Vec::new();

    let mut index = 0;
    while index < exp_stack.len() {
        let node = &exp_stack[index];
        match node {
            ASTNode::Variable(v) => {
                var_name = Some(v.value.clone());
            }
            ASTNode::VariableType(v) => {
                var_type = match v.value.as_str() {
                    "int" => Some(BaseTypes::Int(0)),
                    "float" => Some(BaseTypes::Float(0.0)),
                    "string" => Some(BaseTypes::StringWrapper(String::new())),
                    "boolean" => Some(BaseTypes::Bool(false)),
                    "char" => Some(BaseTypes::Char('\0')),
                    _ => {
                        println!("Syntax Error: Unrecognized type '{}'", v.value);
                        return false;
                    }
                };
            }
            ASTNode::AssignmentOperator(a) => {
                assignment_operator = Some(a.operator.clone());
                inside_assignment = true;
            }
            ASTNode::FunctionCall(c) => {
                if inside_assignment {
                    /*println!(
                                            "Function call found within variable declaration. Expression: {:?}",
                                            exp_stack
                                        );
                    */
                    let mut function_call_stack = exp_stack.clone();
                    function_call_stack.reverse();
                    function_call_stack.pop();
                    function_call_stack.pop();
                    function_call_stack.pop();
                    function_call_stack.reverse();

                    //println!("Function call stack: {:?}", function_call_stack);

                    let result = parse_function_call(&function_call_stack);
                    // TODO set value to result
                    value = result.into();

                    /*println!(
                                            "New variable = name: {}, value: {:?}, type: {:?}",
                                            var_name.clone().unwrap(),
                                            value,
                                            var_type.clone().unwrap()
                                        );
                    */

                    let _variable = Variable::new(var_name.unwrap(), value, var_type.unwrap());
                    return true;
                }
            }
            ASTNode::VariableCall(c) => {
                if inside_assignment {
                    let result = parse_variable_call(&node);
                    //add the value back into the epression at same index
                    exp_stack.insert(
                        index,
                        ASTNode::VariableCall(VariableCallNode {
                            name: result.0.clone(),
                        }),
                    );
                    variable_call_values.push(result);
                } else {
                    println!("Syntax Error: Variable call outside of assignment.");
                    return false;
                }
            }
            ASTNode::Int(n) => {
                if inside_assignment {
                    var_value = operation(exp_stack);
                    if let ASTNode::Int(n) = var_value {
                        value = BaseTypes::Int(n.value);
                    } else {
                        println!("Syntax Error: Expected an integer after the operator.");
                        return false;
                    }
                    break;
                }
            }
            //include other types: float, string, bool, char
            ASTNode::Float(f) => {
                if inside_assignment {
                    first = Some(ASTNode::Float(f.clone()));
                    var_value = operation(exp_stack);
                    if let ASTNode::Float(f) = var_value {
                        value = BaseTypes::Float(f.value.into());
                    } else {
                        println!("Syntax Error: Expected a float after the operator.");
                        return false;
                    }
                    break;
                }
            }
            ASTNode::String(s) => {
                if inside_assignment {
                    //print!("String: {}", s.value);
                    first = Some(ASTNode::String(s.clone()));
                    var_value = operation(exp_stack);
                    if let ASTNode::String(s) = var_value {
                        value = BaseTypes::StringWrapper(s.value.clone());
                    } else {
                        println!("Syntax Error: Expected a string after the assignment_operator.");
                        return false;
                    }
                    break;
                }
            }
            ASTNode::Bool(b) => {
                if inside_assignment {
                    value = BaseTypes::Bool(b.value);
                } else {
                    println!("Syntax Error: Bool outside of assignment.");
                    return false;
                }
            }
            ASTNode::Char(c) => {
                if inside_assignment {
                    value = BaseTypes::Char(c.value);
                } else {
                    println!("Syntax Error: Char outside of assignment.");
                    return false;
                }
            }
            ASTNode::LeftParenthesis => {
                parenthesis = true;
            }
            ASTNode::RightParenthesis => {}
            ASTNode::FunctionCall(c) => {
                if inside_assignment {
                    let resullt = parse_variable_call(&node);
                    //add the value back into the epression at same index
                    exp_stack.insert(
                        index,
                        ASTNode::VariableCall(VariableCallNode {
                            name: resullt.0.clone(),
                        }),
                    );
                //variable_call_values.push(resullt);
                } else {
                    println!("Syntax Error: Function call outside of assignment.");
                    return false;
                }
            }
            _ => {
                println!(
                    "Syntax Error: Unhandled node while parsing variable declaration: {:?}",
                    node
                );
                return false;
            }
        }
        index += 1;
    }

    if var_name.is_none() || var_type.is_none() || assignment_operator.is_none() {
        println!("Syntax Error: Missing variable components.");
        return false;
    }

    let variable = Variable::new(var_name.unwrap(), value, var_type.unwrap());
    // Add to VARIABLE_STACK
    unsafe {
        VARIABLE_STACK.push(variable.clone());
    }
    println!("New variable: {:?}", variable.clone());
    true
}

///
///This Function is in the variable module and takes in a mut ref of a vector of ASTNodes
///It returns a BaseType result of the parsed expression
///
pub fn operation(exp_stack: &mut Vec<ASTNode>) -> ASTNode {
    //cloned_exp_stack.reverse();
    //println!("Cloned stack after reversed: {:?}", cloned_exp_stack);

    //pop 2 elements from the stack
    let mut first_node: Option<ASTNode> = None;
    let mut parenthesis: bool = false;

    exp_stack.reverse();
    exp_stack.pop();
    exp_stack.pop();
    exp_stack.pop(); // Remove the '=' from original stack
    exp_stack.reverse();

    let mut first: bool = false;
    let mut operator: ASTNode = ASTNode::Operator(OperatorNode {
        operator: String::new(),
    });
    let mut left: ASTNode = ASTNode::Int(IntNode { value: 0 });
    let _right: ASTNode = ASTNode::Int(IntNode { value: 0 });

    while let Some(top) = exp_stack.pop() {
        match top {
            ASTNode::AssignmentOperator(o) => {
                operator = ASTNode::AssignmentOperator(o);
            }

            ASTNode::Operator(o) => {
                operator = ASTNode::Operator(o);
            }
            ASTNode::Int(n) => {
                if !first {
                    first = true;
                    left = ASTNode::Int(n);
                } else {
                    let right = ASTNode::Int(n);
                    first_node = Some(parse_operator(&left, &operator, &right));
                }
            }
            ASTNode::Float(f) => {
                if !first {
                    first = true;
                    left = ASTNode::Float(f);
                } else {
                    let right = ASTNode::Float(f);
                    first_node = Some(parse_operator(&left, &operator, &right));
                }
            }
            ASTNode::String(s) => {
                if !first {
                    first = true;
                    left = ASTNode::String(s);
                } else {
                    let right = ASTNode::String(s);
                    first_node = Some(parse_operator(&left, &operator, &right));
                }
            }
            ASTNode::Bool(b) => {
                if !first {
                    first = true;
                    left = ASTNode::Bool(b);
                } else {
                    let right = ASTNode::Bool(b);
                    first_node = Some(parse_operator(&left, &operator, &right));
                }
            }
            ASTNode::Char(c) => {
                if !first {
                    first = true;
                    left = ASTNode::Char(c);
                } else {
                    let right = ASTNode::Char(c);
                    first_node = Some(parse_operator(&left, &operator, &right));
                }
            }

            ASTNode::RightParenthesis => {
                if parenthesis {
                    break;
                }
            }
            ASTNode::LeftParenthesis => {
                parenthesis = true;
            }
            _ => {
                println!("Syntax Error: Unhandled node in operation: {:?}", top);
                return ASTNode::Int(IntNode { value: 0 });
            }
        }
    }

    if first_node.is_none() {
        return left;
    } else {
        return first_node.unwrap();
    }
}

pub fn parse_operator(left: &ASTNode, operator: &ASTNode, right: &ASTNode) -> ASTNode {
    match operator {
        ASTNode::Operator(o) => match o.operator.as_str() {
            "+" => {
                if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                    let result = left_val.value + right_val.value;
                    let result = IntNode { value: result };
                    return ASTNode::Int(result);
                }
            }
            "=" => {
                if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                    let result = right_val.value;
                    let result = IntNode { value: result };
                    return ASTNode::Int(result);
                }
            }
            "-" => {
                if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                    let result = left_val.value - right_val.value;
                    let result = IntNode { value: result };
                    return ASTNode::Int(result);
                }
            }
            "*" => {
                if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                    let result = left_val.value * right_val.value;
                    let result = IntNode { value: result };
                    return ASTNode::Int(result);
                }
            }
            "/" => {
                if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                    if right_val.value != 0 {
                        let result = left_val.value / right_val.value;
                        let result = IntNode { value: result };
                        return ASTNode::Int(result);
                    } else {
                        println!("Syntax Error: Division by zero.");
                        std::process::exit(1);
                    }
                }
            }
            "==" => {
                if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                    let result = left_val.value == right_val.value;
                    let result = IntNode {
                        value: result as i32,
                    };
                    return ASTNode::Int(result);
                }
            }
            "!=" => {
                if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                    let result = left_val.value != right_val.value;
                    let result = IntNode {
                        value: result as i32,
                    };
                    return ASTNode::Int(result);
                }
            }
            ">" => {
                if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                    let result = left_val.value > right_val.value;
                    let result = IntNode {
                        value: result as i32,
                    };
                    return ASTNode::Int(result);
                }
            }
            "<" => {
                if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                    let result = left_val.value < right_val.value;
                    let result = IntNode {
                        value: result as i32,
                    };
                    return ASTNode::Int(result);
                }
            }
            ">=" => {
                if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                    let result = left_val.value >= right_val.value;
                    let result = IntNode {
                        value: result as i32,
                    };
                    return ASTNode::Int(result);
                }
            }
            "<=" => {
                if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                    let result = left_val.value <= right_val.value;
                    let result = IntNode {
                        value: result as i32,
                    };
                    return ASTNode::Int(result);
                }
            }
            "&&" => {
                if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                    let result = left_val.value != 0 && right_val.value != 0;
                    let result = IntNode {
                        value: result as i32,
                    };
                    return ASTNode::Int(result);
                }
            }
            "||" => {
                if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                    let result = left_val.value != 0 || right_val.value != 0;
                    let result = IntNode {
                        value: result as i32,
                    };
                    return ASTNode::Int(result);
                }
            }
            "!" => {
                if let ASTNode::Int(left_val) = left {
                    let result = left_val.value == 0;
                    let result = IntNode {
                        value: result as i32,
                    };
                    return ASTNode::Int(result);
                }
            }
            "==" => {
                if let (ASTNode::Float(left_val), ASTNode::Float(right_val)) = (left, right) {
                    let result = left_val.value == right_val.value;
                    let result = IntNode {
                        value: result as i32,
                    };
                    return ASTNode::Int(result);
                }
            }
            "!=" => {
                if let (ASTNode::Float(left_val), ASTNode::Float(right_val)) = (left, right) {
                    let result = left_val.value != right_val.value;
                    let result = IntNode {
                        value: result as i32,
                    };
                    return ASTNode::Int(result);
                }
            }

            _ => {
                println!("Syntax Error: Unrecognized operator '{}'", o.operator);
                std::process::exit(1);
            }
        },
        _ => {
            println!("Syntax Error: Expected an operator.");
            std::process::exit(1);
        }
    }
    ASTNode::None
}

fn parse_numeric_expression(exp_stack: &mut Vec<ASTNode>) -> BaseTypes {
    let mut evaluate_empty = true;
    let mut evaluate: (ASTNode, ASTNode, ASTNode) = (ASTNode::None, ASTNode::None, ASTNode::None);
    let mut operator = String::new(); // Use a String to store the operator instead of a reference
    let mut first_iter: bool = true;
    let mut result: i32 = 0;

    while let Some(node) = exp_stack.pop() {
        if first_iter {
            first_iter = false;
            continue;
        } else {
            match node {
                ASTNode::Operator(ref o) => {
                    // Borrow the operator to avoid moving `o`
                    operator = o.operator.clone(); // Clone the operator into the longer-living `String`
                    evaluate.1 = node.clone(); // Clone `node` to avoid partial move issues
                }
                ASTNode::Int(ref n) => {
                    // Borrow `n` to avoid moving it
                    if evaluate_empty {
                        evaluate.0 = node.clone(); // Clone the node to avoid moving
                        evaluate_empty = false;
                    } else {
                        evaluate.2 = node.clone(); // Clone the node to avoid moving
                    }
                }
                ASTNode::SemiColon => {
                    match operator.as_str() {
                        "+" => {
                            result = match &evaluate.0 {
                                ASTNode::Int(n) => n.value as i32,
                                _ => 0,
                            } + match &evaluate.2 {
                                ASTNode::Int(n) => n.value as i32,
                                _ => 0,
                            };
                        }
                        "-" => {
                            result = match &evaluate.0 {
                                ASTNode::Int(n) => n.value as i32,
                                _ => 0,
                            } - match &evaluate.2 {
                                ASTNode::Int(n) => n.value as i32,
                                _ => 0,
                            };
                        }
                        "*" => {
                            result = match &evaluate.0 {
                                ASTNode::Int(n) => n.value as i32,
                                _ => 0,
                            } * match &evaluate.2 {
                                ASTNode::Int(n) => n.value as i32,
                                _ => 0,
                            };
                        }
                        "/" => {
                            result = match &evaluate.0 {
                                ASTNode::Int(n) => n.value as i32,
                                _ => 0,
                            } / match &evaluate.2 {
                                ASTNode::Int(n) => n.value as i32,
                                _ => 0,
                            };
                        }
                        _ => {
                            println!("Syntax Error: Unrecognized operator '{}'", operator);
                            std::process::exit(1);
                        }
                    }
                    return BaseTypes::Int(result);
                }
                _ => {}
            }
        }
    }

    BaseTypes::Null
}

fn process_value_expression(exp_stack: &mut Vec<ASTNode>) -> BaseTypes {
    let mut char_buffer = String::new();

    while let Some(node) = exp_stack.pop() {
        match node {
            ASTNode::Int(n) => {
                return BaseTypes::Int(n.value as i32);
            }
            ASTNode::Float(f) => {
                return BaseTypes::Float(f.value.into());
            }
            ASTNode::String(s) => {
                return BaseTypes::StringWrapper(s.value.clone());
            }
            ASTNode::Bool(b) => {
                return BaseTypes::Bool(b.value);
            }
            ASTNode::Char(c) => {
                char_buffer.push(c.value);

                if char_buffer == "true" {
                    return BaseTypes::Bool(true);
                } else if char_buffer == "false" {
                    return BaseTypes::Bool(false);
                } else {
                    return BaseTypes::Char(c.value);
                }
            }
            _ => {
                println!("Unhandled node in value expression: {:?}", node);
            }
        }
    }

    BaseTypes::Null
}
