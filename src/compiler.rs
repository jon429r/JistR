/*
* This file takes in a vector of AST nodes and routes them to be compiled by Rust functions in the
* /compilers directory.
*/
pub mod compilers {
    use crate::compilers::collection::*;
    use crate::compilers::conditional::conditional_compilers::compile_if_elif_else_statement;
    use crate::compilers::function::*;
    use crate::compilers::loops::loop_compilers::{compile_for_loop, compile_while_loop};
    use crate::globals::{IF_ELSE_SKIP, MAKE_LOOP};

    use crate::compilers::variable::parse_variable_call;
    use crate::compilers::variable::{compile_variable_call, parse_variable_declaration};
    use crate::node::nodes::{ASTNode, IntNode, OperatorNode};
    use crate::token_type::token_types::*;
    use std::process::exit;

    pub struct Parser {
        pub tokens: Vec<TokenTypes>,
        pub current: usize,
    }

    impl Parser {
        pub fn new(tokens: Vec<TokenTypes>) -> Self {
            Parser { tokens, current: 0 }
        }

        fn current_token(&self) -> &TokenTypes {
            &self.tokens[self.current]
        }

        fn next_token(&mut self) {
            if self.current < self.tokens.len() - 1 {
                self.current += 1;
            }
        }

        pub fn parse_expression(&mut self) -> ASTNode {
            ASTNode::None
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
                            exit(1);
                        }
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
                "!" => {
                    if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                        let result = !(left_val.value != 0 && right_val.value != 0);
                        let result = IntNode {
                            value: result as i32,
                        };
                        return ASTNode::Int(result);
                    }
                }
                "++" => {
                    if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                        let result = left_val.value + 1;
                        let result = IntNode { value: result };
                        return ASTNode::Int(result);
                    }
                }
                "--" => {
                    if let (ASTNode::Int(left_val), ASTNode::Int(right_val)) = (left, right) {
                        let result = left_val.value - 1;
                        let result = IntNode { value: result };
                        return ASTNode::Int(result);
                    }
                }
                _ => {
                    println!("Syntax Error: Unrecognized operator '{}'", o.operator);
                    exit(1);
                }
            },
            _ => {
                println!("Syntax Error: Expected an operator.");
                exit(1);
            }
        }
        ASTNode::None
    }

    pub fn operation(expression: &mut Vec<ASTNode>) -> ASTNode {
        //let first: Option<ASTNode> = first_value;
        let mut operator: ASTNode = ASTNode::Operator(OperatorNode {
            operator: "+".to_string(),
        });
        let mut right: ASTNode = ASTNode::Int(IntNode { value: 0 });
        let mut left: ASTNode = ASTNode::Int(IntNode { value: 0 });
        let mut first_found = false;

        //let skip_by: usize = if has_parenthisis { 2 } else { 1 };
        expression.reverse();

        //println!("Expression: {:?}", expression);

        for next_node in expression {
            match next_node {
                ASTNode::LeftParenthesis => {}
                ASTNode::RightParenthesis => {}
                ASTNode::Operator(o) => {
                    operator = ASTNode::Operator(o.clone());
                }
                ASTNode::Int(n2) => {
                    if !first_found {
                        left = ASTNode::Int(n2.clone());
                        first_found = true;
                    } else {
                        right = ASTNode::Int(n2.clone());
                        break;
                    }
                }
                _ => {
                    println!(
                        "Syntax Error: Expected operator or number, found {:?}",
                        next_node
                    );
                    exit(1);
                }
            }
        }

        let result = parse_operator(&left, &operator, &right);
        //println!("Parsed expression result: {:?}", result);
        return result;
    }

    pub fn route_to_parser(expression: &mut Vec<ASTNode>, index: Option<usize>) -> bool {
        let mut index = index.unwrap_or(0); // Default to 0 if None

        // Main loop through the expression
        while index < expression.len() {
            let node = &expression[index]; // Access node by index
            let next_node = expression.get(index + 1);
            println!("Node: {:?}", node);

            match node {
                ASTNode::LeftCurly => {
                    println!("Parsing LeftCurlyNode");
                }
                ASTNode::If(_i) => {
                    let result = compile_if_elif_else_statement(expression);
                    if result {
                        index += 2; // Skip to the next statement after processing `if`
                        unsafe { IF_ELSE_SKIP = true };
                        continue;
                    } else {
                        return true;
                    }
                }
                ASTNode::Elif(_i) => {
                    let result = compile_if_elif_else_statement(expression);
                    if result {
                        index += 2; // Skip to the next statement after processing `elif`
                        unsafe { IF_ELSE_SKIP = true };
                        continue;
                    } else {
                        return true;
                    }
                }
                ASTNode::For(_f) => {
                    let result = compile_for_loop(expression);
                    unsafe { MAKE_LOOP = result };
                }
                ASTNode::While(w) => {
                    // Evaluate the condition
                    let condition_result = compile_while_loop(expression);
                    println!("Condition Result: {}\n", condition_result);
                    return true;
                }
                ASTNode::Try => {
                    println!("Parsing TryNode");
                }
                ASTNode::Collection(_c) => {
                    let _value = parse_collection_declaration(expression);
                    return true;
                }
                ASTNode::Variable(_v) => {
                    let end = parse_variable_declaration(expression);
                    if end {
                        return true;
                    }
                }
                ASTNode::Else => {
                    println!("Parsing ElseNode");
                }
                ASTNode::Int(n) => {
                    if expression.len() == 1 {
                        println!("Result: {:?}", ASTNode::Int(n.clone()));
                        break;
                    } else {
                        let result = operation(expression);
                        println!("Result: {:?}", result);
                        break;
                    }
                }
                ASTNode::Function(_f) => {
                    let end = parse_function_declaration(expression);
                    if end {
                        return true;
                    }
                }
                ASTNode::String(s) => {
                    println!("String: {}", s.value);
                }
                ASTNode::Char(c) => {
                    println!("Char: {}", c.value);
                }
                ASTNode::FunctionCall(_f) => {
                    let function_expression: Vec<ASTNode> = expression[index..].to_vec();
                    let _end = parse_function_call(&function_expression);
                    return true;
                }
                ASTNode::VariableCall(_v) => {
                    let call_result = compile_variable_call(expression);
                    if call_result {
                        return true;
                    }
                }
                ASTNode::Comment(_c) => {
                    return true;
                }
                ASTNode::LeftParenthesis => {
                    let value = operation(expression);
                    println!("Result: {:?}", value);
                    break;
                }
                ASTNode::None => {
                    println!("Syntax Error: Unhandled node type.");
                    std::process::exit(1);
                }
                ASTNode::RightParenthesis => {}
                _ => {
                    println!("Syntax Error: Unhandled node: {:?}", node);
                }
            }

            index += 1; // Move to the next node
        }

        true // Return true when done processing
    }
}

#[cfg(test)]
mod complier_tests {
    use crate::compiler::compilers::{operation, parse_operator};
    use crate::node::nodes::{ASTNode, IntNode, OperatorNode};
    //test parse operator
    #[test]
    fn test_parse_operator_addition() {
        let left = ASTNode::Int(IntNode { value: 5 });
        let operator = ASTNode::Operator(OperatorNode {
            operator: "+".to_string(),
        });
        let right = ASTNode::Int(IntNode { value: 5 });
        let result = parse_operator(&left, &operator, &right);
        match result {
            ASTNode::Int(n) => {
                assert_eq!(n.value, 10);
            }
            _ => {
                panic!("Result is not an IntNode");
            }
        }
    }

    #[test]
    fn test_parse_operator_subtraction() {
        let left = ASTNode::Int(IntNode { value: 5 });
        let operator = ASTNode::Operator(OperatorNode {
            operator: "-".to_string(),
        });
        let right = ASTNode::Int(IntNode { value: 5 });
        let result = parse_operator(&left, &operator, &right);
        match result {
            ASTNode::Int(n) => {
                assert_eq!(n.value, 0);
            }
            _ => {
                panic!("Result is not an IntNode");
            }
        }
    }

    #[test]
    fn test_parse_operator_multiplication() {
        let left = ASTNode::Int(IntNode { value: 5 });
        let operator = ASTNode::Operator(OperatorNode {
            operator: "*".to_string(),
        });
        let right = ASTNode::Int(IntNode { value: 5 });
        let result = parse_operator(&left, &operator, &right);
        match result {
            ASTNode::Int(n) => {
                assert_eq!(n.value, 25);
            }
            _ => {
                panic!("Result is not an IntNode");
            }
        }
    }

    #[test]
    fn test_parse_operator_divition() {
        let left = ASTNode::Int(IntNode { value: 5 });
        let operator = ASTNode::Operator(OperatorNode {
            operator: "/".to_string(),
        });
        let right = ASTNode::Int(IntNode { value: 5 });
        let result = parse_operator(&left, &operator, &right);
        match result {
            ASTNode::Int(n) => {
                assert_eq!(n.value, 1);
            }
            _ => {
                panic!("Result is not an IntNode");
            }
        }
    }

    /*
    #[test]
    fn test_operator_unrecognized() {
        let left = ASTNode::Int(IntNode { value: 5 });
        let operator = ASTNode::Operator(OperatorNode {
            operator: "%".to_string(),
        });
        let right = ASTNode::Int(IntNode { value: 5 });
        let result = parse_operator(&left, &operator, &right);
        match result {
            ASTNode::None => {}
            _ => {
                panic!("Result is not None");
            }
        }
    }
    */

    #[test]
    fn test_operation_addition() {
        let mut expression: Vec<ASTNode> = vec![
            ASTNode::Int(IntNode { value: 5 }),
            ASTNode::Operator(OperatorNode {
                operator: "+".to_string(),
            }),
            ASTNode::Int(IntNode { value: 5 }),
        ];
        let result = operation(&mut expression);
        match result {
            ASTNode::Int(n) => {
                assert_eq!(n.value, 10);
            }
            _ => {
                panic!("Result is not an IntNode");
            }
        }
    }

    #[test]
    fn test_operation_subtraction() {
        let mut expression: Vec<ASTNode> = vec![
            ASTNode::Int(IntNode { value: 5 }),
            ASTNode::Operator(OperatorNode {
                operator: "-".to_string(),
            }),
            ASTNode::Int(IntNode { value: 5 }),
        ];
        let result = operation(&mut expression);
        match result {
            ASTNode::Int(n) => {
                assert_eq!(n.value, 0);
            }
            _ => {
                panic!("Result is not an IntNode");
            }
        }

        #[test]
        fn test_operation_multiplication() {
            let mut expression: Vec<ASTNode> = vec![
                ASTNode::Int(IntNode { value: 5 }),
                ASTNode::Operator(OperatorNode {
                    operator: "*".to_string(),
                }),
                ASTNode::Int(IntNode { value: 5 }),
            ];
            let result = operation(&mut expression);
            match result {
                ASTNode::Int(n) => {
                    assert_eq!(n.value, 25);
                }
                _ => {
                    panic!("Result is not an IntNode");
                }
            }
        }
    }

    #[test]
    fn test_operation_divition() {
        let mut expression: Vec<ASTNode> = vec![
            ASTNode::Int(IntNode { value: 5 }),
            ASTNode::Operator(OperatorNode {
                operator: "/".to_string(),
            }),
            ASTNode::Int(IntNode { value: 5 }),
        ];
        let result = operation(&mut expression);
        match result {
            ASTNode::Int(n) => {
                assert_eq!(n.value, 1);
            }
            _ => {
                panic!("Result is not an IntNode");
            }
        }
    }
}
