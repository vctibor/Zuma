// This sits on top of both Zuma parser and SVG generator, and translates from Zuma IR to svg_model.

use crate::ast as zuma;
use crate::generator::svg_model;

pub fn translate(zuma_ir: zuma::Document) -> svg_model::Document {
    
    use zuma::GeometricPrimitive as gp;
    use svg_model::Element as e;

    let mut elements = Vec::new();

    for primitive in zuma_ir.primitives {
        match primitive {
            gp::Line(l) => {
                elements.push(e::Line(svg_model::Line {
                    x1: l.start.x, y1: l.start.y, x2: l.end.x, y2: l.end.y, color: l.color.map(translate_color)
                }));
            }
        }
    }
    
    svg_model::Document { elements }
}

fn translate_color(c: zuma::Color) -> svg_model::Color {
    svg_model::Color { r: c.red, g: c.green, b: c.blue }
}