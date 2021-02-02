use super::helpers::*;

use crate::parsing::ast as ast;
use crate::svg_generator as svg;

use std::boxed::Box;

use maplit::hashmap;
use anyhow::{Result, anyhow};

// Declaration of known functions, basically "stdlib".
// I'd like to move this into something like OnceCell.
pub fn stdlib() -> FunMap {
    hashmap!{
        "line".to_owned() => Function { eval: Box::new(line) },
        "rectangle".to_owned() => Function { eval: Box::new(rectangle) },
        "text".to_owned() => Function { eval: Box::new(text) },
    }
}

fn line(mut args: ArgsMap) -> Result<Vec<svg::Element>> {

    let start = args.remove("start")
                    .ok_or(anyhow!("Missing argument `start`"))?
                    .get_point()?;

    let end = args.remove("end")
                  .ok_or(anyhow!("Missing argument `end`"))?
                  .get_point()?;

    let color = args.remove("color")
                    .map(|x| x.get_color().ok())
                    .flatten()
                    .unwrap_or(ast::Color { red: 0, green: 0, blue: 0 });

    let width = args.remove("width")
                    .map(|x| x.get_number().ok())
                    .flatten()
                    .unwrap_or(1.0);
    
    if args.len() > 0 {
        return Err(anyhow!("Unexpected argument provided."));
    }

    let line: svg::Element = svg::Line::new(start.x, start.y, end.x, end.y)
        .color(color.red, color.green, color.blue)
        .width(width)
        .into();

    Ok(vec!(line))
}

fn rectangle(mut args: ArgsMap) -> Result<Vec<svg::Element>> {

    let start = args.remove("start")
                    .ok_or(anyhow!("Missing argument `start`"))?
                    .get_point()?;

    let size = args.remove("size")
                   .ok_or(anyhow!("Missing argument `size`"))?
                   .get_point()?;

    let color = args.remove("color")
                    .map(|x| x.get_color().ok())
                    .flatten()
                    .unwrap_or(ast::Color { red: 0, green: 0, blue: 0 });

    let stroke_color = args.remove("stroke-color")
                           .map(|x| x.get_color().ok())
                           .flatten()
                           .unwrap_or(ast::Color { red: 0, green: 0, blue: 0 });

    let stroke_width = args.remove("stroke-width")
                           .map(|x| x.get_number().ok())
                           .flatten()
                           .unwrap_or(1.0);

    let opacity = args.remove("opacity")
                           .map(|x| x.get_number().ok())
                           .flatten()
                           .unwrap_or(1.0);
    
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

fn text(mut args: ArgsMap) -> Result<Vec<svg::Element>> {

    let start = args.remove("start")
                    .ok_or(anyhow!("Missing argument `start`"))?
                    .get_point()?;

    let content = args.remove("start")
                    .ok_or(anyhow!("Missing argument `content`"))?
                    .get_string()?;

    let color = args.remove("color")
                    .map(|x| x.get_color().ok())
                    .flatten()
                    .unwrap_or(ast::Color { red: 0, green: 0, blue: 0 });

    if args.len() > 0 {
        return Err(anyhow!("Unexpected argument provided."));
    }

    let text = svg::Text::new(start.x, start.y, content)
        .fill_color(color.red, color.green, color.blue)
        .into();

    Ok(vec!(text))
}