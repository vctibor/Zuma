#[derive(Debug, PartialEq, Clone)]
pub struct Document {
    pub primitives: Vec<GeometricPrimitive>
}

#[derive(Debug, PartialEq, Clone)]
pub enum GeometricPrimitive {
    Line(Line)
}

#[derive(Debug, PartialEq, Clone)]
pub struct Line {
    pub start: Point,
    pub end: Point,
    pub color: Option<Color>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32
}