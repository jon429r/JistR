/*
* This file takes in the wrtten source code and tokenizes it into a vector of tokens
* These tokens can then be used to create an AST
*/

pub mod tokenizers {
    use crate::base_variable::variables::VARIABLE_STACK;
    use crate::token_type::token_types::TokenTypes;

    #[derive(Debug, PartialEq, Clone)]
    pub struct ParseInfo {
        pub token: TokenTypes,
        pub chars_read: i32,
        pub value: String,
    }

    impl ParseInfo {
        pub fn new(token: TokenTypes, chars_read: i32, value: String) -> Self {
            ParseInfo {
                token,
                chars_read,
                value,
            }
        }
        pub fn to_string(&self) -> String {
            let mut str = String::new();
            str.push_str("TokenType: ");
            str.push_str(&self.token.to_string());
            str.push_str("Chars read: ");
            str.push_str(&self.chars_read.to_string());
            str.push_str("value: ");
            str.push_str(&self.value.to_string());
            return str;
        }
    }
    impl ParseInfo {
        fn eq(&self, other: &Self) -> bool {
            self.token == other.token && self.chars_read == other.chars_read
        }
    }

    pub struct Token<T> {
        pub value: T,
        pub token_type: TokenTypes,
    }
    impl<T: std::fmt::Display> Token<T> {
        pub fn to_string(&self) -> String {
            let mut str = String::new();
            str.push_str(&self.value.to_string());
            str.push_str(&self.token_type.to_string());
            return str;
        }
    }

    impl<T> Token<T> {
        pub fn new(value: T, token_type: TokenTypes) -> Self {
            Token { value, token_type }
        }
    }
    static mut MULTLINECOMMENT: bool = false;

    pub fn tokenize(expression: String) -> Vec<ParseInfo> {
        //print!("Tokenizing expression: {}\n", expression);
        let mut token_list: Vec<ParseInfo> = Vec::new();
        let none = ParseInfo::new(TokenTypes::None, 0, "none".to_string());

        let mut index = 0;
        let chars: Vec<char> = expression.chars().collect();

        while index < chars.len() {
            let char = chars[index];
            let nextchar = if index + 1 < chars.len() {
                chars[index + 1]
            } else {
                '\0' // Null character for out-of-bounds safety
            };

            if char == ';' {
                index += 1;
                token_list.push(ParseInfo::new(TokenTypes::SemiColon, 1, ";".to_string()));
                continue;
            }

            if char == ' ' || char == '\n' || char == '\r' || char == '\t' {
                index += 1;
                continue; // Skip whitespace characters
            }

            unsafe {
                if MULTLINECOMMENT {
                    // Skip characters within multi-line comments
                    while index < chars.len() {
                        let char = chars[index];
                        let nextchar = if index + 1 < chars.len() {
                            chars[index + 1]
                        } else {
                            '\0'
                        };
                        if char == '*' && nextchar == '/' {
                            MULTLINECOMMENT = false;
                            index += 2;
                            break;
                        }
                        index += 1;
                    }
                    continue;
                }

                if char == '/' && nextchar == '/' {
                    // Single-line comment - skip the rest of the line
                    let info = ParseInfo::new(
                        TokenTypes::Comment,
                        (chars.len() - index) as i32,
                        "none".to_string(),
                    );
                    token_list.push(info);
                    break;
                } else if char == '/' && nextchar == '*' {
                    MULTLINECOMMENT = true;
                    index += 2;
                    continue;
                }
            }

            // Process regular tokens
            let info: ParseInfo = read_token(&expression, index);
            if info.token != none.token {
                index += info.chars_read as usize;
                token_list.push(info);
                continue;
            }

            index += 1;
        }

        token_list
    }

    pub fn read_token(expression: &String, index: usize) -> ParseInfo {
        let none: ParseInfo = ParseInfo::new(TokenTypes::None, 0, "none".to_string());

        let mut j = index;
        let mut decimals = 0;

        // Loop through the expression
        while j < expression.len() {
            let char: char = expression.chars().nth(j).unwrap();

            // Break if the character is not a digit or decimal point
            if !char.is_digit(10) && char != '.' {
                break;
            }

            // Count decimal points
            if char == '.' {
                decimals += 1;
                // If there are multiple decimal points, return None
                if decimals > 1 {
                    return none;
                }
            }

            j += 1;
        }

        let char = expression.chars().nth(index).unwrap();

        let info = read_boolean(expression.to_string(), index);
        if info.token != none.token {
            return info;
        }

        let info = read_numbers(expression.to_string(), char, index);
        if info.token != none.token {
            return info;
        }

        let info = read_operators(expression.to_string(), char, index);
        if info.token != none.token {
            return info;
        }
        let info = read_strings_chars(expression.to_string(), char, index);

        if info.token != none.token {
            return info;
        }

        let info = read_variable_declaration(expression, index);
        if info.token != none.token {
            return info;
        }

        let info = read_function_declaration(expression, index);
        // Handle number or function parsing if no matches yet
        if info.token != none.token {
            return info;
        }

        let info = read_function_call(expression, index);
        if info.token != none.token {
            return info;
        }

        let info = read_function_assignment(expression, index);
        if info.token != none.token {
            return info;
        }

        let info = read_variable_assignment(expression, index);
        if info.token != none.token {
            return info;
        }

        let info = read_variable_call(expression, index);
        if info.token != none.token {
            return info;
        }

        /*
                // tokenize char value if it matches 'a' to 'z' or 'A' to 'Z'
                if char.is_alphabetic() {
                    let chars_read = 1;
                    return ParseInfo::new(
                        TokenTypes::Char,
                        chars_read.try_into().unwrap(),
                        char.to_string(),
                    );
                }
        */
        /*
        // check for string values
        if char == '"' {
            let mut j = index + 1;
            while j < expression.len() {
                if expression.chars().nth(j).unwrap() == '"' {
                    break;
                }
                j += 1;
            }
            let chars_read = j - index + 1;
            return ParseInfo::new(
                TokenTypes::String,
                chars_read.try_into().unwrap(),
                expression[index..j + 1].to_string(),
            );
        }*/
        return none;
    }

    fn read_boolean(expression: String, index: usize) -> ParseInfo {
        let mut j = index;
        let mut boolean: String = String::new();
        let bool_compare1 = "True";
        let bool_compare2 = "False";

        while j < expression.len() {
            if let Some(char) = expression.chars().nth(j) {
                if char.is_alphabetic() {
                    boolean.push(char);
                    j += 1; // Increment the index to progress through the string
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if boolean == bool_compare1 || boolean == bool_compare2 {
            return ParseInfo::new(TokenTypes::Bool, (j - index).try_into().unwrap(), boolean);
        }

        ParseInfo::new(TokenTypes::None, 0, "none".to_string())
    }

    fn read_function_declaration(expression: &String, index: usize) -> ParseInfo {
        let mut j = index;
        let mut function: String = String::new();
        let func_compare = "func";

        // Collect alphabetic characters forming the function name
        while j < expression.len() {
            if let Some(char) = expression.chars().nth(j) {
                if char.is_alphabetic() {
                    function.push(char);
                } else {
                    break;
                }
            }
            j += 1;

            // If we've collected 4 characters, check if they form "func"
            if function.len() == 4 {
                if function == func_compare {
                    // Skip whitespace and collect the actual function name
                    let mut function_name = String::new();
                    while j < expression.len() {
                        if let Some(char) = expression.chars().nth(j) {
                            if char.is_alphabetic() {
                                function_name.push(char);
                            } else if char != ' ' {
                                break;
                            }
                        }
                        j += 1;
                    }
                    //print!("Function name: {}\n", function_name);

                    // Return ParseInfo with the length of "func" and function name
                    return ParseInfo::new(
                        TokenTypes::Function,
                        (j - index).try_into().unwrap(),
                        "none".to_string(),
                    );
                } else {
                    return ParseInfo::new(
                        TokenTypes::None,
                        (j - index).try_into().unwrap(),
                        "none".to_string(),
                    );
                }
            }

            // If more than 4 characters, remove the first character (sliding window)
            if function.len() > 4 {
                function.remove(0);
            }
        }

        // Return default ParseInfo if function not found
        ParseInfo::new(TokenTypes::None, 0, "none".to_string())
    }

    static mut PARSEFUNCTIONCALL: bool = false;

    fn read_function_call(expression: &String, index: usize) -> ParseInfo {
        let mut j = index;
        let mut function_name = String::new();
        let chars: Vec<char> = expression.chars().collect();

        // Collect the function name
        while j < chars.len() {
            let char = chars[j];
            let next_char = chars.get(j + 1).cloned().unwrap_or('\0');

            if char == '=' {
                return ParseInfo::new(TokenTypes::None, 0, "none".to_string());
            }
            if char == '(' {
                // Detected function call: mark it and return info
                unsafe { PARSEFUNCTIONCALL = true };
                return ParseInfo::new(
                    TokenTypes::FunctionCall,
                    (j - index).try_into().unwrap(),
                    function_name.clone(),
                );
            } else if !char.is_whitespace() {
                // Build up function name
                function_name.push(char);
            }
            j += 1;
        }

        // Parse the function parameters
        if unsafe { PARSEFUNCTIONCALL } {
            let mut parameter = String::new();

            while j < chars.len() {
                let char = chars[j];
                let next_char = chars.get(j + 1).cloned().unwrap_or('\0');

                if char == ')' {
                    // End of function call parameters
                    unsafe { PARSEFUNCTIONCALL = false };
                    return ParseInfo::new(
                        TokenTypes::RightParenthesis,
                        (j - index).try_into().unwrap(),
                        ")".to_string(),
                    );
                } else if char == ',' {
                    // Handle function arguments separated by commas
                    if !parameter.is_empty() {
                        return ParseInfo::new(
                            TokenTypes::FunctionArguments,
                            (j - index).try_into().unwrap(),
                            parameter.clone(),
                        );
                    }
                    parameter.clear();
                } else if !char.is_whitespace() {
                    // Collect parameter characters
                    parameter.push(char);
                }

                j += 1;
            }
        }

        // Default return if no valid function call or parameters found
        ParseInfo::new(TokenTypes::None, 0, "none".to_string())
    }

    fn read_variable_declaration(expression: &String, index: usize) -> ParseInfo {
        let mut j = index;
        let mut variable: String = String::new();
        let let_compare = "let";

        // Collect alphabetic characters forming the variable name
        while j < expression.len() {
            if let Some(char) = expression.chars().nth(j) {
                if char.is_alphabetic() {
                    variable.push(char);
                } else {
                    break;
                }
            }
            j += 1;

            if variable.len() == 3 {
                if variable == let_compare {
                    // Skip whitespace and collect the actual variable name
                    let mut variable_name = String::new();
                    while j < expression.len() {
                        if let Some(char) = expression.chars().nth(j) {
                            if char.is_alphabetic() {
                                variable_name.push(char);
                            } else if char != ' ' {
                                break;
                            } else if char == ':' {
                                break;
                            }
                        }
                        j += 1;
                    }
                    //print!("Variable name: {}\n", variable_name);

                    return ParseInfo::new(
                        TokenTypes::Variable,
                        (j - index).try_into().unwrap(),
                        variable_name,
                    );
                } else {
                    return ParseInfo::new(TokenTypes::None, 0, "none".to_string());
                }
            }

            if variable.len() > 3 {
                variable.remove(0);
            }
        }

        // Return default ParseInfo if function not found
        ParseInfo::new(TokenTypes::None, 0, "none".to_string())
    }

    fn read_function_assignment(expression: &String, index: usize) -> ParseInfo {
        let chars: Vec<char> = expression.chars().collect();
        let mut j = index;

        // Look for the `->` pattern
        while j < chars.len() {
            let char = chars[j];
            let nextchar = if j + 1 < chars.len() {
                chars[j + 1]
            } else {
                '\0' // Null character for out-of-bounds safety
            };

            if char == '-' && nextchar == '>' {
                j += 2; // Move past `->`

                // Skip whitespace after `->`
                while j < chars.len() && chars[j].is_whitespace() {
                    j += 1;
                }

                // Collect the return type (lowercase type)
                let mut return_type = String::new();
                while j < chars.len() {
                    let char = chars[j];
                    if char.is_lowercase() || char == '_' {
                        return_type.push(char);
                        j += 1;
                    } else {
                        break;
                    }
                }

                // Check if we collected a valid return type
                if !return_type.is_empty() {
                    return ParseInfo::new(
                        TokenTypes::ReturnTypeAssignment,
                        (j - index).try_into().unwrap(),
                        return_type,
                    );
                }
            } else {
                j += 1; // Continue scanning if `->` not found
            }
        }
        ParseInfo::new(TokenTypes::None, 0, "none".to_string())
    }

    fn read_variable_assignment(expression: &String, index: usize) -> ParseInfo {
        let chars: Vec<char> = expression.chars().collect();
        let mut j = index;
        let original_index = index;

        // Look for the `:` pattern
        while j < chars.len() {
            let char = chars[j];

            if char == ':' {
                j += 1; // Move past `:`

                // Skip whitespace after `:`
                while j < chars.len() && chars[j].is_whitespace() {
                    j += 1;
                }

                // Collect the type (assuming it is lowercase or an identifier)
                let mut var_type = String::new();
                while j < chars.len() {
                    let char = chars[j];
                    if char.is_lowercase() || char.is_alphanumeric() || char == '_' {
                        var_type.push(char);
                        j += 1;
                    } else {
                        break;
                    }
                }

                // Skip whitespace after the type and check for `=`
                while j < chars.len() && chars[j].is_whitespace() {
                    j += 1;
                }

                if j < chars.len() && chars[j] == '=' {
                    // Print for debugging
                    /*
                        print!(

                            "Variable Type: '{}', Length: {}\n",
                            var_type,
                            j - original_index
                        );
                    */
                    return ParseInfo::new(
                        TokenTypes::VarTypeAssignment,
                        (j - original_index).try_into().unwrap(),
                        var_type,
                    );
                }
            } else {
                j += 1; // Continue scanning if `:` not found
            }
        }

        // Return None if no valid variable assignment found
        ParseInfo::new(TokenTypes::None, 0, "none".to_string())
    }

    fn read_variable_call(expression: &String, index: usize) -> ParseInfo {
        let mut j = index;
        let mut variable_name = String::new();

        // Iterate over characters in the string starting at 'index'
        let chars = expression.chars().skip(index).enumerate();

        for (_i, char) in chars {
            // Check if the current or next character is '=' (assignment)
            let next_char = expression.chars().nth(j + 1).unwrap_or('\0');
            if char == '(' || next_char == '(' {
                return ParseInfo::new(
                    TokenTypes::FunctionCall,
                    (j - index).try_into().unwrap(),
                    variable_name,
                );
            } else if char == '=' || next_char == '=' {
                return ParseInfo::new(
                    TokenTypes::VariableCall,
                    (j - index).try_into().unwrap(),
                    variable_name,
                );
            }
            // Collect valid variable name characters (alphanumeric and '_')

            if char.is_alphanumeric() || char == '_' {
                variable_name.push(char);
            } else if char.is_whitespace() {
                continue;
            } else {
                break;
            }

            j += 1;
        }

        // After collecting the variable name, check if it exists in the stack
        if !variable_name.is_empty() {
            for variable in unsafe { &VARIABLE_STACK } {
                if variable.name == variable_name {
                    return ParseInfo::new(
                        TokenTypes::VariableCall,
                        (j - index).try_into().unwrap(),
                        variable_name,
                    );
                }
            }
        }

        // Return None if no valid variable call found
        ParseInfo::new(
            TokenTypes::None,
            0,
            "No valid variable call found".to_string(),
        )
    }

    pub fn read_strings_chars(expression: String, char: char, index: usize) -> ParseInfo {
        match char {
            '"' | '\'' => {
                let mut j = index + 1;
                while j < expression.len() {
                    match char {
                        '"' => {
                            if expression.chars().nth(j).unwrap() == char {
                                return ParseInfo::new(
                                    TokenTypes::String,
                                    (j - index + 1).try_into().unwrap(),
                                    expression[index..j + 1].to_string(),
                                );
                            }
                        }
                        '\'' => {
                            if expression.chars().nth(j).unwrap() == char {
                                return ParseInfo::new(
                                    TokenTypes::Char,
                                    (j - index + 1).try_into().unwrap(),
                                    expression[index..j + 1].to_string(),
                                );
                            }
                        }
                        _ => {}
                    }
                    if expression.chars().nth(j).unwrap() == char {
                        return ParseInfo::new(
                            TokenTypes::Char,
                            1,
                            expression[index..j + 1].to_string(),
                        );
                    }
                    j += 1;
                }
                {
                    let chars_read = j - index + 1;
                    return ParseInfo::new(
                        TokenTypes::String,
                        chars_read.try_into().unwrap(),
                        expression[index..j + 1].to_string(),
                    );
                }
            }
            _ => {
                return ParseInfo::new(TokenTypes::None, 0, "none".to_string());
            }
        }
    }

    pub fn read_numbers(expression: String, char: char, index: usize) -> ParseInfo {
        // Extract the number substring
        let mut j = index;
        let mut decimals = 0;

        // Traverse through the expression to identify the full number (including decimals)
        while j < expression.len()
            && (expression[j..j + 1].chars().all(|c| c.is_digit(10))
                || expression[j..j + 1] == *".")
        {
            if expression[j..j + 1] == *"." {
                decimals += 1;
            }
            j += 1;
        }

        let number_str = &expression[index..j];

        // Check if it's a valid number and if there's only one decimal point
        if decimals <= 1 && number_str.parse::<f64>().is_ok() {
            let chars_read = j - index;
            if decimals == 0 {
                return ParseInfo::new(
                    TokenTypes::Int,
                    chars_read.try_into().unwrap(),
                    number_str.to_string(),
                );
            } else {
                return ParseInfo::new(
                    TokenTypes::Float,
                    chars_read.try_into().unwrap(),
                    number_str.to_string(),
                );
            }
        }

        // Return None token type if parsing fails
        ParseInfo::new(TokenTypes::None, 0, "none".to_string())
    }

    pub fn read_operators(expression: String, char: char, index: usize) -> ParseInfo {
        match char {
            '+' | '-' | '*' | '/' => {
                let chars_read = 1;
                return ParseInfo::new(
                    TokenTypes::Operator,
                    chars_read.try_into().unwrap(),
                    char.to_string(),
                );
            }
            '(' => return ParseInfo::new(TokenTypes::LeftParenthesis, 1, char.to_string()),
            ')' => return ParseInfo::new(TokenTypes::RightParenthesis, 1, char.to_string()),
            '{' => return ParseInfo::new(TokenTypes::LeftCurly, 1, char.to_string()),
            '}' => return ParseInfo::new(TokenTypes::RightCurly, 1, char.to_string()),
            ',' => return ParseInfo::new(TokenTypes::ArgumentSeparator, 1, char.to_string()),
            '=' => return ParseInfo::new(TokenTypes::AssignmentOperator, 1, char.to_string()),
            _ => {
                return ParseInfo::new(TokenTypes::None, 0, "none".to_string());
            }
        }
    }
}

mod tests {

    use super::*;
    use crate::token_type::token_types::TokenTypes;
    use crate::tokenizer::tokenizers::ParseInfo;

    #[test]
    fn test_tokenize_variable_declaration_int() {
        let input = "let a: int = 1;".to_string();

        let expected = vec![
            ParseInfo {
                token: TokenTypes::Variable,
                chars_read: 5,
                value: "a".to_string(),
            },
            ParseInfo {
                token: TokenTypes::VarTypeAssignment,
                chars_read: 6,
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
                chars_read: 5,
                value: "a".to_string(),
            },
            ParseInfo {
                token: TokenTypes::VarTypeAssignment,
                chars_read: 8,
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
                chars_read: 5,
                value: "a".to_string(),
            },
            ParseInfo {
                token: TokenTypes::VarTypeAssignment,
                chars_read: 9,
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
                chars_read: 5,
                value: "a".to_string(),
            },
            ParseInfo {
                token: TokenTypes::VarTypeAssignment,
                chars_read: 7,
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
    fn test_tokenize_variable_declaration_boolean() {
        let input = "let a: bool = True;".to_string();
        let expected = vec![
            ParseInfo {
                token: TokenTypes::Variable,
                chars_read: 5,
                value: "a".to_string(),
            },
            ParseInfo {
                token: TokenTypes::VarTypeAssignment,
                chars_read: 7,
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
}
