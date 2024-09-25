pub mod ast {
    use crate::node::nodes::ASTNode;

    #[derive(Debug, Clone)]
    pub struct AST {
        pub root: ASTNode,
    }

    pub fn display_node(node: &ASTNode) {
        display_node(node);
    }

    impl AST {
        pub fn new(root: ASTNode) -> Self {
            AST { root }
        }

        pub fn add_child(&mut self, node: ASTNode) {
            self.root = node;
        }

        pub fn display(&self) {
            self.display_node(&self.root, 0);
        }

        pub fn display_node(&self, node: &ASTNode, depth: usize) {
            let indent = " ".repeat(depth * 4);
            match node {
                ASTNode::Float(f) => println!("{}FloatNode: Value: {}", indent, f.value),
                ASTNode::SemiColon => println!("{}SemiColonNode", indent),
                ASTNode::Bool(b) => println!("{}BoolNode: Value: {}", indent, b.value),
                ASTNode::Variable(v) => println!(
                    "{}VariableNode: Type: {}, Value: {}",
                    indent, v.var_type, v.value
                ),
                ASTNode::Int(n) => println!("{}IntNode: Value: {}", indent, n.value),
                ASTNode::Operator(o) => {
                    println!("{}OperatorNode: Operator: {}", indent, o.operator)
                }
                ASTNode::Function(f) => println!("{}FunctionNode: Name: {}", indent, f.name),
                ASTNode::String(s) => println!("{}StringNode: Value: {}", indent, s.value),
                ASTNode::Char(c) => println!("{}CharNode: Value: {}", indent, c.value),
                ASTNode::Assignment(a) => println!("{}AssignmentNode: Value: {}", indent, a.value),
                ASTNode::VarTypeAssignment(v) => {
                    println!("{}VarTypeAssignmentNode: Value: {}", indent, v.value)
                }
                ASTNode::FunctionCall(f) => {
                    println!("{}FunctionCallNode: Value: {}", indent, f.name)
                }
                ASTNode::VariableCall(v) => {
                    println!("{}VariableCallNode: Value: {}", indent, v.name)
                }
                ASTNode::VariableType(v) => {
                    println!("{}VariableTypeNode: Value: {}", indent, v.value)
                }
                ASTNode::VariableValue(v) => {
                    println!("{}VariableValueNode: Value: {}", indent, v.value)
                }
                ASTNode::FunctionArguments(f) => {
                    println!("{}FunctionArgumentsNode: Value: {}", indent, f.value)
                }
                ASTNode::AssignmentOperator(a) => {
                    println!("{}AssignmentOperatorNode: Value: {}", indent, a.operator)
                }
                ASTNode::ReturnTypeAssignment(r) => {
                    println!("{}ReturnTypeAssignmentNode: Value: {}", indent, r.value)
                }
                ASTNode::FunctionCallArguments(f) => {
                    println!("{}FunctionCallArgumentsNode: Value: {}", indent, f.value)
                }
                ASTNode::Comment(c) => println!("{}CommentNode: Value: {}", indent, c.value),
                ASTNode::LeftParenthesis => println!("{}LeftParenthesisNode", indent),
                ASTNode::RightParenthesis => println!("{}RightParenthesisNode", indent),
                ASTNode::ArgumentSeparator => println!("{}ArgumentSeparatorNode", indent),
                ASTNode::LeftCurly => println!("{}LeftCurlyNode", indent),
                ASTNode::RightCurly => println!("{}RightCurlyNode", indent),
                ASTNode::None => println!("{}NoneNode", indent),
            }
        }
    }
}
