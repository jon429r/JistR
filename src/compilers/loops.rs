pub mod loop_compilers {
    use crate::compiler::compilers::route_to_parser;
    use crate::compilers::conditional::conditional_compilers::compile_conditional_statement;
    use crate::node::nodes::match_token_to_node;
    use crate::node::nodes::ASTNode;
    use crate::statement_tokenizer::tokenizer::tokenizers::tokenize;
    use crate::statement_tokenizer::tokenizer::tokenizers::ParseInfo;

    static mut MAKE_LOOP: bool = false;

    fn set_make_loop(value: bool) {
        unsafe {
            MAKE_LOOP = value;
        }
    }

    pub fn compile_for_loop(expression: &Vec<ASTNode>) -> bool {
        // check the condition and run
        // for i in 0..10 {}
        // set i to 0 iterate until 10
        // condtion is i < 10
        // for i in 4..10 {}
        // set i to 4 iterate until 10
        // i is var call, may need to be declared, in token range: { range }
        //

        let mut tokenized: Vec<ParseInfo> = Vec::new();
        let mut index = 0;

        while index < expression.len() {
            let node = &expression[index];
            match node {
                ASTNode::For(ifnode) => {
                    let tokenized_statement = tokenize(ifnode.condition.clone());

                    tokenized.extend(tokenized_statement.clone());
                    let mut nodes: Vec<ASTNode> = Vec::new();
                    // convert to ast nodes
                    for token in tokenized_statement {
                        nodes.push(match_token_to_node(token));
                    }

                    // call the operation function or make custom function for conditional operations
                    let result = compile_conditional_statement(&mut nodes);
                    return result;
                }
                ASTNode::Else => {}
                _ => {}
            }
            index += 1;
        }
        return true;
    }

    pub fn compile_while_loop(expression: &mut Vec<ASTNode>) -> bool {
        let mut tokenized: Vec<ParseInfo> = Vec::new();
        let mut index = 0;

        while index < expression.len() {
            let node = &expression[index];
            match node {
                ASTNode::While(while_node) => {
                    //println!(while_node.condition);

                    // Tokenize and evaluate the condition
                    let tokenized_statement = tokenize(while_node.condition.clone());
                    tokenized.extend(tokenized_statement.clone());

                    // Convert tokens to AST nodes
                    let mut condition_nodes: Vec<ASTNode> = tokenized_statement
                        .into_iter()
                        .map(match_token_to_node)
                        .collect();

                    // Evaluate the initial condition
                    let mut result = compile_conditional_statement(&mut condition_nodes);
                    //println!("Initial condition evaluation result: {}", result);

                    while result {
                        set_make_loop(true);

                        //println!("Entering while loop body");

                        // Process the body of the while loop
                        let mut body_index = index + 1; // Start after the while node
                        while body_index < expression.len() {
                            let body_node = &expression[body_index];

                            //println!("Processing body node: {:?}", body_node);

                            // Handle each body node
                            let body_result = route_to_parser(expression, Some(body_index));
                            if !body_result {
                                //println!("Parsing failed for body node. Exiting loop.");
                                return false; // Exit the loop if parsing stops
                            }

                            body_index += 1; // Move to the next body node
                        }

                        // Re-evaluate the while loop condition after each iteration
                        result = compile_conditional_statement(&mut condition_nodes);
                        //println!("Condition re-evaluation result: {}", result);

                        if !result {
                            //println!("Condition is false. Exiting while loop.");
                            return false; // Exit the loop if the condition is false
                        }
                    }

                    set_make_loop(false);

                    // Increment index to move to the next node after the while
                    index += 1;
                    //println!("Moving to next node after while loop.");
                    continue; // Skip to the next iteration
                }
                ASTNode::Else => {
                    //println!("Else node detected.");
                    // Handle else statements if needed
                }
                _ => {
                    println!("Unhandled node: {:?}", node);
                }
            }
            index += 1; // Move to the next node
        }
        //println!("While loop processing completed.");
        true // Indicate successful processing
    }
}
