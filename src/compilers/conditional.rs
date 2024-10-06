pub mod conditional_compilers {
    use crate::base_variable::base_types::BaseTypes;
    use crate::compiler::compilers::parse_operator;
    use crate::compilers::function::parse_function_call;
    use crate::compilers::variable::parse_variable_call;
    use crate::node::nodes::from_base_type;
    use crate::node::nodes::match_token_to_node;
    use crate::node::nodes::to_base_type;
    use crate::node::nodes::ASTNode;
    use crate::statement_tokenizer::tokenizer::tokenizers::tokenize;
    use crate::statement_tokenizer::tokenizer::tokenizers::ParseInfo;
    use crate::token_type::token_types::TokenTypes;

    pub fn compile_conditional_statement(expression: &mut Vec<ASTNode>) -> bool {
        let mut index = 0;
        let mut first_value: ASTNode = ASTNode::None;
        let mut operation: ASTNode = ASTNode::None;
        let mut second_value: ASTNode = ASTNode::None;

        while index < expression.len() {
            let node = &expression[index];
            match node {
                ASTNode::VariableCall(_) => {
                    let (_, value) = parse_variable_call(node);
                    first_value = if first_value == ASTNode::None {
                        from_base_type(value)
                    } else {
                        first_value
                    };
                }

                ASTNode::FunctionCall(_) => {
                    let mut function_call: Vec<ASTNode> = Vec::new();
                    while index < expression.len() {
                        match &expression[index] {
                            ASTNode::RightParenthesis => {
                                function_call.push(expression[index].clone());
                                break;
                            }
                            _ => function_call.push(expression[index].clone()),
                        }
                        index += 1;
                    }

                    if first_value == ASTNode::None {
                        let value = parse_function_call(&function_call);
                        first_value = from_base_type(value);
                    } else {
                        let value = parse_function_call(&function_call);
                        second_value = from_base_type(value);
                    }
                }

                ASTNode::Operator(o) => {
                    operation = ASTNode::Operator(o.clone());
                }

                ASTNode::Int(n) => {
                    if first_value == ASTNode::None {
                        first_value = ASTNode::Int(n.clone());
                    } else {
                        second_value = ASTNode::Int(n.clone());
                    }
                }

                ASTNode::Float(n) => {
                    if first_value == ASTNode::None {
                        first_value = ASTNode::Float(n.clone());
                    } else {
                        second_value = ASTNode::Float(n.clone());
                    }
                }

                ASTNode::String(n) => {
                    if first_value == ASTNode::None {
                        first_value = ASTNode::String(n.clone());
                    } else {
                        second_value = ASTNode::String(n.clone());
                    }
                }

                ASTNode::Char(c) => {
                    if first_value == ASTNode::None {
                        first_value = ASTNode::Char(c.clone());
                    } else {
                        second_value = ASTNode::Char(c.clone());
                    }
                }

                ASTNode::Bool(b) => {
                    if first_value == ASTNode::None {
                        first_value = ASTNode::Bool(b.clone());
                    } else {
                        second_value = ASTNode::Bool(b.clone());
                    }
                }

                _ => {}
            }
            index += 1;
        }

        // Assuming the operation is a comparison operator (e.g., >, <, ==)
        let ast_result = parse_operator(&first_value, &operation, &second_value);

        // Safely convert AST result to BaseTypes, with error handling
        let base_result: BaseTypes = match to_base_type(&ast_result) {
            Some(result) => result,
            None => {
                println!("Error: Unable to convert to BaseTypes");
                return false;
            }
        };

        // Convert BaseTypes result into i32 and return true/false
        let result: i32 = base_result.into();
        result == 1
    }

    pub fn compile_if_elif_else_statement(expression: &mut Vec<ASTNode>) -> bool {
        let mut tokenized: Vec<ParseInfo> = Vec::new();
        let mut index = 0;

        while index < expression.len() {
            let node = &expression[index];
            match node {
                ASTNode::If(ifnode) => {
                    let tokenized_statement = tokenize(ifnode.condition.clone());

                    tokenized.extend(tokenized_statement.clone());
                    let mut nodes: Vec<ASTNode> = Vec::new();
                    // convert to ast nodes
                    for token in tokenized_statement {
                        nodes.push(match_token_to_node(token));
                    }

                    // call the operation function or make custom function for conditional operations
                    let mut result = compile_conditional_statement(&mut nodes);
                    return result;
                }
                ASTNode::Elif(elifnode) => {
                    let tokenized_statement = tokenize(elifnode.condition.clone());
                    tokenized.extend(tokenized_statement.clone());
                    let mut nodes: Vec<ASTNode> = Vec::new();
                    // convert to ast nodes
                    for token in tokenized_statement {
                        nodes.push(match_token_to_node(token));
                    } // call the operation function or make custom function for conditional operations
                    let mut result = compile_conditional_statement(&mut nodes);
                    return result;
                }
                ASTNode::Else => {}
                _ => {}
            }
            index += 1;
        }
        return true;
    }
}
