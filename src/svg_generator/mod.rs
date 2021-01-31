//
// PUBLIC INTERFACE
//

pub struct Document {
    elements: Vec<Element>
}

impl Document {
    pub fn new() -> Document {
        Document {
            elements: Vec::new() 
        }
    }

    #[allow(dead_code)]
    pub fn add(mut self, el: Element) -> Document {
        self.elements.push(el);
        self
    }

    pub fn add_many(mut self, els: &mut Vec<Element>) -> Document {
        self.elements.append(els);
        self
    }

    pub fn generate(self) -> String {
        generate(self)
    }
}

pub enum Element {
    Line(Line),
    Rectangle(Rectangle)
}

impl From<Line> for Element {
    fn from(line: Line) -> Self {
        Element::Line(line)
    }
}

impl From<Rectangle> for Element {
    fn from(rect: Rectangle) -> Self {
        Element::Rectangle(rect)
    }
}

pub struct Line {
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
    color: Option<(u8, u8, u8)>,
    width: Option<f32>
}

impl Line {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Line {
        Line {
            x1, y1, x2, y2,
            color: None,
            width: None,
        }
    }

    pub fn color(mut self, r: u8, g: u8, b: u8) -> Line {
        self.color = Some((r, g, b));
        self
    }

    pub fn width(mut self, width: f32) -> Line {
        self.width = Some(width);
        self
    }
}

pub struct Rectangle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    stroke_width: Option<f32>,
    stroke_color: Option<(u8, u8, u8)>,
    fill_color: Option<(u8, u8, u8)>,
    opacity: Option<f32>
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Rectangle {
        Rectangle {
            x, y, width, height,
            stroke_width: None,
            stroke_color: None,
            fill_color: None,
            opacity: None
        }
    }

    pub fn stroke_width(mut self, width: f32) -> Rectangle {
        self.stroke_width = Some(width);
        self
    }

    pub fn stroke_color(mut self, r: u8, g: u8, b: u8) -> Rectangle {
        self.stroke_color = Some((r, g, b));
        self
    }

    pub fn fill_color(mut self, r: u8, g: u8, b: u8) -> Rectangle {
        self.fill_color = Some((r, g, b));
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Rectangle {
        self.opacity = Some(opacity);
        self
    }
}

//
// GENERATOR
//

static INDENT_SIZE: usize = 4;

static SVG_OPEN: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\">";
static SVG_CLOSE: &str = "</svg>";

use maplit::hashmap;
use std::collections::HashMap;

struct Generator {
    content: String
}

impl Generator {

    pub fn open() -> Generator {
        let mut content = "".to_owned();
        content.push_str(SVG_OPEN);
        content.push_str("\n");
        Generator { content }
    }

    pub fn add(&mut self, line: String, indent_level: usize) {
        let indent_chars = " ".repeat(INDENT_SIZE * indent_level);
        self.content.push_str(&indent_chars);
        self.content.push_str(&line);
        self.content.push_str("\n");
    }

    pub fn close(mut self) -> String {
        self.content.push_str(SVG_CLOSE);
        self.content
    }
}

pub fn generate(doc: Document) -> String {

    let mut generator = Generator::open();

    use crate::svg_generator::Element::*;
    for el in doc.elements {
        match el {
            Line(l) => generator.add(line(l), 1),
            Rectangle(r) => generator.add(rectangle(r), 1)
        }
    }

    generator.close()
}

fn rectangle(r: Rectangle) -> String {

    let mut attrs = hashmap!(
        "x".to_owned() => r.x.to_string(),
        "y".to_owned() => r.y.to_string(),
        "width".to_owned() => r.width.to_string(),
        "height".to_owned() => r.height.to_string()
    );

    let mut style_attrs = vec!();
    style_attrs.push(stroke_width(r.stroke_width));
    style_attrs.push(stroke_color(r.stroke_color));
    style_attrs.push(fill_color(r.fill_color));
    style_attrs.push(opacity(r.opacity));
    if style_attrs.len() > 0 {
        let style = style_attrs.join(";");
        attrs.insert("style".to_owned(), style);
    }
    
    xml_tag("rect".to_owned(), attrs)
}

fn line(l: Line) -> String {
    let mut attrs = hashmap!(
        "x1".to_owned() => l.x1.to_string(),
        "y1".to_owned() => l.y1.to_string(),
        "x2".to_owned() => l.x2.to_string(),
        "y2".to_owned() => l.y2.to_string()
    );

    let mut style_attrs = vec!();
    style_attrs.push(stroke_width(l.width));
    style_attrs.push(stroke_color(l.color));
    if style_attrs.len() > 0 {
        let style = style_attrs.join(";");
        attrs.insert("style".to_owned(), style);
    }
    
    xml_tag("line".to_owned(), attrs)
}


// XML helper functions

fn xml_tag(name: String, attrs: HashMap<String, String>) -> String {
    format!("<{} {}/>", name, attributes(attrs))
}

fn attributes(attributes: HashMap<String, String>) -> String {
    let mut attrs = vec!();

    for attr in attributes {
        attrs.push(format!("{}=\"{}\"", attr.0, attr.1));
    }

    attrs.sort();

    attrs.join(" ")
}


// Style attributes

fn opacity(opacity: Option<f32>) -> String {
    if let Some(opacity) = opacity {
        format!("opacity:{}", opacity)
    } else {
        "".to_owned()
    }
}

fn fill_color(fill: Option<(u8, u8, u8)>) -> String {
    if let Some(fill) = fill {
        format!("fill:rgb({},{},{})", fill.0, fill.1, fill.2)
    } else {
        "".to_owned()
    }
}

fn stroke_width(width: Option<f32>) -> String {
    if let Some(width) = width {
        format!("stroke-width:{}", width)
    } else {
        "".to_owned()
    }
}

fn stroke_color(rgb: Option<(u8, u8, u8)>) -> String {
    if let Some(rgb) = rgb {
        format!("stroke:rgb({},{},{})", rgb.0, rgb.1, rgb.2)
    } else {
        "".to_owned()
    }
}

////

//
// TESTS
//

#[test]
fn svg_gen_test_line() {
    let actual = Document::new()
        .add(Line::new(0.0, 0.0, 1.0, 10.0)
            .color(128, 25, 45)
            .width(3.0)
            .into()
        )
        .generate();

    let expected = r#"
<svg xmlns="http://www.w3.org/2000/svg">
    <line style="stroke-width:3;stroke:rgb(128,25,45)" x1="0" x2="1" y1="0" y2="10"/>
</svg>
    "#.trim();

    assert_eq!(actual, expected);
}