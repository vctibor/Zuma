use super::helpers::*;

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
        "line".to_owned() => Function {
            eval: Box::new(line)
        },

        "rectangle".to_owned() => Function {
            eval: Box::new(rectangle)
        },

        "text".to_owned() => Function {
            eval: Box::new(text)
        },
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

fn rectangle(fc: ast::FunctionCall) -> Result<Vec<svg::Element>> {

    let mut args = fc.args;

    let start = get_arg(&mut args, "start")?;
    let start = get_point(start)?;

    let size = get_arg(&mut args, "size")?;
    let size = get_point(size)?;

    let color: Option<ast::Arg> = get_arg(&mut args, "color").ok();
    let color = if let Some(color) = color {
        get_color(color)?
    } else {
        ast::Color { red: 0, green: 0, blue: 0 } 
    };

    let stroke_color: Option<ast::Arg> = get_arg(&mut args, "stroke-color").ok();
    let stroke_color = if let Some(stroke_color) = stroke_color {
        get_color(stroke_color)?
    } else {
        ast::Color { red: 0, green: 0, blue: 0 } 
    };

    let stroke_width: Option<ast::Arg> = get_arg(&mut args, "stroke-width").ok();
    let stroke_width = if let Some(stroke_width) = stroke_width {
        get_number(stroke_width)?
    } else {
        1.0
    };
    
    let opacity: Option<ast::Arg> = get_arg(&mut args, "opacity").ok();
    let opacity = if let Some(opacity) = opacity {
        get_number(opacity)?
    } else {
        1.0
    };

    if args.len() > 0 {
        return Err(anyhow!("Unexpected argument provided."));
    }

    let rectangle = svg::Rectangle::new(start.x, start.y, size.x, size.y)
        .stroke_width(stroke_width)
        .stroke_color(stroke_color.red, stroke_color.green, stroke_color.blue)
        .fill_color(color.red, color.green, color.blue)
        .opacity(opacity)
        .into();

    Ok(vec!(rectangle))
}

fn text(fc: ast::FunctionCall) -> Result<Vec<svg::Element>> {
    let mut args = fc.args;

    let start = get_arg(&mut args, "start")?;
    let start = get_point(start)?;

    let content = get_arg(&mut args, "text")?;
    let content = get_string(content)?;
    let mut content = remove_first(&content).to_owned();
    content.pop();

    let color: Option<ast::Arg> = get_arg(&mut args, "color").ok();
    let color = if let Some(color) = color {
        get_color(color)?
    } else {
        ast::Color { red: 0, green: 0, blue: 0 } 
    };

    if args.len() > 0 {
        println!("{:?}", args);
        return Err(anyhow!("Unexpected argument provided."));
    }

    let text = svg::Text::new(start.x, start.y, content)
        .fill_color(color.red, color.green, color.blue)
        .into();

    Ok(vec!(text))
}