use anyhow::{Result, anyhow};

#[derive(Debug, PartialEq, Clone)]
pub struct Document {
    pub expressions: Vec<Expression>
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    ConstantDeclaration(ConstantDeclaration),
    FunctionCall(FunctionCall),
    Scope(Scope),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Scope {
    pub expressions: Vec<Expression>
}

#[derive(Debug, PartialEq, Clone)]
pub enum ConstantOrLiteral {
    Const(String),  // name of constant used
    Literal(Value)
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConstantDeclaration {
    pub name: String,
    pub value: Value
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Arg>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Arg {
    pub name: String,
    pub value: ConstantOrLiteral
}


/// Types which can be declared by literal in Zuma.
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(f32),
    Point(Point),
    Color(Color),
    String(String)
}

impl Value {
    pub fn get_string(self) -> Result<String> {
        match self {
            Value::String(s) => Ok(s),
            _ => Err(anyhow!("Wrong type."))
        }
    }
    
    pub fn get_point(self) -> Result<Point> {
        match self {
            Value::Point(p) => Ok(p),
            _ => Err(anyhow!("Wrong type."))
        }
    }
    
    pub fn get_color(self) -> Result<Color> {
        match self {
            Value::Color(c) => Ok(c),
            _ => Err(anyhow!("Wrong type."))
        }
    }
    
    pub fn get_number(self) -> Result<f32> {
        match self {
            Value::Number(n) => Ok(n),
            _ => Err(anyhow!("Wrong type."))
        }
    }    
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