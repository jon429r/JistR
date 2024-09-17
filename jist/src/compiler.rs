/*
* This file takes in a vector of AST nodes and routes them to be compiled by Rust functions in the
* /compilers directory.
*/
pub mod compiler {
    use crate::compilers::function::parse_function_declaration_or_call;
    use crate::compilers::variable::parse_variable_declaration_or_assignment;
    use crate::node::node::{ASTNode, IntNode, OperatorNode};
    use crate::token_types::token_type::*;
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
            ASTNode::Operator(o) => {
                match o.operator.as_str() {
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
                    _ => {
                        println!("Syntax Error: Unrecognized operator '{}'", o.operator);
                        exit(1);
                    }
                }
            }
            _ => {
                println!("Syntax Error: Expected an operator.");
                exit(1);
            }
        }
        ASTNode::None
    }

    pub fn operation(
        expression: &mut Vec<ASTNode>, 
        first_value: Option<ASTNode>,
        has_parenthisis: bool,
    ) -> ASTNode {
        let first: Option<ASTNode> = first_value;
        let mut operator: ASTNode = ASTNode::Operator(OperatorNode { operator: "+".to_string() });
        let mut right: ASTNode = ASTNode::Int(IntNode { value: 0 });
        let mut left: ASTNode = ASTNode::Int(IntNode { value: 0 });
        let mut first_found = false;

        //let skip_by: usize = if has_parenthisis { 2 } else { 1 };
        expression.reverse();

        println!("Expression: {:?}", expression);

        for next_node in expression {
            match next_node {
                ASTNode::LeftParenthesis => {}
                ASTNode::RightParenthesis => {}
                ASTNode::Operator(o) => {
                    operator = ASTNode::Operator(o.clone());
                }
                ASTNode::Int(n2) => {
                    if first_found == false {
                        left = ASTNode::Int(n2.clone());
                        first_found = true;
                    } else {
                        right = ASTNode::Int(n2.clone());
                        break;
                    }
                }
                _ => {
                    println!("Syntax Error: Expected operator or number.");
                    exit(1);
                }
            }
        }

            let result = parse_operator(&left, &operator, &right);
            println!("Parsed expression result: {:?}", result);
            return result;
       
    }

    pub fn route_to_parser(expression: &mut Vec<ASTNode>) {
        let mut index = 0; // Start with index-based iteration
        while index < expression.len() {
            let node = &expression[index]; // Access node by index
            let next_node = expression.get(index + 1);
    
            match node {
                ASTNode::Variable(v) => {
                    let end = parse_variable_declaration_or_assignment(expression);  // Pass mutable reference
                    if end {
                        return;
                    }
                }
                ASTNode::Int(n) => {
                    let first: Option<ASTNode> = Some(ASTNode::Int(n.clone()));
    
                    // If the expression is just a single number, return it
                    if expression.len() == 1 {
                        println!("Result: {:?}", first);
                        break;
                    } else {
                        let result = operation(expression, first, false);  // Mutable reference
                        println!("Result: {:?}", result);
                        break;
                    }
                }
                ASTNode::Function(f) => {
                    let end = parse_function_declaration_or_call(expression);  // Mutable reference
                    if end {
                        return;
                    }
                }
                ASTNode::String(s) => {
                    println!("String: {}", s.value);
                }
                ASTNode::Char(c) => {
                    println!("Char: {}", c.value);
                }
                ASTNode::FunctionCall(f) => {
                    let end = parse_function_declaration_or_call(expression);  // Mutable reference
                    if end {
                        return;
                    }
                }
                ASTNode::VariableCall(v) => {
                    let end = parse_variable_declaration_or_assignment(expression);  // Mutable reference
                    if end {
                        return;
                    }
                }
                ASTNode::Comment(c) => {
                    return;
                }
                ASTNode::LeftParenthesis => {
                    let first: Option<ASTNode> = next_node.cloned();
    
                    let value = operation(expression, first, true);  // Mutable reference
                    print!("Result: {:?}", value);
                    break;
                }
                ASTNode::LeftCurly => {
                    println!("Parsing LeftCurlyNode");
                }
                ASTNode::None => {
                    println!("Syntax Error: Unhandled node type.");
                    exit(1);
                }
                _ => {
                    println!("Syntax Error: Unhandled node: {:?}", node);
                }
            }
            index += 1; // Move to the next node
        }
    }
    
}
