/*
* This file contains the different AST node types used to create the AST tree and functions to go
* with them
*/

pub mod node {
    use crate::base_variables::base_types::BaseTypes;
    use crate::token_types::token_type::*;
    use crate::tokenizer::tokenizer::ParseInfo;

    pub fn to_base_type(node: &ASTNode) -> Option<BaseTypes> {
        match node {
            ASTNode::SemiColon => Some(BaseTypes::Null),
            ASTNode::Operator(_) => Some(BaseTypes::Null),
            ASTNode::Int(int_node) => Some(BaseTypes::Int(int_node.value)),
            ASTNode::String(string_node) => {
                Some(BaseTypes::StringWrapper(string_node.value.clone()))
            }
            ASTNode::Char(char_node) => Some(BaseTypes::Char(char_node.value)),
            ASTNode::Bool(bool_node) => Some(BaseTypes::Bool(bool_node.value)),
            ASTNode::Float(float_node) => Some(BaseTypes::Float(float_node.value as f64)),
            ASTNode::Assignment(_) => Some(BaseTypes::Null),
            ASTNode::VarTypeAssignment(_) => Some(BaseTypes::Null),
            ASTNode::Variable(_) => Some(BaseTypes::Null),
            ASTNode::Function(_) => Some(BaseTypes::Null),
            ASTNode::FunctionCall(_) => Some(BaseTypes::Null),
            ASTNode::VariableCall(_) => Some(BaseTypes::Null),
            ASTNode::VariableType(_) => Some(BaseTypes::Null),
            ASTNode::VariableValue(_) => Some(BaseTypes::Null),
            ASTNode::FunctionArguments(_) => Some(BaseTypes::Null),
            ASTNode::AssignmentOperator(_) => Some(BaseTypes::Null),
            ASTNode::ReturnTypeAssignment(_) => Some(BaseTypes::Null),
            ASTNode::Comment(_) => Some(BaseTypes::Null),
            ASTNode::FunctionCallArguments(_) => Some(BaseTypes::Null),
            ASTNode::LeftParenthesis => Some(BaseTypes::Null),
            ASTNode::RightParenthesis => Some(BaseTypes::Null),
            ASTNode::ArgumentSeparator => Some(BaseTypes::Null),
            ASTNode::LeftCurly => Some(BaseTypes::Null),
            ASTNode::RightCurly => Some(BaseTypes::Null),
            ASTNode::None => Some(BaseTypes::Null),
        }
    }

    #[derive(Debug, Clone)]
    pub enum ASTNode {
        SemiColon,
        Operator(OperatorNode),
        Int(IntNode),
        String(StringNode),
        Char(CharNode),
        Bool(BoolNode),
        Float(FloatNode),
        Assignment(AssignmentNode),
        VarTypeAssignment(VarTypeAssignmentNode),
        Variable(VariableNode),
        Function(FunctionNode),
        FunctionCall(FunctionCallNode),
        VariableCall(VariableCallNode),
        VariableType(VariableTypeNode),
        VariableValue(VariableValueNode),
        FunctionArguments(FunctionArgumentsNode),
        AssignmentOperator(AssignmentOperatorNode),
        ReturnTypeAssignment(ReturnTypeAssignmentNode),
        Comment(CommentNode),
        FunctionCallArguments(FunctionArgumentsNode),
        LeftParenthesis,
        RightParenthesis,
        ArgumentSeparator,
        LeftCurly,
        RightCurly,
        None,
    }

    #[derive(Debug, Clone)]
    pub struct BoolNode {
        pub value: bool,
    }

    impl BoolNode {
        pub fn new(value: bool) -> Self {
            BoolNode { value }
        }
        pub fn display_info(&self) {
            println!("Bool: {}", self.value);
        }
    }

    #[derive(Debug, Clone)]
    pub struct SemiColonNode;

    impl SemiColonNode {
        pub fn new() -> Self {
            SemiColonNode
        }
        pub fn display_info(&self) {
            println!("SemiColon");
        }
    }

    #[derive(Debug, Clone)]
    pub struct OperatorNode {
        pub operator: String,
    }

    impl OperatorNode {
        pub fn new(operator: String) -> Self {
            OperatorNode { operator }
        }
        pub fn display_info(&self) {
            println!("Operator: {}", self.operator);
        }
    }

    #[derive(Debug, Clone)]
    pub struct IntNode {
        pub value: i32,
    }

    impl IntNode {
        pub fn new(value: i32) -> Self {
            IntNode { value }
        }
        pub fn display_info(&self) {
            println!("Int: {}", self.value);
        }
    }

    #[derive(Debug, Clone)]
    pub struct StringNode {
        pub value: String,
    }

    impl StringNode {
        pub fn new(value: String) -> Self {
            StringNode { value }
        }
        pub fn display_info(&self) {
            println!("String: {}", self.value);
        }
    }

    #[derive(Debug, Clone)]
    pub struct CharNode {
        pub value: char,
    }

    impl CharNode {
        pub fn new(value: char) -> Self {
            CharNode { value }
        }
        pub fn display_info(&self) {
            println!("Char: {}", self.value);
        }
    }

    #[derive(Debug, Clone)]
    pub struct AssignmentNode {
        pub value: String,
    }

    impl AssignmentNode {
        pub fn new(value: String) -> Self {
            AssignmentNode { value }
        }
        pub fn display_info(&self) {
            println!("Assignment: {}", self.value);
        }
    }

    #[derive(Debug, Clone)]
    pub struct VarTypeAssignmentNode {
        pub value: String,
    }

    impl VarTypeAssignmentNode {
        pub fn new(value: String) -> Self {
            VarTypeAssignmentNode { value }
        }
        pub fn display_info(&self) {
            println!("VarTypeAssignment: {}", self.value);
        }
    }

    #[derive(Debug, Clone)]
    pub struct VariableNode {
        pub var_type: String,
        pub value: String,
    }

    impl VariableNode {
        pub fn new(var_type: String, value: String) -> Self {
            VariableNode { var_type, value }
        }
        pub fn display_info(&self) {
            println!("Variable Type: {}, Value: {}", self.var_type, self.value);
        }
    }

    #[derive(Debug, Clone)]
    pub struct VariableValueNode {
        pub value: String,
    }

    impl VariableValueNode {
        pub fn new(value: String) -> Self {
            VariableValueNode { value }
        }
        pub fn display_info(&self) {
            println!("Variable Value: {}", self.value);
        }
    }

    #[derive(Debug, Clone)]
    pub struct VariableTypeNode {
        pub value: String,
    }

    impl VariableTypeNode {
        pub fn new(value: String) -> Self {
            VariableTypeNode { value }
        }
        pub fn display_info(&self) {
            println!("Variable Type: {}", self.value);
        }
    }

    #[derive(Debug, Clone)]
    pub struct FunctionNode {
        pub name: String,
    }

    impl FunctionNode {
        pub fn new(name: String) -> Self {
            FunctionNode { name }
        }
        pub fn display_info(&self) {
            println!("Function Name: {}", self.name);
        }
    }

    #[derive(Debug, Clone)]
    pub struct FunctionCallNode {
        pub name: String,
    }

    impl FunctionCallNode {
        pub fn new(name: String) -> Self {
            FunctionCallNode { name }
        }
        pub fn display_info(&self) {
            println!("Function Call Name: {}", self.name);
        }
    }

    #[derive(Debug, Clone)]
    pub struct FunctionArgumentsNode {
        pub value: String,
    }

    impl FunctionArgumentsNode {
        pub fn new(value: String) -> Self {
            FunctionArgumentsNode { value }
        }
        pub fn display_info(&self) {
            println!("Function Arguments: {}", self.value);
        }
    }

    #[derive(Debug, Clone)]
    pub struct VariableCallNode {
        pub name: String,
    }

    impl VariableCallNode {
        pub fn new(name: String) -> Self {
            VariableCallNode { name }
        }
        pub fn display_info(&self) {
            println!("Variable Call Name: {}", self.name);
        }
    }

    #[derive(Debug, Clone)]
    pub struct AssignmentOperatorNode {
        pub operator: String,
    }

    impl AssignmentOperatorNode {
        pub fn new(operator: String) -> Self {
            AssignmentOperatorNode { operator }
        }
        pub fn display_info(&self) {
            println!("Assignment Operator: {}", self.operator);
        }
    }

    #[derive(Debug, Clone)]
    pub struct FloatNode {
        pub value: f32,
    }

    impl FloatNode {
        pub fn new(value: f32) -> Self {
            FloatNode { value }
        }
        pub fn display_info(&self) {
            println!("Float: {}", self.value);
        }
    }

    #[derive(Debug, Clone)]
    pub struct ReturnTypeAssignmentNode {
        pub value: String,
    }

    impl ReturnTypeAssignmentNode {
        pub fn new(value: String) -> Self {
            ReturnTypeAssignmentNode { value }
        }
        pub fn display_info(&self) {
            println!("ReturnTypeAssignment: {}", self.value);
        }
    }

    #[derive(Debug, Clone)]
    pub struct CommentNode {
        pub value: String,
    }

    impl CommentNode {
        pub fn new(value: String) -> Self {
            CommentNode { value }
        }
        pub fn display_info(&self) {
            println!("Comment: {}", self.value);
        }
    }

    pub fn match_token_to_node(parse_info: ParseInfo) -> ASTNode {
        match parse_info.token {
            TokenTypes::Int => {
                if let Ok(value) = parse_info.value.parse::<i32>() {
                    ASTNode::Int(IntNode::new(value))
                } else if let Ok(value) = parse_info.value.parse::<i32>() {
                    ASTNode::Int(IntNode::new(value as i32))
                } else {
                    panic!("Failed to parse Int: {}", parse_info.value);
                }
            }
            TokenTypes::String => ASTNode::String(StringNode::new(parse_info.value)),
            TokenTypes::Bool => ASTNode::Bool(BoolNode::new(
                parse_info.value.parse::<bool>().expect("Invalid bool"),
            )),
            TokenTypes::Float => ASTNode::Float(FloatNode::new(
                parse_info.value.parse::<f32>().expect("Invalid float"),
            )),
            TokenTypes::Char => ASTNode::Char(CharNode::new(
                parse_info.value.chars().next().expect("Invalid char"),
            )),
            TokenTypes::Operator => ASTNode::Operator(OperatorNode::new(parse_info.value)),
            TokenTypes::AssignmentOperator => {
                ASTNode::AssignmentOperator(AssignmentOperatorNode::new(parse_info.value))
            }
            TokenTypes::LeftParenthesis => ASTNode::LeftParenthesis,
            TokenTypes::RightParenthesis => ASTNode::RightParenthesis,
            TokenTypes::Function => ASTNode::Function(FunctionNode::new(parse_info.value)),
            TokenTypes::FunctionArguments => {
                ASTNode::FunctionArguments(FunctionArgumentsNode::new(parse_info.value))
            }
            TokenTypes::FunctionCall => {
                ASTNode::FunctionCall(FunctionCallNode::new(parse_info.value))
            }
            TokenTypes::Variable => {
                ASTNode::Variable(VariableNode::new("".to_string(), parse_info.value))
            }
            TokenTypes::VarTypeAssignment => {
                ASTNode::VariableType(VariableTypeNode::new(parse_info.value))
            }
            TokenTypes::VariableCall => {
                ASTNode::VariableCall(VariableCallNode::new(parse_info.value))
            }
            TokenTypes::ArgumentSeparator => ASTNode::ArgumentSeparator,
            TokenTypes::Assignment => ASTNode::Assignment(AssignmentNode::new(parse_info.value)),
            TokenTypes::RightCurly => ASTNode::RightCurly,
            TokenTypes::LeftCurly => ASTNode::LeftCurly,
            TokenTypes::ReturnTypeAssignment => {
                ASTNode::ReturnTypeAssignment(ReturnTypeAssignmentNode::new(parse_info.value))
            }
            TokenTypes::Comment => ASTNode::Comment(CommentNode::new(parse_info.value)),
            TokenTypes::SemiColon => ASTNode::SemiColon,
            TokenTypes::None => ASTNode::None,
            _ => {
                panic!("Unrecognized token: {:?}", parse_info.token);
            }
        }
    }
}
