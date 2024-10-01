pub mod function_tokenizers {
    use crate::statement_tokenizer::tokenizer::tokenizers::ParseInfo;
    use crate::token_type::token_types::TokenTypes;

    pub fn read_function_assignment(expression: &String, index: usize) -> ParseInfo {
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

    static mut PARSEFUNCTIONCALL: bool = false;

    pub fn read_function_call(expression: &String, index: usize) -> ParseInfo {
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

    pub fn read_function_declaration(expression: &String, index: usize) -> ParseInfo {
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
}
