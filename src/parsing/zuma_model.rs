#[derive(Debug, PartialEq, Clone)]
pub struct Document {
    pub lines: Vec<FunctionCall>
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Arg>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Arg {
    pub name: String,
    pub value: Literal
}

/// Types which can be decalred by literal in Zuma.
#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Number(f32),
    Point(Point),
    Color(Color)
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