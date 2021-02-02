use crate::parsing::ast as ast;
use crate::svg_generator as svg;

use std::collections::HashMap;

use anyhow::{Result, anyhow};

pub type ArgsMap = HashMap<String, ast::Value>;

pub type ConstantsMap = HashMap::<String, ast::Value>;

pub struct Function {
    pub eval: Box<dyn Fn(ArgsMap) -> Result<Vec<svg::Element>>>
}

pub type FunMap = HashMap<String, Function>;

pub fn create_arg_map(arg_vec: Vec<ast::Arg>, constants: &ConstantsMap) -> Result<ArgsMap> {
    
    use ast::ConstantOrLiteral::*;

    let mut arg_map = HashMap::new();

    for arg in arg_vec {
        
        let ast::Arg { name: arg_name, value } = arg;

        match value {
            Const(const_name) => {
                match constants.get(&const_name) {
                    Some(val) => { arg_map.insert(arg_name, val.clone()); },
                    None => return Err(anyhow!("Unknown constant {}", &const_name))
                }
            }

            Literal(val) => {
                arg_map.insert(arg_name, val);
            }
        }
    }

    Ok(arg_map)
}