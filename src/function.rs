use crate::function::functions::Function;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref FUNCTION_STACK: Mutex<Vec<Function>> = Mutex::new(Vec::new());
}

pub mod functions {

    use crate::base_variable::base_types::BaseTypes;
    use crate::node::nodes::ASTNode;
    use crate::{base_variable::variable::Variable, token_type::token_types::TokenTypes};
    use std::fmt;

    #[derive(Clone, Debug)]
    pub struct Function {
        pub name: String,
        pub return_type: BaseTypes,
        pub arguments: Vec<Variable>,
        pub body: Vec<ASTNode>,
    }

    // Constructor for creating a new function
    impl Function {
        pub fn new(
            name: String,
            return_type: BaseTypes,
            arguments: Vec<Variable>,
            body: Vec<ASTNode>,
        ) -> Function {
            Function {
                name,
                return_type,
                arguments,
                body,
            }
        }
    }

    // Implementing Display trait for Function
    impl fmt::Display for Function {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Formatting the arguments and body for proper display
            let arguments = self
                .arguments
                .iter()
                .map(|arg| format!("{}", arg)) // Assuming Variable implements Display
                .collect::<Vec<String>>()
                .join(", ");

            let body = self
                .body
                .iter()
                .map(|token| format!("{:?}", token)) // Assuming TokenTypes implements Display
                .collect::<Vec<String>>()
                .join("\n");

            write!(
                f,
                "func {}({}) -> {}\n{}",
                self.name, arguments, self.return_type, body
            )
        }
    }

    impl Function {
        // Function to call the function
        pub fn call(&mut self) {
            // Logic for calling the function can be implemented here
        }
    }
}
