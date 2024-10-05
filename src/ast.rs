pub mod ast {
    use crate::node::nodes::ASTNode;

    #[derive(Debug, Clone)]
    pub struct AST {
        pub root: ASTNode,
    }

    impl AST {
        pub fn new(root: ASTNode) -> Self {
            AST { root }
        }

        pub fn add_child(&mut self, node: ASTNode) {
            self.root = node;
        }

        /*       pub fn display(&self) {
            self.display_node(&self.root, 0);
        }

                pub fn display_node(&self, node: &ASTNode, depth: usize) {
                    let indent = " ".repeat(depth * 4);
                    match node {
                        ASTNode::FatArrow => println!("{}FatArrowNode", indent),
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
                        ASTNode::Collection(c) => {
                            println!(
                                "{}CollectionNode: Name: {}, Type: {}, Single: {:?}, Tuple: {:?}",
                                indent, c.name, c.collection_type, c.value_type_single, c.value_type_tuple,
                            )
                        }
                        ASTNode::RightBracket => println!("{}RightBracketNode", indent),
                        ASTNode::LeftBracket => println!("{}LeftBracketNode", indent),
                        ASTNode::Comment(c) => println!("{}CommentNode: Value: {}", indent, c.value),
                        ASTNode::LeftParenthesis => println!("{}LeftParenthesisNode", indent),
                        ASTNode::RightParenthesis => println!("{}RightParenthesisNode", indent),
                        ASTNode::ArgumentSeparator => println!("{}ArgumentSeparatorNode", indent),
                        ASTNode::LeftCurly => println!("{}LeftCurlyNode", indent),
                        ASTNode::RightCurly => println!("{}RightCurlyNode", indent),
                        ASTNode::While(w) => println!("{}WhileNode: Value: {}", indent, w.value),
                        ASTNode::If(i) => println!("{}IfNode: Value: {}", indent, i.value),
                        ASTNode::Elif(e) => println!("{}ElifNode: Value: {}", indent, e.value),
                        ASTNode::Else(e) => println!("{}ElseNode: Value: {}", indent),
                        ASTNode::For(f) => println!("{}ForNode: Value: {}", indent, f.value),
                        ASTNode::Try(t) => println!("{}TryNode: Value: {}", indent),
                        ASTNode::Catch(c) => println!("{}CatchNode: Value: {}", indent),
                        ASTNode::Finally(f) => println!("{}FinallyNode: Value: {}", indent),
                        ASTNode::None => println!("{}NoneNode", indent),
                    }
                }
        */

        // Allowing user to conver the AST to the corresponding rust types
    }

    impl From<ASTNode> for char {
        fn from(node: ASTNode) -> char {
            if let ASTNode::Char(c) = node {
                c.value
            } else {
                panic!("Cannot convert non-char ASTNode to char")
            }
        }
    }

    impl From<char> for ASTNode {
        fn from(c: char) -> ASTNode {
            ASTNode::Char(crate::node::nodes::CharNode { value: c })
        }
    }
}
