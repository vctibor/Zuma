use crate::parsing::ast as ast;
use crate::code_generation as svg;

use std::collections::HashMap;

use anyhow::{Result, anyhow};

pub type ArgsMap = HashMap<String, ast::Value>;

pub type ConstantsMap = HashMap::<String, ast::Value>;

pub type Constants<'a> = Vec<&'a ConstantsMap>;

pub struct Function {
    pub eval: Box<dyn Fn(ArgsMap, &Constants) -> Result<Vec<svg::Element>>>
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

pub fn get_constant(name: &str, constants: &Constants) -> Result<ast::Value> {
    for const_map in constants {
        if let Some(constant) = const_map.get(name) {
            return Ok(constant.clone())
        }
    }

    Err(anyhow!("Unknown constant {}", &name))
}

pub fn eval_operation(operation: ast::Operation, constants: &Constants) -> Result<ast::Value> {

    use ast::Operator::*;

    let ast::Operation { lh, op, rh } = operation;

    let lh: ast::Value = get_value(lh.as_ref(), constants)?;
    let rh: ast::Value = get_value(rh.as_ref(), constants)?;
    
    match op {
        Addition => {
            let lh = lh.get_number()?;
            let rh = rh.get_number()?;
            return Ok(ast::Value::Number(lh + rh));
        },
        Subtraction => {
            let lh = lh.get_number()?;
            let rh = rh.get_number()?;
            return Ok(ast::Value::Number(lh - rh));
        },
        Multiplication => {
            let lh = lh.get_number()?;
            let rh = rh.get_number()?;
            return Ok(ast::Value::Number(lh * rh));
        },
        Division => {
            let lh = lh.get_number()?;
            let rh = rh.get_number()?;
            
            if rh == 0.0 {
                return Err(anyhow!("Division by zero!"));
            }

            return Ok(ast::Value::Number(lh / rh));
        },
        LessThan => {
            let lh = lh.get_number()?;
            let rh = rh.get_number()?;
            return Ok(ast::Value::Bool(lh < rh));
        },
        GreaterThan => {
            let lh = lh.get_number()?;
            let rh = rh.get_number()?;
            return Ok(ast::Value::Bool(lh > rh));
        },
        Equality => {
            panic!("Not yet implemented!");
        },
        LogicalAnd => {
            let lh = lh.get_bool()?;
            let rh = rh.get_bool()?;
            return Ok(ast::Value::Bool(lh && rh));
        },
        LogicalOr => {
            let lh = lh.get_bool()?;
            let rh = rh.get_bool()?;
            return Ok(ast::Value::Bool(lh | rh));
        }
    }
}

pub fn get_value(input: &ast::OperationInput, constants: &Constants) -> Result<ast::Value> {
    use ast::OperationInput::*;
    match input {
        Literal(value) => Ok(value.clone()),
        Constant(name) => get_constant(&name, constants),
        Operation(op) => eval_operation(op.clone(), constants),
    }
}

pub fn get_color(color: ast::Color, constants: &Constants) -> Result<(u8, u8, u8)> {
    let red = get_value(&color.red, constants)?.get_number()? as u8;
    let green = get_value(&color.green, constants)?.get_number()? as u8;
    let blue = get_value(&color.blue, constants)?.get_number()? as u8;
    Ok((red, green, blue))
}