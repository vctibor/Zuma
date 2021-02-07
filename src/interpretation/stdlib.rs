use super::helpers::*;
use super::graphics::GraphicNode;

use std::boxed::Box;

use maplit::hashmap;
use anyhow::{Result, anyhow};

/// Declaration of known functions, basically "stdlib".
/// I'd like to move this into something like OnceCell.
pub fn stdlib() -> FunMap {
    hashmap!{
        "line".to_owned() => Function { eval: Box::new(line) },
        "rectangle".to_owned() => Function { eval: Box::new(rectangle) },
        "text".to_owned() => Function { eval: Box::new(text) },
        "ellipse".to_owned() => Function { eval: Box::new(ellipse) },
    }
}

fn line(mut args: ArgsMap, constants: &Constants) -> Result<Vec<GraphicNode>> {

    let start = args.remove("start")
                    .ok_or(anyhow!("Missing argument `start`"))?
                    .get_point()?;

    let end = args.remove("end")
                  .ok_or(anyhow!("Missing argument `end`"))?
                  .get_point()?;

    let color = args.remove("color")
                    .map(|x| x.get_color().ok())
                    .flatten()
                    .map(|x| get_color(x, constants).ok())
                    .flatten()
                    .unwrap_or((0, 0, 0));

    let width = args.remove("width")
                    .map(|x| x.get_number().ok())
                    .flatten()
                    .unwrap_or(1.0)
                    .to_string();
    
    if args.len() > 0 {
        return Err(anyhow!("Unexpected argument provided."));
    }

    let start_x = get_value(start.x.as_ref(), &constants)?.get_number()?.to_string();
    let start_y = get_value(start.y.as_ref(), &constants)?.get_number()?.to_string();
    let end_x = get_value(end.x.as_ref(), &constants)?.get_number()?.to_string();
    let end_y = get_value(end.y.as_ref(), &constants)?.get_number()?.to_string();

    let color = color_to_string(color);

    Ok(vec!(
        GraphicNode::tag("line")
            .insert("x1", start_x)
            .insert("y1", start_y)
            .insert("x2", end_x)
            .insert("y2", end_y)
            .insert("stroke", color)
            .insert("stroke-width", width)
    ))
}

fn rectangle(mut args: ArgsMap, constants: &Constants) -> Result<Vec<GraphicNode>> {

    let start = args.remove("start")
                    .ok_or(anyhow!("Missing argument `start`"))?
                    .get_point()?;

    let size = args.remove("size")
                   .ok_or(anyhow!("Missing argument `size`"))?
                   .get_point()?;

    let color = args.remove("color")
                   .map(|x| x.get_color().ok())
                   .flatten()
                   .map(|x| get_color(x, constants).ok())
                   .flatten()
                   .unwrap_or((0, 0, 0));

    let stroke_color = args.remove("stroke-color")
                   .map(|x| x.get_color().ok())
                   .flatten()
                   .map(|x| get_color(x, constants).ok())
                   .flatten()
                   .unwrap_or((0, 0, 0));

    let stroke_width = args.remove("stroke-width")
                           .map(|x| x.get_number().ok())
                           .flatten()
                           .unwrap_or(1.0)
                           .to_string();

    let opacity = args.remove("opacity")
                           .map(|x| x.get_number().ok())
                           .flatten()
                           .unwrap_or(1.0)
                           .to_string();
    
    if args.len() > 0 {
        return Err(anyhow!("Unexpected argument provided."));
    }

    let start_x = get_value(start.x.as_ref(), &constants)?.get_number()?.to_string();
    let start_y = get_value(start.y.as_ref(), &constants)?.get_number()?.to_string();
    let size_x = get_value(size.x.as_ref(), &constants)?.get_number()?.to_string();
    let size_y = get_value(size.y.as_ref(), &constants)?.get_number()?.to_string();

    let color = color_to_string(color);
    let stroke_color = color_to_string(stroke_color);

    Ok(vec!(
        GraphicNode::tag("rect")
            .insert("x", start_x)
            .insert("y", start_y)
            .insert("width", size_x)
            .insert("height", size_y)
            .insert("fill", color)
            .insert("stroke", stroke_color)
            .insert("stroke-width", stroke_width)
            .insert("opacity", opacity)
    ))
}

fn text(mut args: ArgsMap, constants: &Constants) -> Result<Vec<GraphicNode>> {

    let start = args.remove("start")
                    .ok_or(anyhow!("Missing argument `start`"))?
                    .get_point()?;

    let content = args.remove("text")
                    .ok_or(anyhow!("Missing argument `text`"))?
                    .get_string()?;

    let color = args.remove("color")
                    .map(|x| x.get_color().ok())
                    .flatten()
                    .map(|x| get_color(x, constants).ok())
                    .flatten()
                    .unwrap_or((0, 0, 0));

    if args.len() > 0 {
        return Err(anyhow!("Unexpected argument provided."));
    }

    let start_x = get_value(start.x.as_ref(), &constants)?.get_number()?.to_string();
    let start_y = get_value(start.y.as_ref(), &constants)?.get_number()?.to_string();

    let color = color_to_string(color);

    Ok(vec!(
        GraphicNode::element("text", &content)
            .insert("x", start_x)
            .insert("y", start_y)
            .insert("fill", color)
    ))
}

fn ellipse(mut args: ArgsMap, constants: &Constants) -> Result<Vec<GraphicNode>> {

    let center = args.remove("center")
                    .ok_or(anyhow!("Missing argument `center`"))?
                    .get_point()?;

    let radius = args.remove("radius")
                    .ok_or(anyhow!("Missing argument `radius`"))?
                    .get_point()?;

    let color = args.remove("color")
                   .map(|x| x.get_color().ok())
                   .flatten()
                   .map(|x| get_color(x, constants).ok())
                   .flatten()
                   .unwrap_or((0, 0, 0));

    let stroke_color = args.remove("stroke-color")
                   .map(|x| x.get_color().ok())
                   .flatten()
                   .map(|x| get_color(x, constants).ok())
                   .flatten()
                   .unwrap_or((0, 0, 0));

    let stroke_width = args.remove("stroke-width")
                           .map(|x| x.get_number().ok())
                           .flatten()
                           .unwrap_or(1.0)
                           .to_string();

    let opacity = args.remove("opacity")
                           .map(|x| x.get_number().ok())
                           .flatten()
                           .unwrap_or(1.0)
                           .to_string();

    if args.len() > 0 {
        return Err(anyhow!("Unexpected argument provided."));
    }

    let center_x = get_value(center.x.as_ref(), &constants)?.get_number()?.to_string();
    let center_y = get_value(center.y.as_ref(), &constants)?.get_number()?.to_string();
    let radius_x = get_value(radius.x.as_ref(), &constants)?.get_number()?.to_string();
    let radius_y = get_value(radius.y.as_ref(), &constants)?.get_number()?.to_string();

    let color = color_to_string(color);
    let stroke_color = color_to_string(stroke_color);

    Ok(vec!(
        GraphicNode::tag("ellipse")
            .insert("cx", center_x)
            .insert("cy", center_y)
            .insert("rx", radius_x)
            .insert("ry", radius_y)
            .insert("fill", color)
            .insert("stroke", stroke_color)
            .insert("stroke-width", stroke_width)
            .insert("opacity", opacity)
    ))
}