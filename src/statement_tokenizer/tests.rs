#[cfg(test)]
mod tokenizer_tests {
    use crate::statement_tokenizer::tokenizer::tokenizers;
    use crate::statement_tokenizer::tokenizer::tokenizers::ParseInfo;
    use crate::token_type::token_types::TokenTypes;

    #[test]
    fn test_tokenize_variable_declaration_int() {
        let input = "let a: int = 1;".to_string();

        let expected = vec![
            ParseInfo {
                token: TokenTypes::Variable,
                chars_read: 1,
                value: "a".to_string(),
            },
            ParseInfo {
                token: TokenTypes::VarTypeAssignment,
                chars_read: 10,
                value: "int".to_string(),
            },
            ParseInfo {
                token: TokenTypes::AssignmentOperator,
                chars_read: 1,
                value: "=".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Int,
                chars_read: 1,
                value: "1".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];

        let result = tokenizers::tokenize(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_variable_declaration_float() {
        let input = "let a: float = 1.102;".to_string();
        let expected = vec![
            ParseInfo {
                token: TokenTypes::Variable,
                chars_read: 1,
                value: "a".to_string(),
            },
            ParseInfo {
                token: TokenTypes::VarTypeAssignment,
                chars_read: 12,
                value: "float".to_string(),
            },
            ParseInfo {
                token: TokenTypes::AssignmentOperator,
                chars_read: 1,
                value: "=".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Float,
                chars_read: 5,
                value: "1.102".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];
        let result = tokenizers::tokenize(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_variable_declaration_string() {
        let input = "let a: string = \"Hello, World!\";".to_string();
        let expected = vec![
            ParseInfo {
                token: TokenTypes::Variable,
                chars_read: 1,
                value: "a".to_string(),
            },
            ParseInfo {
                token: TokenTypes::VarTypeAssignment,
                chars_read: 13,
                value: "string".to_string(),
            },
            ParseInfo {
                token: TokenTypes::AssignmentOperator,
                chars_read: 1,
                value: "=".to_string(),
            },
            ParseInfo {
                token: TokenTypes::String,
                chars_read: 15,
                value: "\"Hello, World!\"".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];
        let result = tokenizers::tokenize(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_variable_declaration_char() {
        let input = "let a: char = 'a';".to_string();
        let expected = vec![
            ParseInfo {
                token: TokenTypes::Variable,
                chars_read: 1,
                value: "a".to_string(),
            },
            ParseInfo {
                token: TokenTypes::VarTypeAssignment,
                chars_read: 11,
                value: "char".to_string(),
            },
            ParseInfo {
                token: TokenTypes::AssignmentOperator,
                chars_read: 1,
                value: "=".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Char,
                chars_read: 3,
                value: "'a'".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];
        let result = tokenizers::tokenize(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_variable_declaration_boolean_true() {
        let input = "let a: bool = True;".to_string();
        let expected = vec![
            ParseInfo {
                token: TokenTypes::Variable,
                chars_read: 1,
                value: "a".to_string(),
            },
            ParseInfo {
                token: TokenTypes::VarTypeAssignment,
                chars_read: 11,
                value: "bool".to_string(),
            },
            ParseInfo {
                token: TokenTypes::AssignmentOperator,
                chars_read: 1,
                value: "=".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Bool,
                chars_read: 4,
                value: "True".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];
        let result = tokenizers::tokenize(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_variable_declaration_boolean_false() {
        let input = "let a: bool = False;".to_string();
        let expected = vec![
            ParseInfo {
                token: TokenTypes::Variable,
                chars_read: 1,
                value: "a".to_string(),
            },
            ParseInfo {
                token: TokenTypes::VarTypeAssignment,
                chars_read: 11,
                value: "bool".to_string(),
            },
            ParseInfo {
                token: TokenTypes::AssignmentOperator,
                chars_read: 1,
                value: "=".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Bool,
                chars_read: 5,
                value: "False".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];
        let result = tokenizers::tokenize(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_variable_call_int() {
        let input = "a = 1;".to_string();
        let expected = vec![
            ParseInfo {
                token: TokenTypes::VariableCall,
                chars_read: 1,
                value: "a".to_string(),
            },
            ParseInfo {
                token: TokenTypes::AssignmentOperator,
                chars_read: 1,
                value: "=".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Int,
                chars_read: 1,
                value: "1".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];
        let result = tokenizers::tokenize(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_variable_call_float() {
        let input = "a = 1.102;".to_string();
        let expected = vec![
            ParseInfo {
                token: TokenTypes::VariableCall,
                chars_read: 1,
                value: "a".to_string(),
            },
            ParseInfo {
                token: TokenTypes::AssignmentOperator,
                chars_read: 1,
                value: "=".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Float,
                chars_read: 5,
                value: "1.102".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];
        let result = tokenizers::tokenize(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_variable_call_string() {
        let input = "a = \"Hello, World!\";".to_string();
        let expected = vec![
            ParseInfo {
                token: TokenTypes::VariableCall,
                chars_read: 1,
                value: "a".to_string(),
            },
            ParseInfo {
                token: TokenTypes::AssignmentOperator,
                chars_read: 1,
                value: "=".to_string(),
            },
            ParseInfo {
                token: TokenTypes::String,
                chars_read: 15,
                value: "\"Hello, World!\"".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];
        let result = tokenizers::tokenize(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_variable_call_char() {
        let input = "a = 'a';".to_string();
        let expected = vec![
            ParseInfo {
                token: TokenTypes::VariableCall,
                chars_read: 1,
                value: "a".to_string(),
            },
            ParseInfo {
                token: TokenTypes::AssignmentOperator,
                chars_read: 1,
                value: "=".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Char,
                chars_read: 3,
                value: "'a'".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];
        let result = tokenizers::tokenize(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_variable_call_boolean() {
        let input = "a = True;".to_string();
        let expected = vec![
            ParseInfo {
                token: TokenTypes::VariableCall,
                chars_read: 1,
                value: "a".to_string(),
            },
            ParseInfo {
                token: TokenTypes::AssignmentOperator,
                chars_read: 1,
                value: "=".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Bool,
                chars_read: 4,
                value: "True".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];
        let result = tokenizers::tokenize(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_function_call() {
        let input = "add(1, 2);".to_string();
        let expected = vec![
            ParseInfo {
                token: TokenTypes::FunctionCall,
                chars_read: 3,
                value: "add".to_string(),
            },
            ParseInfo {
                token: TokenTypes::LeftParenthesis,
                chars_read: 1,
                value: "(".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Int,
                chars_read: 1,
                value: "1".to_string(),
            },
            ParseInfo {
                token: TokenTypes::ArgumentSeparator,
                chars_read: 1,
                value: ",".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Int,
                chars_read: 1,
                value: "2".to_string(),
            },
            ParseInfo {
                token: TokenTypes::RightParenthesis,
                chars_read: 1,
                value: ")".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];
        let result = tokenizers::tokenize(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_operation() {
        let input = "1 + 2;".to_string();
        let expected = vec![
            ParseInfo {
                token: TokenTypes::Int,
                chars_read: 1,
                value: "1".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Operator,
                chars_read: 1,
                value: "+".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Int,
                chars_read: 1,
                value: "2".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];
        let result = tokenizers::tokenize(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_operation_with_parethisis() {
        let input = "(1 + 2);".to_string();
        let expected = vec![
            ParseInfo {
                token: TokenTypes::LeftParenthesis,
                chars_read: 1,
                value: "(".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Int,
                chars_read: 1,
                value: "1".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Operator,
                chars_read: 1,
                value: "+".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Int,
                chars_read: 1,
                value: "2".to_string(),
            },
            ParseInfo {
                token: TokenTypes::RightParenthesis,
                chars_read: 1,
                value: ")".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];
        let result = tokenizers::tokenize(input);
        assert_eq!(result, expected);
    }

    /*
    #[test]
    fn test_tokenize_array_collection_declaration() {
        let input = "let a: array<int> = [1, 2, 3];".to_string();
        let expected = vec![
            ParseInfo {
                token: TokenTypes::Collection,
                chars_read: 1,
                value: "a".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Array,
                chars_read: 10,
                value: "array".to_string(),
            },
            ParseInfo {
                token: TokenTypes::ArrayType,
                chars_read: 3,
                value: "int".to_string(),
            },
            ParseInfo {
                token: TokenTypes::AssignmentOperator,
                chars_read: 1,
                value: "=".to_string(),
            },
            ParseInfo {
                token: TokenTypes::Collection,
                chars_read: 7,
                value: "[1, 2, 3]".to_string(),
            },
            ParseInfo {
                token: TokenTypes::SemiColon,
                chars_read: 1,
                value: ";".to_string(),
            },
        ];
    }
    */
}
