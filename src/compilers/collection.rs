
use crate::base_variable::base_types::BaseTypes;
use crate::collection::collections::{Array, Dictionary};
use crate::node::nodes::{ASTNode, CollectionNode};

pub fn parse_collection_declaration(expression: &[ASTNode]) -> bool {
    println!("Expression: {:?}", expression);
    
    // Check if the first node is a Collection
    if let Some(node) = expression.get(0) {
        if let ASTNode::Collection(collection_node) = node {
            // Access fields from the CollectionNode
            let name = &collection_node.name;
            let collection_type = &collection_node.collection_type;
            let value_type_single = collection_node.value_type_single.as_deref().unwrap_or("");
            let value_type_tuple = collection_node.value_type_tuple.as_ref().map(|(v1, v2)| (v1.clone(), v2.clone()));

            // Convert the tuple elements to BaseTypes
            let key_type: BaseTypes = value_type_tuple.as_ref().map_or(BaseTypes::Null, |(v1, _)| v1.clone().into());
            let value_type: BaseTypes = value_type_tuple.as_ref().map_or(BaseTypes::Null, |(_, v2)| v2.clone().into());

            // Print out the collected values for debugging
            println!("Collection Name: {}", name);
            println!("Collection Type: {}", collection_type);
            println!("Single Value Type: {}", value_type_single);
            println!("Tuple Value Types: {:?}", value_type_tuple);

            match collection_type.as_str() {
                "array" => {
                    // Create a new Array
                    let mut values: Vec<BaseTypes> = Vec::new();
                    // Add the values to the array
                    for node in &expression[1..] {
                        match node {
                            ASTNode::Int(int) => {
                                let value = BaseTypes::Int(int.value);
                                values.push(value);
                            }
                            ASTNode::Float(float) => {
                                let value = BaseTypes::Float(float.value.into()); // Corrected from Int to Float
                                values.push(value); // Changed from value.push(value) to values.push(value)
                            }
                            ASTNode::String(string) => {
                                let value = BaseTypes::StringWrapper(string.value.clone()); // Corrected from Int to String
                                values.push(value);
                            }
                            ASTNode::Char(char) => {
                                let value = BaseTypes::Char(char.value); // Corrected from Int to Char
                                values.push(value);
                            }
                            ASTNode::Bool(bool) => {
                                let value = BaseTypes::Bool(bool.value); // Corrected from Int to Bool
                                values.push(value);
                            }
                            ASTNode::AssignmentOperator(_) | ASTNode::ArgumentSeparator | ASTNode::LeftBracket => {},
                            ASTNode::RightBracket => { break; }
                            _ => {
                                println!("Syntax Error: Value type not recognized.");
                                return false;
                            }
                        }                 
                    }

                    let array = Array::new(name.clone(), key_type.clone(), values); // Assume array expects BaseTypes
                    println!("Created new Array: {}", array.to_string());
                }
                "dict" => {
                    // Create a new Dictionary
                    let dict = Dictionary::new(name.clone(), key_type, value_type);
                    println!("Created new Dict: {}", dict.to_string());
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

