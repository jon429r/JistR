pub mod basic_tokenizers {
    use crate::statement_tokenizer::tokenizer::tokenizers::ParseInfo;
    use crate::token_type::token_types::TokenTypes;

    pub fn read_object_call(expression: &String) -> ParseInfo {
        // if the whole expression is a group of characters it is a object call eg foo eg a eg
        // rad_var
        let mut object_name: String = String::new();
        let mut j = 0;

        while j < expression.len() {
            let char: char = expression.chars().nth(j).unwrap();
            if !char.is_alphabetic() {
                return ParseInfo::new(TokenTypes::None, 0, "none".to_string());
            } else {
                //add char to object object_name
                object_name.push(char);
            }
            j += 1;
        }
        return ParseInfo::new(
            TokenTypes::ObjectCall {
                name: object_name.clone(),
            },
            expression.len().try_into().unwrap(),
            object_name,
        );
    }

    pub fn read_boolean(expression: String, index: usize) -> ParseInfo {
        let mut j = index;
        let mut boolean: String = String::new();
        let bool_compare1 = "True";
        let bool_compare2 = "False";
        let bool_compare3 = "true";
        let bol_compare4 = "false";

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

        if boolean == bool_compare1
            || boolean == bool_compare2
            || boolean == bool_compare3
            || boolean == bol_compare4
        {
            return ParseInfo::new(TokenTypes::Bool, (j - index).try_into().unwrap(), boolean);
        }

        ParseInfo::new(TokenTypes::None, 0, "none".to_string())
    }

    pub fn read_operators(
        expression: String,
        char: char,
        next_char: char,
        index: usize,
    ) -> ParseInfo {
        let two_chars = format!("{}{}", char, next_char);
        match two_chars.to_string().as_str() {
            "==" | "!=" | ">=" | "<=" | "&&" | "||" | "++" | "--" => {
                let chars_read = 2;
                return ParseInfo::new(
                    TokenTypes::Operator,
                    chars_read.try_into().unwrap(),
                    two_chars,
                );
            }
            _ => {}
        }
        match char {
            '+' | '-' | '*' | '/' | '>' | '<' | '!' => {
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
}
