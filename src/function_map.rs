pub mod functions {
    use crate::function::functions::Function;

    pub static mut FUNCTIONSTACK: Vec<Function> = Vec::new();
}

use crate::base_variable::base_types::BaseTypes;
use crate::base_variable::variable::Variable;
use crate::function::functions::Function;

use crate::function::functions::FunctionTypes;
use crate::node::nodes::ASTNode;

use std::collections::HashMap;
use std::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref USER_FUNCTIONS: Mutex<HashMap<String, Function>> = {
        let map = HashMap::new();
        map.into()
    };

    pub static ref FUNCTIONS: Mutex<HashMap<&'static str, FunctionTypes>> = {
        let mut map = HashMap::new();
        map.insert("max", FunctionTypes::DoubleFloatFn(FunctionMap::max as fn(f64, f64) -> f64));
        map.insert("min", FunctionTypes::DoubleFloatFn(FunctionMap::min as fn(f64, f64) -> f64));
        map.insert("add", FunctionTypes::DoubleFloatFn(FunctionMap::add as fn(f64, f64) -> f64));
        map.insert("sub", FunctionTypes::DoubleFloatFn(FunctionMap::sub as fn(f64, f64) -> f64));
        map.insert("mult", FunctionTypes::DoubleFloatFn(FunctionMap::mult as fn(f64, f64) -> f64));
        map.insert("divide", FunctionTypes::DoubleFloatFn(FunctionMap::divide as fn(f64, f64) -> f64));
        map.insert("floor", FunctionTypes::SingleFloatFn(FunctionMap::floor as fn(f64) -> f64));
        map.insert("ceil", FunctionTypes::SingleFloatFn(FunctionMap::ceil as fn(f64) -> f64));
        map.insert("round", FunctionTypes::SingleFloatFn(FunctionMap::round as fn(f64) -> f64));
        map.insert("rand", FunctionTypes::NoArgFloatFn(FunctionMap::rand as fn() -> f64));
        map.insert("echo", FunctionTypes::EchoFn(FunctionMap::echo as fn(String)));
        map.insert("echoln", FunctionTypes::EchoFn(FunctionMap::echoln as fn(String)));
        map.insert("abs", FunctionTypes::SingleFloatFn(FunctionMap::abs as fn(f64) -> f64));
        map.insert("pow", FunctionTypes::DoubleFloatFn(FunctionMap::pow as fn(f64, f64) -> f64));
        map.insert("sqrt", FunctionTypes::SingleFloatFn(FunctionMap::sqrt as fn(f64) -> f64));
        map.insert("log", FunctionTypes::DoubleFloatFn(FunctionMap::log as fn(f64, f64) -> f64));
        map.insert("sin", FunctionTypes::SingleFloatFn(FunctionMap::sin as fn(f64) -> f64));
        map.insert("cos", FunctionTypes::SingleFloatFn(FunctionMap::cos as fn(f64) -> f64));
        map.insert("tan", FunctionTypes::SingleFloatFn(FunctionMap::tan as fn(f64) -> f64));
        map.insert("concat", FunctionTypes::DoubleStringFn(FunctionMap::concat as fn(String, String) -> String));
        //map.insert("len", FunctionTypes::SingleFn(FunctionMap::len as fn(String) -> usize));
        map.insert("to_uppercase", FunctionTypes::SingleStringFn(FunctionMap::to_uppercase as fn(String) -> String));
        map.insert("to_lowercase", FunctionTypes::SingleStringFn(FunctionMap::to_lowercase as fn(String) -> String));
        map.insert("trim", FunctionTypes::SingleStringFn(FunctionMap::trim as fn(String) -> String));
        map.insert("input", FunctionTypes::SingleStringFn(FunctionMap::input as fn(String) -> String));
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
    Input,
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
        print!("{}", a);
        //println!("After echo");
    }

    fn echoln(a: String) {
        println!("{}", a);
    }

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

    fn input(s: String) -> String {
        print!("{}", s);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        return input;
    }
}
