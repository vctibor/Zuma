pub mod svg_model {

    pub struct Document {
        pub elements: Vec<Element>
    }
    
    pub enum Element {
        Line(Line)
    }

    pub struct Line {
        pub x1: f32,
        pub y1: f32,
        pub x2: f32,
        pub y2: f32,
    }
}

use crate::generator::svg_model::*;

pub fn generate_svg(doc: Document) -> String {
    use crate::generator::Element::*;

    let mut svg = "<svg>".to_owned();

    for el in doc.elements {
        match el {
            Line(l) => svg.push_str(&generate_line(l))
        }
    }

    svg.push_str("</svg>");

    svg
}

fn generate_line(l: Line) -> String {
    format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>", l.x1, l.y1, l.x2, l.y2)
}