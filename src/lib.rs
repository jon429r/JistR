pub mod ast;
pub mod base_variable;
pub mod collection;
pub mod compiler;
pub mod function_map;
pub mod node;
pub mod token_type;
pub mod statement_tokenizer {
    pub mod basic_tokenizer;
    pub mod collection_tokenizer;
    pub mod function_tokenizer;
    pub mod tests;
    pub mod tokenizer;
    pub mod variable_tokenizer;
}

pub mod compilers {
    pub mod function;
    pub mod operation;
    pub mod variable;
}
