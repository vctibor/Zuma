use crate::parsing::ast as ast;
use crate::svg_generator as svg;

use std::collections::HashMap;
use std::boxed::Box;

use maplit::hashmap;
use anyhow::{Result, anyhow};


pub struct Function {
    pub eval: Box<dyn Fn(ast::FunctionCall) -> Result<Vec<svg::Element>>>
}

pub type FunMap = HashMap<String, Function>;

// Declaration of known functions, basically "stdlib".
// I'd like to move this into something like OnceCell.
pub fn stdlib() -> FunMap {
    hashmap!{
        "line".to_owned() =>  Function {
            eval: Box::new(line)
        }
    }
}

fn line(fc: ast::FunctionCall) -> Result<Vec<svg::Element>> {

    let mut args = fc.args;
    
    // Required args
    let start: ast::Arg = get_arg(&mut args, "start")?;
    let end: ast::Arg = get_arg(&mut args, "end")?;

    // Optional args
    let color: Option<ast::Arg> = get_arg(&mut args, "color").ok();
    let width: Option<ast::Arg> = get_arg(&mut args, "width").ok();

    // Fail if unknown argument is passed
    if args.len() > 0 {
        return Err(anyhow!("Unexpected argument provided."));
    }

    // Perform type checks and extract values
    let start = get_point(start)?;
    let end = get_point(end)?;

    // Type checks on optional args
    let color = if let Some(color) = color {
        get_color(color)?
    } else {
        ast::Color { red: 0, green: 0, blue: 0 } 
    };

    let width = if let Some(width) = width {
        get_number(width)?
    } else {
        1.0
    };

    // Result of function - construct line from provided args
    let line: svg::Element = svg::Line::new(start.x, start.y, end.x, end.y)
        .color(color.red, color.green, color.blue)
        .width(width)
        .into();

    Ok(vec!(line))
}

fn get_arg(args: &mut Vec<ast::Arg>, name: &str) -> Result<ast::Arg> {

    let index = args.iter().position(|arg| arg.name == name);

    if let Some(index) = index {
        return Ok(args.remove(index));
    }

    Err(anyhow!("Missing required argument `{}`.", name))
}


fn get_point(arg: ast::Arg) -> Result<ast::Point> {
    match arg.value {
        ast::Literal::Point(p) => Ok(p),
        _ => Err(anyhow!("Wrong type."))
    }
}

fn get_color(arg: ast::Arg) -> Result<ast::Color> {
    match arg.value {
        ast::Literal::Color(c) => Ok(c),
        _ => Err(anyhow!("Wrong type."))
    }
}

fn get_number(arg: ast::Arg) -> Result<f32> {
    match arg.value {
        ast::Literal::Number(n) => Ok(n),
        _ => Err(anyhow!("Wrong type."))
    }
}