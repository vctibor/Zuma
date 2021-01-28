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
    pub color: Option<Color>
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}