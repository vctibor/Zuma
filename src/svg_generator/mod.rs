pub mod svg_model;

mod tests;

use crate::svg_generator::svg_model::*;

pub fn generate_svg(doc: Document) -> String {
    use crate::svg_generator::Element::*;

    let mut svg = "<svg xmlns=\"http://www.w3.org/2000/svg\">\n".to_owned();

    for el in doc.elements {
        match el {
            Line(l) => svg.push_str(&line(l))
        }
    }

    svg.push_str("</svg>");

    svg
}

fn line(l: Line) -> String {
    let mut line = format!("    <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" ", l.x1, l.y1, l.x2, l.y2);

    let col = l.color.unwrap_or(Color { r: 0, g: 0, b: 0 });

    let mut style_attrs = vec!();
    style_attrs.push(stroke_color(col));
    let style = style(style_attrs);
    line.push_str(&style);

    line.push_str("/>\n");

    line
}

fn style(attrs: Vec<String>) -> String {
    format!("style=\"{}\"", attrs.join(";"))
}

fn stroke_color(col: Color) -> String {
    format!("stroke:rgb({},{},{})", col.r, col.g, col.b)
}