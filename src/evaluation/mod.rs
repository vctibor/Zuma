/// This module takes Zuma AST,
/// evaluates all language constructs like variables, functions, loops, etc.,
/// and returns svg_generator's SVG representation.

mod stdlib;
mod helpers;

use crate::parsing::ast as ast;
use crate::svg_generator as svg;

use anyhow::{Result, anyhow};

pub fn evaluate(zuma_doc: ast::Document) -> Result<svg::Document> {

    let mut document = svg::Document::new();

    for row in zuma_doc.rows {

        let function_call: ast::FunctionCall = row;    // later `row` will be enum
        let mut res = handle_function_call(function_call)?;
        document = document.add_many(&mut res);
        
    };

    Ok(document)
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