pub mod collection_tokenizers {
    use crate::statement_tokenizer::tokenizer::tokenizers::ParseInfo;
    use crate::token_type::token_types::TokenTypes;

    pub fn read_collection_assignment(expression: &str, index: usize) -> ParseInfo {
        let chars: Vec<char> = expression.chars().collect();
        let mut j = index;

        let mut variable_name = String::new();
        let mut collection_type = String::new();
        let mut stored_value_type_tuple = (String::new(), String::new());
        let mut inside_angle_brackets = false;
        let mut found_comma = false;

        // Check for "let" keyword
        while j < expression.len() && expression[j..].starts_with("let") {
            j += 3; // Move past "let"
            break;
        }

        // Skip any spaces after "let"
        while j < expression.len() && chars[j].is_whitespace() {
            j += 1;
        }

        // Collect the variable name
        while j < expression.len() {
            let char = chars[j];
            if char.is_alphabetic() || char == '_' {
                variable_name.push(char);
            } else if char == ':' {
                break; // Break on type declaration
            } else if !char.is_whitespace() {
                return ParseInfo::new(TokenTypes::None, 0, "none".to_string());
            }
            j += 1;
        }

        // Skip spaces after the variable name
        while j < chars.len() && chars[j].is_whitespace() {
            j += 1;
        }

        // Look for ':' followed by a collection type with '<>'
        if j < chars.len() && chars[j] == ':' {
            j += 1; // Move past ':'
            while j < chars.len() && chars[j].is_whitespace() {
                j += 1;
            }

            // Collect the collection type (e.g., "dict")
            while j < chars.len() {
                let char = chars[j];
                if char.is_alphabetic() || char == '_' {
                    collection_type.push(char);
                    j += 1;
                } else if char == '<' {
                    inside_angle_brackets = true;
                    j += 1;
                    break;
                } else {
                    break;
                }
            }

            // Now we are inside the '<>' brackets to collect the tuple types (e.g., char, int)
            while j < chars.len() && inside_angle_brackets {
                let char = chars[j];
                if char == '>' {
                    inside_angle_brackets = false;
                    j += 1; // Move past '>'
                    break;
                } else if char == ',' {
                    found_comma = true;
                    j += 1; // Move past ','
                    continue;
                }

                // Collect key type (before comma) and value type (after comma)
                if found_comma {
                    stored_value_type_tuple.1.push(char);
                } else {
                    stored_value_type_tuple.0.push(char);
                }
                j += 1;
            }

            // Skip spaces after the type declaration
            while j < chars.len() && chars[j].is_whitespace() {
                j += 1;
            }

            // Stop parsing at '=' (ignore the collection initialization part)
            if j < chars.len() && chars[j] == '=' {
                // Return the collected information about the collection type
                if !found_comma {
                    return ParseInfo::new(
                        TokenTypes::Collection,
                        (j - index).try_into().unwrap(),
                        format!("{}<{}>", collection_type, stored_value_type_tuple.0.trim()),
                    );
                } else {
                    return ParseInfo::new(
                        TokenTypes::Collection,
                        (j - index).try_into().unwrap(),
                        format!(
                            "{}<{}, {}>",
                            collection_type,
                            stored_value_type_tuple.0.trim(),
                            stored_value_type_tuple.1.trim()
                        ),
                    );
                }
            }
        }

        // If no valid collection declaration is found
        ParseInfo::new(TokenTypes::None, 0, "none".to_string())
    }
}
