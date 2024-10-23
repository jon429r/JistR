pub mod loop_tokenizers {
    use crate::statement_tokenizer::tokenizer::tokenizers::ParseInfo;
    use crate::token_type::token_types::TokenTypes;

    fn extract_condition(chars: &[char], mut index: usize) -> (String, usize) {
        let mut condition = String::new();
        let mut parenthesis_count = 1;

        // Extract content within parentheses
        while index < chars.len() && parenthesis_count > 0 {
            let c = chars[index];
            if c == '(' {
                parenthesis_count += 1;
            } else if c == ')' {
                parenthesis_count -= 1;
                if parenthesis_count == 0 {
                    index += 1;

                    break;
                }
            }
            condition.push(c);
            index += 1;
        }
        //tokenize the condition
        (condition, index)
    }

    fn parse_keyword(expression: &str, index: usize, keyword: &str) -> Option<usize> {
        let slice = &expression[index..];
        if slice.starts_with(keyword) {
            Some(index + keyword.len())
        } else {
            None
        }
    }

    pub fn tokenize_for_while_statement(expression: &str, index: usize) -> ParseInfo {
        let chars: Vec<char> = expression.chars().collect();
        let mut j = index;

        while j < chars.len() {
            let c = chars[j];
            if c.is_whitespace() {
                j += 1;
                continue;
            }

            // Tokenize the "for" loop
            if let Some(new_index) = parse_keyword(expression, j, "for") {
                j = new_index;

                // Look for '(' and extract the loop condition
                while j < chars.len() && chars[j].is_whitespace() {
                    j += 1;
                }

                if chars[j] == '(' {
                    let (condition, new_j) = extract_condition(&chars, j + 1);
                    return ParseInfo::new(
                        TokenTypes::For {
                            statement: condition,
                        },
                        new_j.try_into().unwrap(),
                        "for".to_string(),
                    );
                }

            // Tokenize the "while" loop
            } else if let Some(new_index) = parse_keyword(expression, j, "while") {
                j = new_index;

                // Look for '(' and extract the loop condition
                while j < chars.len() && chars[j].is_whitespace() {
                    j += 1;
                }

                if chars[j] == '(' {
                    let (condition, new_j) = extract_condition(&chars, j + 1);
                    return ParseInfo::new(
                        TokenTypes::While {
                            statement: condition,
                        },
                        new_j.try_into().unwrap(),
                        "while".to_string(),
                    );
                }
            }

            j += 1;
        }

        ParseInfo::new(TokenTypes::None, 0, "none".to_string())
    }
}
