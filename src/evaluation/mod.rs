/// This module takes Zuma AST,
/// evaluates all language constructs like variables, functions, loops, etc.,
/// and returns svg_generator's SVG representation.

mod stdlib;
mod helpers;
use helpers::{ConstantsMap, Constants};

use crate::parsing::ast as ast;
use crate::svg_generator as svg;
use crate::parsing::OperationInput::Literal;
use crate::parsing::OperationInput::Constant;
use crate::parsing::OperationInput::Operation;

use std::collections::HashMap;

use anyhow::{Result, anyhow};

pub fn evaluate(zuma: ast::Document) -> Result<svg::Document> {
    let mut doc = svg::Document::new();
    let constants = vec!();
    doc = handle_expressions(zuma.expressions, doc, &constants)?;
    Ok(doc)
}

fn handle_expressions(expressions: Vec<ast::Expression>,
                      doc: svg::Document,
                      upper_scope_constants: &Constants)
    -> Result<svg::Document>
{
    use crate::parsing::ast::Expression::*;

    let mut local_consts: ConstantsMap = HashMap::new();

    let mut doc = doc;
    for expr in expressions {

        // TODO: I know this is highly suboptimal, but until it starts to cause measurable
        //  performance impact, I am leaving it. Anyway, it would be nice to clean it up.
        let mut mut_upper = upper_scope_constants.clone();
        let mut all_constants = vec!();
        all_constants.push(&local_consts);
        all_constants.append(&mut mut_upper);

        match expr {

            FunctionCall(fc) => {
                let mut res = handle_function_call(fc, &all_constants)?;
                doc = doc.add_many(&mut res);
            },
            
            ConstantDeclaration(c) => {
                match c.value {
                    Literal(value) => { local_consts.insert(c.name, value); },
                    Constant(name) => {},
                    Operation(op) => {},
                }
            },
            
            Scope(s) => {
                doc = handle_expressions(s.expressions, doc, &all_constants)?;
            },
        }       
    }

    Ok(doc)
}

fn handle_function_call(function_call: ast::FunctionCall, consts: &Constants)
    -> Result<Vec<svg::Element>>
{
    
    let ast::FunctionCall { name, args } = function_call;
    
    let stdlib = stdlib::stdlib();

    match stdlib.get(&name) {
        Some(fun) => {
            let args = helpers::create_arg_map(args, &consts)?;
            Ok((fun.eval)(args)?)
        }

        None => Err(anyhow!("Unknown function {}", &name))
    }
}