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

    lazy_static::lazy_static! {
    pub static ref FUNCTIONS: Mutex<HashMap<&'static str, FunctionType>> = {
        let mut map = HashMap::new();
        map.insert("max", FunctionType::DoubleFn(FunctionMap::max as fn(f64, f64) -> f64));
        map.insert("min", FunctionType::DoubleFn(FunctionMap::min as fn(f64, f64) -> f64));
        map.insert("add", FunctionType::DoubleFn(FunctionMap::add as fn(f64, f64) -> f64));
        map.insert("sub", FunctionType::DoubleFn(FunctionMap::sub as fn(f64, f64) -> f64));
        map.insert("mult", FunctionType::DoubleFn(FunctionMap::mult as fn(f64, f64) -> f64));
        map.insert("divide", FunctionType::DoubleFn(FunctionMap::divide as fn(f64, f64) -> f64));
        map.insert("floor", FunctionType::SingleFn(FunctionMap::floor as fn(f64) -> f64));
        map.insert("ceil", FunctionType::SingleFn(FunctionMap::ceil as fn(f64) -> f64));
        map.insert("round", FunctionType::SingleFn(FunctionMap::round as fn(f64) -> f64));
        map.insert("rand", FunctionType::NoArgFn(FunctionMap::rand as fn() -> f64));
        map.insert("echo", FunctionType::StringFn(FunctionMap::echo as fn(String)));
        map.insert("abs", FunctionType::SingleFn(FunctionMap::abs as fn(f64) -> f64));
        map.insert("pow", FunctionType::DoubleFn(FunctionMap::pow as fn(f64, f64) -> f64));
        map.insert("sqrt", FunctionType::SingleFn(FunctionMap::sqrt as fn(f64) -> f64));
        map.insert("log", FunctionType::DoubleFn(FunctionMap::log as fn(f64, f64) -> f64));
        map.insert("sin", FunctionType::SingleFn(FunctionMap::sin as fn(f64) -> f64));
        map.insert("cos", FunctionType::SingleFn(FunctionMap::cos as fn(f64) -> f64));
        map.insert("tan", FunctionType::SingleFn(FunctionMap::tan as fn(f64) -> f64));
        map.insert("concat", FunctionType::DoubleFn(FunctionMap::concat as fn(String, String) -> String));
        map.insert("len", FunctionType::SingleFn(FunctionMap::len as fn(String) -> usize));
        map.insert("to_uppercase", FunctionType::SingleFn(FunctionMap::to_uppercase as fn(String) -> String));
        map.insert("to_lowercase", FunctionType::SingleFn(FunctionMap::to_lowercase as fn(String) -> String));
        map.insert("trim", FunctionType::SingleFn(FunctionMap::trim as fn(String) -> String));
        map.into()
    };
}

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
    Abs,
    Pow,
    Sqrt,
    Log,
    Sin,
    Cos,
    Tan,
    Concat,
    Len,
    ToUppercase,
    ToLowercase,
    Trim,
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

impl FunctionMap {
    fn abs(a: f64) -> f64 {
        a.abs()
    }

    fn pow(a: f64, b: f64) -> f64 {
        a.powf(b)
    }

    fn sqrt(a: f64) -> f64 {
        a.sqrt()
    }

    fn log(a: f64, base: f64) -> f64 {
        a.log(base)
    }

    fn sin(a: f64) -> f64 {
        a.sin()
    }

    fn cos(a: f64) -> f64 {
        a.cos()
    }

    fn tan(a: f64) -> f64 {
        a.tan()
    }

    fn concat(a: String, b: String) -> String {
        [a, b].concat()
    }

    fn len(s: String) -> usize {
        s.len()
    }

    fn to_uppercase(s: String) -> String {
        s.to_uppercase()
    }

    fn to_lowercase(s: String) -> String {
        s.to_lowercase()
    }

    fn trim(s: String) -> String {
        s.trim().to_string()
    }
}
