
use crate::base_variable::base_types::BaseTypes;
use crate::collection::collections::{Array, Dictionary};
use crate::node::nodes::ASTNode;
use crate::collection::ARRAY_STACK;
use crate::collection::DICTIONARY_STACK;
//use std::sync::Mutex;
//use lazy_static::lazy_static;

fn add_to_dictionary_stack(dict: Dictionary) {
    DICTIONARY_STACK.lock().unwrap().push(dict.clone());
    // You can still use `dict` after this line because we cloned it
    //println!("dict pushed to stack")
}

fn add_to_array_stack(array: Array) {
    ARRAY_STACK.lock().unwrap().push(array.clone());

}

pub fn parse_collection_declaration(expression: &[ASTNode]) -> bool {
    //println!("Expression: {:?}", expression);
    
    //let mut array_stack = ARRAY_STACK.lock().unwrap();
    //let mut dict_stack = DICTIONARY_STACK.lock().unwrap();

    // Check if the first node is a Collection
    if let Some(node) = expression.get(0) {
        if let ASTNode::Collection(collection_node) = node {
            // Access fields from the CollectionNode
            let name = &collection_node.name;
            let collection_type = &collection_node.collection_type;
            let value_type_single = collection_node.value_type_single.as_deref().unwrap_or("");
            let value_type_tuple = collection_node.value_type_tuple.as_ref().map(|(v1, v2)| (v1.clone(), v2.clone()));

            // Convert the tuple elements to BaseTypes
            let single_key_type: BaseTypes = value_type_single.clone().into();
            let key_type: BaseTypes = value_type_tuple.as_ref().map_or(BaseTypes::Null, |(v1, _)| v1.clone().into());
            let value_type: BaseTypes = value_type_tuple.as_ref().map_or(BaseTypes::Null, |(_, v2)| v2.clone().into());

            // Print out the collected values for debugging
            /*
            println!("Collection Name: {}", name);
            println!("Collection Type: {}", collection_type);
            println!("Single Value Type: {}", value_type_single);
            println!("Tuple Value Types: {:?}", value_type_tuple);
            */

            match collection_type.as_str() {
                "array" => {
                    // Create a new Array
                    let mut values: Vec<BaseTypes> = Vec::new();
                    for node in &expression[1..] {
                        match node {
                            ASTNode::Int(int) => values.push(BaseTypes::Int(int.value)),
                            ASTNode::Float(float) => values.push(BaseTypes::Float(float.value.into())),
                            ASTNode::String(string) => values.push(BaseTypes::StringWrapper(string.value.clone())),
                            ASTNode::Char(char) => values.push(BaseTypes::Char(char.value)),
                            ASTNode::Bool(bool) => values.push(BaseTypes::Bool(bool.value)),
                            ASTNode::AssignmentOperator(_) | ASTNode::ArgumentSeparator | ASTNode::LeftBracket => {},
                            ASTNode::RightBracket => break,
                            _ => {
                                println!("Syntax Error: Value type not recognized.");
                                return false;
                            }
                        }                 
                    }

                    let array = Array::new(name.clone(), single_key_type.clone(), values);
                    add_to_array_stack(array);
                    //println!("Added array to stack");
                }
                "dict" => {
                    // Create a new Dictionary
                    let mut values: Vec<(BaseTypes, BaseTypes)> = Vec::new();
                    let mut key: Option<BaseTypes> = None;
                    let mut have_fat_arrow = false;

                    for node in &expression[1..] {
                        match node {
                            ASTNode::Int(int) => {
                                let base_int = BaseTypes::Int(int.value);
                                if have_fat_arrow {
                                    // This is the value
                                    if let Some(k) = key.take() {
                                        values.push((k, base_int));
                                        have_fat_arrow = false;
                                    } else {
                                        println!("Syntax Error: Missing key.");
                                        return false;
                                    }
                                } else {
                                    // This is the key
                                    if key.is_some() {
                                        println!("Syntax Error: Unexpected key.");
                                        return false;
                                    }
                                    key = Some(base_int);
                                }
                            }
                            ASTNode::Float(float) => {
                                let base_float = BaseTypes::Float(float.value.into());
                                if have_fat_arrow {
                                    if let Some(k) = key.take() {
                                        values.push((k, base_float));
                                        have_fat_arrow = false;
                                    } else {
                                        println!("Syntax Error: Missing key.");
                                        return false;
                                    }
                                } else {
                                    if key.is_some() {
                                        println!("Syntax Error: Unexpected key.");
                                        return false;
                                    }
                                    key = Some(base_float);
                                }
                            }
                            ASTNode::String(string) => {
                                let base_string = BaseTypes::StringWrapper(string.value.clone());
                                if have_fat_arrow {
                                    if let Some(k) = key.take() {
                                        values.push((k, base_string));
                                        have_fat_arrow = false;
                                    } else {
                                        println!("Syntax Error: Missing key.");
                                        return false;
                                    }
                                } else {
                                    if key.is_some() {
                                        println!("Syntax Error: Unexpected key.");
                                        return false;
                                    }
                                    key = Some(base_string);
                                }
                            }
                            ASTNode::Char(char) => {
                                let base_char = BaseTypes::Char(char.value);
                                if have_fat_arrow {
                                    if let Some(k) = key.take() {
                                        values.push((k, base_char));
                                        have_fat_arrow = false;
                                    } else {
                                        println!("Syntax Error: Missing key.");
                                        return false;
                                    }
                                } else {
                                    if key.is_some() {
                                        println!("Syntax Error: Unexpected key.");
                                        return false;
                                    }
                                    key = Some(base_char);
                                }
                            }
                            ASTNode::Bool(bool) => {
                                let base_bool = BaseTypes::Bool(bool.value);
                                if have_fat_arrow {
                                    if let Some(k) = key.take() {
                                        values.push((k, base_bool));
                                        have_fat_arrow = false;
                                    } else {
                                        println!("Syntax Error: Missing key.");
                                        return false;
                                    }
                                } else {
                                    if key.is_some() {
                                        println!("Syntax Error: Unexpected key.");
                                        return false;
                                    }
                                    key = Some(base_bool);
                                }
                            }
                            ASTNode::FatArrow => { have_fat_arrow = true;}
                            ASTNode::AssignmentOperator(_) => {
                            }
                            ASTNode::RightCurly => {
                                break;
                            }
                            ASTNode::LeftCurly => {}
                            ASTNode::ArgumentSeparator => {
                                // Continue to the next key-value pair
                            }
                            _ => {
                                println!("Syntax Error: Unexpected node type:{} ", node);
                                return false;
                            }
                        }
                    }

                    // Create and print the dictionary with the parsed values
                    let dict = Dictionary::new(name.clone(), key_type, value_type, values);
                    // add dict to the stack
                    
                    //println!("Before pushing to stack");
                    add_to_dictionary_stack(dict.clone());
                    //println!("Created new Dictionary with values: {}", dict.to_string());
                }
                _ => {
                    println!("Collection type not recognized.");
                    return false;
                }
            }
            return true;
        } else {
            println!("The first node is not a collection.");
            return false;
        }
    } else {
        println!("The expression vector is empty.");
        return false;
    }
}

