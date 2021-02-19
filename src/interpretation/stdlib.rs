use super::helpers::*;
use super::graphics::{GraphicNode, ElementContent};

use anyhow::{Result, anyhow};

pub fn line(mut args: ArgsMap, constants: &Constants) -> Result<Vec<GraphicNode>> {

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
        GraphicNode::empty_element("line")
            .add_attr("x1", &start_x)
            .add_attr("y1", &start_y)
            .add_attr("x2", &end_x)
            .add_attr("y2", &end_y)
            .add_attr("stroke", &color)
            .add_attr("stroke-width", &width)
    ))
}

pub fn rectangle(mut args: ArgsMap, constants: &Constants) -> Result<Vec<GraphicNode>> {

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

    let round_corners = args.remove("round-corners")
                           .map(|x| x.get_number().ok())
                           .flatten()
                           .unwrap_or(0.0)
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
    // let round_corners = get_value(round_corners, &constants)?.get_number()?.to_string();

    Ok(vec!(
        GraphicNode::empty_element("rect")
            .add_attr("x", &start_x)
            .add_attr("y", &start_y)
            .add_attr("width", &size_x)
            .add_attr("height", &size_y)
            .add_attr("fill", &color)
            .add_attr("stroke", &stroke_color)
            .add_attr("stroke-width", &stroke_width)
            .add_attr("opacity", &opacity)
            .add_attr("rx", &round_corners)
            .add_attr("ry", &round_corners)
    ))
}

pub fn text(mut args: ArgsMap, constants: &Constants) -> Result<Vec<GraphicNode>> {

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
        GraphicNode::element("text", ElementContent::from_str(&content))
            .add_attr("x", &start_x)
            .add_attr("y", &start_y)
            .add_attr("fill", &color)
    ))
}

pub fn ellipse(mut args: ArgsMap, constants: &Constants) -> Result<Vec<GraphicNode>> {

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
        GraphicNode::empty_element("ellipse")
            .add_attr("cx", &center_x)
            .add_attr("cy", &center_y)
            .add_attr("rx", &radius_x)
            .add_attr("ry", &radius_y)
            .add_attr("fill", &color)
            .add_attr("stroke", &stroke_color)
            .add_attr("stroke-width", &stroke_width)
            .add_attr("opacity", &opacity)
    ))
}