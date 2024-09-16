pub mod operation {
    use crate::node::node::ASTNode;

    pub fn parse_operation(expression: &Vec<ASTNode>) {
        println!("Parsing operation.");
        let mut operation: Option<String> = None;
        let mut left_operand: Option<String> = None;
        let mut right_operand: Option<String> = None;

        for node in expression {
            match node {
                ASTNode::Operator(o) => {
                    operation = Some(o.operator.clone());
                    //println!("Operator: {}", o.operator);
                }
                ASTNode::Int(n) => {
                    if left_operand.is_none() {
                        left_operand = Some(n.value.clone().to_string());
                        //println!("Left operand: {}", n.value);
                    } else {
                        right_operand = Some(n.value.clone().to_string());
                        //println!("Right operand: {}", n.value);
                    }
                }
                _ => {
                    println!("Syntax Error: Unhandled node in operation: {:?}", node);
                }
            }
        }

        if operation.is_none() || left_operand.is_none() || right_operand.is_none() {
            println!("Syntax Error: Operation is incomplete.");
            return;
        }

        //println!("Successfully parsed operation.");
    }
}
