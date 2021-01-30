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

    pub fn add(mut self, el: Element) -> Document {
        self.elements.push(el);
        self
    }

    pub fn generate(self) -> String {
        generate(self)
    }
}

pub enum Element {
    Line(Line)
}

impl From<Line> for Element {
    fn from(line: Line) -> Self {
        Element::Line(line)
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


//
// GENERATOR
//

static INDENT_SIZE: usize = 4;

static SVG_OPEN: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\">";
static SVG_CLOSE: &str = "</svg>";

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
            Line(l) => generator.add(line(l), 1)
        }
    }

    generator.close()
}

fn line(l: Line) -> String {
    let mut line = format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" ", l.x1, l.y1, l.x2, l.y2);

    let col = l.color.unwrap_or((0, 0, 0 ));

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

fn stroke_color(rgb: (u8, u8, u8)) -> String {
    format!("stroke:rgb({},{},{})", rgb.0, rgb.1, rgb.2)
}

//
// TESTS
//

#[test]
fn svg_gen_test_1() {
    Document::new()
        .add(Line::new(0.0, 0.0, 1.0, 10.0)
            .color(128, 25, 45)
            .width(3.0)
            .into()
        )
    .generate();
}