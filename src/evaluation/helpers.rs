use crate::parsing::ast as ast;
use crate::svg_generator as svg;

use std::collections::HashMap;

use anyhow::{Result, anyhow};

pub type ArgsMap = HashMap<String, ast::Value>;

pub type ConstantsMap = HashMap::<String, ast::Value>;

pub type Constants<'a> = Vec<&'a ConstantsMap>;

pub struct Function {
    pub eval: Box<dyn Fn(ArgsMap) -> Result<Vec<svg::Element>>>
}

pub type FunMap = HashMap<String, Function>;

pub fn create_arg_map(arg_vec: Vec<ast::Arg>, constants: &Constants) -> Result<ArgsMap> {
    let mut arg_map = HashMap::new();
    for arg in arg_vec {
        let ast::Arg { name: arg_name, value } = arg;
        let constant = get_value(&value, constants)?;
        arg_map.insert(arg_name, constant);
    }
    Ok(arg_map)
}

fn get_constant(name: &str, constants: &Constants) -> Result<ast::Value> {
    for const_map in constants {
        if let Some(constant) = const_map.get(name) {
            return Ok(constant.clone())
        }
    }

    Err(anyhow!("Unknown constant {}", &name))
}

fn eval_operation(operation: ast::Operation, constants: &Constants) -> Result<ast::Value> {

    use ast::Operator::*;

    let ast::Operation { lh, op, rh } = operation;

    let left_hand_value: ast::Value = get_value(lh.as_ref(), constants).unwrap();
    let right_hand_value: ast::Value = get_value(rh.as_ref(), constants).unwrap();
    
    match op {
        Addition => {
            panic!("Not yet implemented!");
        },
        Subtraction => {
            panic!("Not yet implemented!");
        },
        Multiplication => {
            panic!("Not yet implemented!");
        },
        Division => {
            panic!("Not yet implemented!");
        },
        LessThan => {
            panic!("Not yet implemented!");
        },
        GreaterThan => {
            panic!("Not yet implemented!");
        },
        Equality => {
            panic!("Not yet implemented!");
        },
        LogicalAnd => {
            panic!("Not yet implemented!");
        },
        LogicalOr => {
            panic!("Not yet implemented!");
        },
        Negation => {
            panic!("Not yet implemented!");
        },
    }
}

fn get_value(input: &ast::OperationInput, constants: &Constants) -> Result<ast::Value> {
    use ast::OperationInput::*;
    match input {
        Literal(value) => Ok(value.clone()),
        Constant(name) => get_constant(&name, constants),
        Operation(op) => eval_operation(op.clone(), constants),
    }
}