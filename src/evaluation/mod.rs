/// This module takes Zuma AST,
/// evaluates all language constructs like variables, functions, loops, etc.,
/// and returns svg_generator's SVG representation.

mod stdlib;
mod helpers;

use crate::parsing::ast as ast;
use crate::svg_generator as svg;

use std::collections::HashMap;

use anyhow::{Result, anyhow};

type ConstantsMap = HashMap::<String, ast::Value>;

pub fn evaluate(zuma: ast::Document) -> Result<svg::Document> {
    let mut doc = svg::Document::new();
    doc = handle_expressions(zuma.expressions, doc)?;
    Ok(doc)
}

fn handle_expressions<'a>(expressions: Vec<ast::Expression>, doc: svg::Document) -> Result<svg::Document> {
    use crate::parsing::ast::Expression::*;

    let mut local_consts: ConstantsMap = HashMap::new();

    let mut doc = doc;
    for expr in expressions {
        match expr {

            FunctionCall(fc) => {
                let mut res = handle_function_call(fc)?;
                doc = doc.add_many(&mut res);
            },
            
            ConstantDeclaration(c) => {
                local_consts.insert(c.name, c.value);
            },
            
            Scope(s) => {
                doc = handle_expressions(s.expressions, doc)?;
            },
        }       
    }

    Ok(doc)
}

fn handle_function_call(function_call: ast::FunctionCall) -> Result<Vec<svg::Element>> {
    
    let name = function_call.name.clone();
    
    let stdlib = stdlib::stdlib();

    let function = stdlib.get(&name);

    if function.is_none() {
        return Err(anyhow!("Unknown function {}", &function_call.name));
    }

    let function = function.unwrap();

    (function.eval)(function_call)
}