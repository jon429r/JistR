pub mod functions {
    use crate::function_map::function::Function;

    pub static mut FUNCTIONSTACK: Vec<Function> = Vec::new();
}

pub mod function {
    use crate::base_variable::base_types::BaseTypes;
    use crate::base_variable::variable::Variable;
    use crate::node::nodes::ASTNode;

    pub struct Function {
        pub name: String,
        pub arguments: Vec<Variable>,
        pub return_type: BaseTypes,
        pub body: Vec<ASTNode>,
    }

    impl Function {
        pub fn new(
            name: String,
            arguments: Vec<Variable>,
            return_type: BaseTypes,
            body: Vec<ASTNode>,
        ) -> Function {
            Function {
                name,
                arguments,
                return_type,
                body,
            }
        }
    }
}

use std::collections::HashMap;
use std::sync::Mutex;

use function::Function; // For thread safety

lazy_static::lazy_static! {
    pub static ref USER_FUNCTIONS: Mutex<HashMap<String, Function>> = {
        let map = HashMap::new();
        map.into()
    };

    pub static ref STD_FUNCTIONS_DOUBLE: Mutex<HashMap<&'static str, fn(f64, f64) -> f64>> = {
        let mut map = HashMap::new();
        map.insert("max", FunctionMap::max as fn(f64, f64) -> f64);
        map.insert("min", FunctionMap::min as fn(f64, f64) -> f64);
        map.insert("add", FunctionMap::add as fn(f64, f64) -> f64);
        map.insert("sub", FunctionMap::sub as fn(f64, f64) -> f64);
        map.insert("mult", FunctionMap::mult as fn(f64, f64) -> f64);
        map.insert("divide", FunctionMap::divide as fn(f64, f64) -> f64);
        map.into()
    };

    pub static ref STD_FUNCTIONS_SINGLE: Mutex<HashMap<&'static str, fn(f64) -> f64>> = {
        let mut map = HashMap::new();
        map.insert("floor", FunctionMap::floor as fn(f64) -> f64);
        map.insert("ceil", FunctionMap::ceil as fn(f64) -> f64);
        map.insert("round", FunctionMap::round as fn(f64) -> f64);
        map.into()
    };

    pub static ref STD_FUNCTIONS: Mutex<HashMap<&'static str, fn() -> f64>> = {
        let mut map = HashMap::new();
        map.insert("rand", FunctionMap::rand as fn() -> f64);
        map.into()
    };

    pub static ref STD_FUNCTIONS_ECHO: Mutex<HashMap<&'static str, fn(String)>> = {
        let mut map = HashMap::new();
        map.insert("echo", FunctionMap::echo as fn(String));
        map.into()
    };
}

enum FunctionMap {
    Max,
    Min,
    Rand,
    Floor,
    Ceil,
    Round,
    Add,
    Sub,
    Mult,
    Divide,
    Echo,
}

impl FunctionMap {
    fn max(a: f64, b: f64) -> f64 {
        if a > b {
            a
        } else {
            b
        }
    }

    fn min(a: f64, b: f64) -> f64 {
        if a < b {
            a
        } else {
            b
        }
    }

    fn rand() -> f64 {
        rand::random::<f64>()
    }

    fn floor(a: f64) -> f64 {
        a.floor()
    }

    fn ceil(a: f64) -> f64 {
        a.ceil()
    }

    fn round(a: f64) -> f64 {
        a.round()
    }

    fn add(a: f64, b: f64) -> f64 {
        a + b
    }

    fn sub(a: f64, b: f64) -> f64 {
        a - b
    }

    fn mult(a: f64, b: f64) -> f64 {
        a * b
    }

    fn divide(a: f64, b: f64) -> f64 {
        a / b
    }

    fn echo(a: String) {
        print!("String: {:?}", a);
        //println!("After echo");
    }
}
