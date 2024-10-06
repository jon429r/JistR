pub mod loop_compilers {
    use crate::base_variable::base_types::BaseTypes;
    use crate::compiler::compilers::parse_operator;
    use crate::compilers::conditional::conditional_compilers::compile_conditional_statement;
    use crate::compilers::function::parse_function_call;
    use crate::compilers::variable::parse_variable_call;
    use crate::node::nodes::from_base_type;
    use crate::node::nodes::match_token_to_node;
    use crate::node::nodes::to_base_type;
    use crate::node::nodes::ASTNode;
    use crate::statement_tokenizer::tokenizer::tokenizers::tokenize;
    use crate::statement_tokenizer::tokenizer::tokenizers::ParseInfo;
    use crate::token_type::token_types::TokenTypes;

    pub fn compile_for_loop(expression: &Vec<ASTNode>) -> bool {
        // check the condition and run
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

    pub fn compile_while_loop(expression: &Vec<ASTNode>) -> bool {
        //check the condition and run
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
