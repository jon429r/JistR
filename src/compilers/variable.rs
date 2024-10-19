use std::process::exit;

use crate::base_variable::base_types::BaseTypes;
use crate::base_variable::variable::Variable;
use crate::base_variable::variables::VARIABLE_STACK;
use crate::collection::collections::{Array, Dictionary};
use crate::compilers::collection::parse_collection_call;
use crate::compilers::function::parse_function_call;
use crate::node::nodes::match_token_to_node;
use crate::node::nodes::ASTNode;
use crate::node::nodes::VariableCallNode;
use crate::node::nodes::{IntNode, OperatorNode};
use crate::statement_tokenizer::tokenizer::tokenizers::tokenize;

use crate::collection::{ARRAY_STACK, DICTIONARY_STACK};
use crate::statement_tokenizer::variable_tokenizer::variable_tokenizers::read_variable_call;

pub fn search_for_dict_name(name: String) -> bool {
    // if name is in DICTIONARY_STACK reutrn true
    let dict_stack = DICTIONARY_STACK.lock().unwrap(); // Lock the mutex
    for dict in dict_stack.iter() {
        if dict.name == name {
            return true;
        }
    }
    return false;
}

pub fn search_for_array_name(name: String) -> bool {
    let array_stack = ARRAY_STACK.lock().unwrap();
    for array in array_stack.iter() {
        if array.name == name {
            return true;
        }
    }
    return false;
}

/*
pub fn search_for_var_name(name: String) -> bool {
    let var_stack = unsafe { VARIABLE_STACK };
    for var in var_stack.inter() {
        if var.name == name {
            return true;
        }
    }
    return False;
} */

pub fn get_dict(name: String) -> Dictionary {
    let dict_stack = DICTIONARY_STACK.lock().unwrap();
    for dict in dict_stack.iter() {
        if dict.name == name {
            return dict.clone();
        }
    }
    println!("Dictionary not found");
    exit(1);
}

pub fn get_array(name: String) -> Array {
    let array_stack = ARRAY_STACK.lock().unwrap();
    for array in array_stack.iter() {
        if array.name == name {
            return array.clone();
        }
    }
    println!("Array not found");
    exit(1);
}

pub fn parse_object_call(node: &ASTNode) -> (String, String) {
    // will look at object call name and see if it is in variable stack or collection stack then
    // return the approiate ast node, eg variable call or collection call

    match node.clone() {
        ASTNode::Dot(c) => {
            let object_name = c.object.clone();
            let dict: bool = search_for_dict_name(object_name.clone());
            let array: bool = search_for_array_name(object_name.clone());
            let variable: bool = false;

            if dict {
                return (object_name, "dictionary".to_string());
            }
            if array {
                return (object_name, "array".to_string());
            }
            if variable {
                return (object_name, "variable".to_string());
            } else {
                println!("Object call is not a variable, dictionary or array");
                exit(1);
            }
        }
        _ => {
            println!(
                "Error expected an object call node in parse object call found {:?}",
                node
            );
        }
    }

    exit(1);
}

pub fn compile_dot_statement(exp_stack: &mut Vec<ASTNode>) -> bool {
    println!("compiling dot statement");

    // in order to compile we must, tokenize variable or collection call, tokenize function call
    // check what type object is
    // then make an object type specific function call

    let node: ASTNode = exp_stack.get(0).unwrap().clone();

    match node.clone() {
        ASTNode::Dot(d) => {
            println!("Dot call\nobject: {}, function: {}", d.object, d.function);
            let mut variable: (String, BaseTypes);
            let mut collection: (String, Vec<BaseTypes>);
            let mut object_name_type: (String, String);

            // first tokenize object
            let objects = tokenize(d.object);
            let mut object_nodes: Vec<ASTNode> = Vec::new();
            //now match to node
            for object in objects.clone() {
                object_nodes.push(match_token_to_node(object));
            }

            // check type of object, var, collection? then comiple
            match object_nodes.get(0).unwrap() {
                ASTNode::ObjectCall(c) => {
                    object_name_type = parse_object_call(&node);

                    //determine object type
                    //get that from stack, then call approiate function

                    match object_name_type.1.to_string().as_str() {
                        "dictionary" => {
                            println!("Object call is a dictionary");
                            let dict: Option<Dictionary> = get_dict(object_name_type.0).into();
                            let tokenized_function = tokenize(d.function.clone());
                            let mut function_nodes: Vec<ASTNode> = Vec::new();
                            for function in tokenized_function {
                                function_nodes.push(match_token_to_node(function));
                            }
                            println!("Function nodes: {:?}", function_nodes);
                            parse_function_call(
                                &function_nodes,
                                "dictionary".to_string(),
                                None,
                                dict,
                                None,
                            );
                        }
                        "array" => {
                            println!("Object call is an array");
                            let array: Option<Array> = get_array(object_name_type.0).into();
                            let tokenized_function = tokenize(d.function.clone());
                            let mut function_nodes: Vec<ASTNode> = Vec::new();
                            for function in tokenized_function {
                                function_nodes.push(match_token_to_node(function));
                            }
                            println!("Function nodes: {:?}", function_nodes);

                            //make sure array is not None

                            match array.clone() {
                                Some(Array {
                                    name,
                                    data,
                                    value_type,
                                }) => {
                                    println!(
                                        "Array is not None with name: {}, and data type: {}",
                                        name, value_type
                                    );
                                    // You can use name, data, value_type as needed
                                }
                                None => {
                                    println!("Array is None");
                                    exit(1);
                                }
                            }

                            parse_function_call(
                                &function_nodes,
                                "array".to_string(),
                                array,
                                None,
                                None,
                            );
                        }
                        "variable" => {
                            println!("Object call is a variable");
                            let tokenized_function = tokenize(d.function.clone());
                            let mut function_nodes: Vec<ASTNode> = Vec::new();
                            for function in tokenized_function {
                                function_nodes.push(match_token_to_node(function));
                            }
                            println!("Function nodes: {:?}", function_nodes);
                        }
                        _ => {
                            println!("Object call is not a variable, dictionary or array");
                            exit(1);
                        }
                    }
                }
                ASTNode::Collection(c) => collection = parse_collection_call(&object_nodes),
                ASTNode::VariableCall(c) => {
                    variable = parse_variable_call(object_nodes.get(0).unwrap())
                }
                _ => {
                    println!("Unexpected node found within comple dot statement function");
                    exit(1);
                }
            }
        }
        _ => {
            println!("Syntax Error: Expected a dot statement found {}", node);
            exit(1);
        }
    }

    return true;
}

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
            ASTNode::FunctionCall(_c) => {
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

                    let result = parse_function_call(
                        &function_call_stack,
                        "None".to_string(),
                        None,
                        None,
                        None,
                    );
                    // TODO set value to result
                    value = result;

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
            ASTNode::VariableCall(_c) => {
                if inside_assignment {
                    let result = parse_variable_call(node);
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
            ASTNode::Int(_n) => {
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
