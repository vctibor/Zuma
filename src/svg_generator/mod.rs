pub mod svg_model;

mod tests;

use crate::svg_generator::svg_model::*;

pub fn generate_svg(doc: Document) -> String {
    use crate::svg_generator::Element::*;

    let mut svg = "<svg xmlns=\"http://www.w3.org/2000/svg\">\n".to_owned();

    for el in doc.elements {
        match el {
            Line(l) => svg.push_str(&generate_line(l))
        }
    }

    svg.push_str("</svg>");

    svg
}

fn generate_line(l: Line) -> String {
    let mut line = format!("    <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" ", l.x1, l.y1, l.x2, l.y2);

    if let Some(color) = l.color {
        let col_info = format!("style=\"stroke:rgb({},{},{})\" ", color.r, color.g, color.b);
        line.push_str(&col_info);
    }

    line.push_str("/>\n");

    line
}
