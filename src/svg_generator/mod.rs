mod tests;

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
    Rectangle(Rectangle),
    Text(Text)
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

impl From<Text> for Element {
    fn from(text: Text) -> Self {
        Element::Text(text)
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

pub struct Text {
    x: f32,
    y: f32,
    text: String,
    fill_color: Option<(u8, u8, u8)>
}

impl Text {

    pub fn new(x: f32, y: f32, text: String) -> Text {
        Text {
            x, y, text,
            fill_color: None
        }
    }

    pub fn fill_color(mut self, r: u8, g: u8, b: u8) -> Text {
        self.fill_color = Some((r, g, b));
        self
    }
}

//
// GENERATOR
//

mod style_attributes;
use style_attributes::*;

mod xml_helpers;
use xml_helpers::*;

static INDENT_SIZE: usize = 4;

//static SVG_OPEN: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"500\" height=\"500\">";
static SVG_OPEN: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1000\" height=\"1000\">";
static SVG_CLOSE: &str = "</svg>";

use maplit::hashmap;

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
            Rectangle(r) => generator.add(rectangle(r), 1),
            Text(t) => generator.add(text(t), 1),
        }
    }

    generator.close()
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

fn text(t: Text) -> String {
    let mut attrs = hashmap!(
        "x".to_owned() => t.x.to_string(),
        "y".to_owned() => t.y.to_string()
    );

    let mut style_attrs = vec!();
    style_attrs.push(fill_color(t.fill_color));
    if style_attrs.len() > 0 {
        let style = style_attrs.join(";");
        attrs.insert("style".to_owned(), style);
    }
    
    xml_element("text".to_owned(), attrs, t.text)
}