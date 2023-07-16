use image::*;
use lazy_static::lazy_static;
use crate::functions;

#[derive(Clone)]
enum EFunction {
    Add,
}

#[derive(Clone)]
struct FUNCTION {
    name: String,
    function: EFunction,
    arity: usize,
}

lazy_static! {
    static ref FUNCTION_SET: Vec<FUNCTION> = Vec::from([
        FUNCTION {
            name: "Add".to_string(),
            function: EFunction::Add,
            arity: 4,
        }
    ]);
}

static TERMINAL_SET: [i32; 0] = [];