/// This module takes Zuma AST,
/// evaluates all language constructs like variables, functions, loops, etc.,
/// and returns svg_generator's SVG representation.

use crate::parsing::ast as ast;
use crate::svg_generator as svg;

use std::collections::HashMap;
use std::boxed::Box;

use maplit::hashmap;
use anyhow::{Result, anyhow};

enum Type {
    Number,
    Point,
    Color
}

struct Function {
    eval: Box<dyn Fn(ast::FunctionCall) -> Result<Vec<svg::Element>>>
}

struct Arg {
    r#type: Type,
    required: bool
}

pub fn evaluate(zuma_doc: ast::Document) -> Result<svg::Document> {

    // declaration of known functions, basically "stdlib"
    let functions: HashMap<String, Function> = hashmap!{
        "line".to_owned() =>  Function {
            eval: Box::new(eval_line)
        }
    };


    let mut document = svg::Document::new();

    for row in zuma_doc.rows {

        let function_call: ast::FunctionCall = row;    // later `row` will be enum
        let mut res = handle_function_call(function_call, &functions)?;
        document = document.add_many(&mut res);
        
    };

    Ok(document)
}

fn handle_function_call(function_call: ast::FunctionCall, functions: &HashMap<String, Function>) -> Result<Vec<svg::Element>> {
    
    let name = function_call.name.clone();
    
    println!("Function {:?}", name);

    let function = functions.get(&name);

    if function.is_none() {
        return Err(anyhow!("Unknown function {}", &function_call.name));
    }

    let function = function.unwrap();

    (function.eval)(function_call)
}

fn eval_line(fc: ast::FunctionCall) -> Result<Vec<svg::Element>> {

    // TODO: This is complete mess, make it better, much better.

    let mut args = fc.args;
    
    let start = get_arg(&mut args, "start")?;
    let end = get_arg(&mut args, "end")?;

    let color: Option<ast::Arg> = get_arg(&mut args, "color").ok();
    let width = get_arg(&mut args, "width").ok();

    if args.len() > 0 {
        return Err(anyhow!("Unexpected argument provided."));
    }    
    
    let mut s = None;
    let mut e = None;
    let mut c = None;
    let mut w = None;

    if let ast::Literal::Point(start) = start.value {
        s = Some(start);
    }
    else
    {
        return Err(anyhow!("Wrong type."));
    }

    if let ast::Literal::Point(end) = end.value {
        e = Some(end);
    }
    else
    {
        return Err(anyhow!("Wrong type."));
    }

    if color.is_some() {
        if let ast::Literal::Color(color) = color.unwrap().value {
            c = Some(color);
        }
    }

    if width.is_some() {
        if let ast::Literal::Number(width) = width.unwrap().value {
            w = Some(width);
        }
    }

    let start = s.unwrap();
    let end = e.unwrap();

    let color = c.unwrap_or(ast::Color { red: 0, green: 0, blue: 0 });
    let width = w.unwrap_or(1.0);

    let line: svg::Element = svg::Line::new(start.x, start.y, end.x, end.y)
        .color(color.red, color.green, color.blue)
        .width(width)
        .into();

    Ok(vec!(line))
}

fn get_arg(args: &mut Vec<ast::Arg>, name: &str) -> Result<ast::Arg> {

    let index = args.iter().position(|arg| arg.name == name);

    if index.is_none() {
        return Err(anyhow!("Missing required argument `start`."));
    }
    
    let index = index.unwrap();

    let arg = args.remove(index);

    Ok(arg)
}