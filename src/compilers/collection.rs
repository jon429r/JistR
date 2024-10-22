use crate::base_variable::base_types::BaseTypes;
use crate::collection::collections::{Array, Dictionary};
use crate::collection::ARRAY_STACK;
use crate::collection::DICTIONARY_STACK;
use crate::node::nodes::ASTNode;
//use std::sync::Mutex;
//use lazy_static::lazy_static;

use std::fmt;

#[derive(Debug)]
pub enum CollectionError {
    SyntaxError(String),
    InvalidType(String),
}

impl fmt::Display for CollectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CollectionError::SyntaxError(msg) => write!(f, "Syntax Error: {}", msg),
            CollectionError::InvalidType(msg) => write!(f, "Invalid Type: {}", msg),
        }
    }
}

impl std::error::Error for CollectionError {}

fn add_to_dictionary_stack(dict: Dictionary) {
    DICTIONARY_STACK.lock().unwrap().push(dict.clone());
    // You can still use `dict` after this line because we cloned it
    //println!("dict pushed to stack")
}

fn add_to_array_stack(array: Array) {
    ARRAY_STACK.lock().unwrap().push(array.clone());
}

pub fn parse_collection_call(
    expression: &[ASTNode],
) -> Result<(String, Vec<BaseTypes>), CollectionError> {
    println!("Parsing collection call");
    if expression.is_empty() {
        return Err(CollectionError::SyntaxError("Expression is empty".into()));
    }

    let name: String = "example".into();
    let values: Vec<BaseTypes> = Vec::new();

    Ok((name, values))
}

pub fn parse_collection_declaration(expression: &[ASTNode]) -> Result<(), CollectionError> {
    // Check if the expression has any nodes
    if expression.is_empty() {
        return Err(CollectionError::SyntaxError(
            "Expression vector is empty".into(),
        ));
    }

    // Try to get the first node and match it against the expected ASTNode::Collection type
    if let Some(node) = expression.first() {
        if let ASTNode::Collection(collection_node) = node {
            let name = &collection_node.name;
            let collection_type = &collection_node.collection_type;
            let value_type_single = collection_node.value_type_single.as_deref().unwrap_or("");
            let value_type_tuple = collection_node
                .value_type_tuple
                .as_ref()
                .map(|(v1, v2)| (v1.clone(), v2.clone()));

            let single_key_type: BaseTypes = value_type_single.into();
            let key_type: BaseTypes = value_type_tuple
                .as_ref()
                .map_or(BaseTypes::Null, |(v1, _)| v1.clone().into());
            let value_type: BaseTypes = value_type_tuple
                .as_ref()
                .map_or(BaseTypes::Null, |(_, v2)| v2.clone().into());

            // Match the collection type and call the corresponding parser
            match collection_type.as_str() {
                "array" => parse_array_declaration(expression, single_key_type, name.clone())?,
                "dict" => parse_dict_declaration(expression, key_type, value_type, name.clone())?,
                _ => {
                    return Err(CollectionError::InvalidType(
                        "Unknown collection type".into(),
                    ));
                }
            }

            // Return Ok if parsing was successful
            return Ok(());
        } else {
            return Err(CollectionError::SyntaxError(
                "First node is not a collection".into(),
            ));
        }
    }

    // This point should not be reachable because of the previous checks, but is kept for safety
    Err(CollectionError::SyntaxError(
        "Expression vector is empty or malformed".into(),
    ))
}

fn parse_array_declaration(
    expression: &[ASTNode],
    single_key_type: BaseTypes,
    name: String,
) -> Result<(), CollectionError> {
    let mut values: Vec<BaseTypes> = Vec::new();
    for node in &expression[1..] {
        match node {
            ASTNode::Int(int) => values.push(BaseTypes::Int(int.value)),
            ASTNode::Float(float) => values.push(BaseTypes::Float(float.value.into())),
            ASTNode::String(string) => values.push(BaseTypes::StringWrapper(string.value.clone())),
            ASTNode::Char(char) => values.push(BaseTypes::Char(char.value)),
            ASTNode::Bool(bool) => values.push(BaseTypes::Bool(bool.value)),
            ASTNode::RightBracket => break,
            ASTNode::AssignmentOperator(_) | ASTNode::ArgumentSeparator | ASTNode::LeftBracket => {}
            _ => {
                return Err(CollectionError::InvalidType(format!(
                    "Unexpected node type: {:?}",
                    node
                )))
            }
        }
    }

    let array = Array::new(name, single_key_type, values);
    add_to_array_stack(array);
    Ok(())
}

fn parse_dict_declaration(
    expression: &[ASTNode],
    key_type: BaseTypes,
    value_type: BaseTypes,
    name: String,
) -> Result<(), CollectionError> {
    let mut values: Vec<(BaseTypes, BaseTypes)> = Vec::new();
    let mut key: Option<BaseTypes> = None;
    let mut have_fat_arrow = false;

    for node in &expression[1..] {
        match node {
            ASTNode::Int(int) => handle_key_value(
                BaseTypes::Int(int.value),
                &mut key,
                &mut have_fat_arrow,
                &mut values,
            )?,
            ASTNode::Float(float) => handle_key_value(
                BaseTypes::Float(float.value.into()),
                &mut key,
                &mut have_fat_arrow,
                &mut values,
            )?,
            ASTNode::String(string) => handle_key_value(
                BaseTypes::StringWrapper(string.value.clone()),
                &mut key,
                &mut have_fat_arrow,
                &mut values,
            )?,
            ASTNode::Char(char) => handle_key_value(
                BaseTypes::Char(char.value),
                &mut key,
                &mut have_fat_arrow,
                &mut values,
            )?,
            ASTNode::Bool(bool) => handle_key_value(
                BaseTypes::Bool(bool.value),
                &mut key,
                &mut have_fat_arrow,
                &mut values,
            )?,
            ASTNode::FatArrow => have_fat_arrow = true,
            ASTNode::AssignmentOperator(_)
            | ASTNode::LeftCurly
            | ASTNode::RightCurly
            | ASTNode::ArgumentSeparator => {}
            _ => {
                return Err(CollectionError::InvalidType(format!(
                    "Unexpected node type: {:?}",
                    node
                )))
            }
        }
    }

    let dict = Dictionary::new(name, key_type, value_type, values);
    add_to_dictionary_stack(dict);
    Ok(())
}

fn handle_key_value(
    base_value: BaseTypes,
    key: &mut Option<BaseTypes>,
    have_fat_arrow: &mut bool,
    values: &mut Vec<(BaseTypes, BaseTypes)>,
) -> Result<(), CollectionError> {
    if *have_fat_arrow {
        if let Some(k) = key.take() {
            values.push((k, base_value));
            *have_fat_arrow = false;
            Ok(())
        } else {
            Err(CollectionError::SyntaxError("Missing key for value".into()))
        }
    } else {
        if key.is_some() {
            Err(CollectionError::SyntaxError("Unexpected key".into()))
        } else {
            *key = Some(base_value);
            Ok(())
        }
    }
}
