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
    
    use ast::OperationInput::*;

    let mut arg_map = HashMap::new();

    for arg in arg_vec {
        
        let ast::Arg { name: arg_name, value } = arg;

        match value {
            Constant(const_name) => {
                match  get_constant(&const_name, &constants) {
                    Some(val) => { arg_map.insert(arg_name, val.clone()); },
                    None => return Err(anyhow!("Unknown constant {}", &const_name))
                }
            }

            Literal(val) => {
                arg_map.insert(arg_name, val);
            }

            Operation(_) => {
                
            }
        }
    }

    Ok(arg_map)
}

fn get_constant(name: &str, const_vec: &Constants) -> Option<ast::Value> {
    for const_map in const_vec {
        if let Some(constant) = const_map.get(name) {
            return Some(constant.clone())
        }
    }

    None
}