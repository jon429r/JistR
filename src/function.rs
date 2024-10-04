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

    pub enum FunctionTypes {
        FloatFn(fn(f64)),
        DoubleFloatFn(fn(f64, f64) -> f64),
        SingleFloatFn(fn(f64) -> f64),
        NoArgFloatFn(fn() -> f64),
        StringFn(fn(String)),
        DoubleStringFn(fn(String, String) -> String),
        SingleStringFn(fn(String) -> String),
    }

    use std::any::Any;

    pub fn call_function(func: &FunctionTypes, arguments: Vec<Box<dyn Any>>) -> Box<dyn Any> {
        match func {
            FunctionTypes::DoubleFloatFn(f) => {
                if arguments.len() == 2 {
                    let arg1 = arguments[0].downcast_ref::<f64>().expect("Expected f64");
                    let arg2 = arguments[1].downcast_ref::<f64>().expect("Expected f64");
                    let result = f(*arg1, *arg2);
                    println!("DoubleFn result: {}", result);
                    return Box::new(result);
                } else {
                    panic!("Expected exactly two arguments for DoubleFn");
                }
            }
            FunctionTypes::SingleFloatFn(f) => {
                if arguments.len() == 1 {
                    let arg = arguments[0].downcast_ref::<f64>().expect("Expected f64");
                    let result = f(*arg);
                    println!("SingleFn result: {}", result);
                    return Box::new(result);
                } else {
                    panic!("Expected exactly one argument for SingleFn");
                }
            }
            FunctionTypes::NoArgFloatFn(f) => {
                if arguments.is_empty() {
                    let result = f();
                    println!("NoArgFn result: {}", result);
                    return Box::new(result);
                } else {
                    panic!("Expected no arguments for NoArgFn");
                }
            }
            FunctionTypes::StringFn(f) => {
                if arguments.len() == 1 {
                    let arg = arguments[0]
                        .downcast_ref::<String>()
                        .expect("Expected String");
                    f(arg.clone());
                    println!("StringFn called with: {}", arg);
                    return Box::new(());
                } else {
                    panic!("Expected exactly one argument for StringFn");
                }
            }
            FunctionTypes::DoubleStringFn(f) => {
                if arguments.len() == 2 {
                    let arg1 = arguments[0]
                        .downcast_ref::<String>()
                        .expect("Expected String");
                    let arg2 = arguments[1]
                        .downcast_ref::<String>()
                        .expect("Expected String");
                    let result = f(arg1.clone(), arg2.clone());
                    println!("DoubleStringFn result: {}", result);
                    return Box::new(result);
                } else {
                    panic!("Expected exactly two arguments for DoubleStringFn");
                }
            }
            FunctionTypes::SingleStringFn(f) => {
                if arguments.len() == 1 {
                    let arg = arguments[0]
                        .downcast_ref::<String>()
                        .expect("Expected String");
                    let result = f(arg.clone());
                    println!("SingleStringFn result: {}", result);
                    return Box::new(result);
                } else {
                    panic!("Expected exactly one argument for SingleStringFn");
                }
            }
            _ => {
                panic!("Function not implemented");
            }
        }
    }

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
