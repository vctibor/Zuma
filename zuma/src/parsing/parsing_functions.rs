use crate::parsing::*;

use anyhow::Result;

pub fn parse_color(s: String) -> Result<Color> {
    let red = u8::from_str_radix(&s[1..3], 16)?;
    let green = u8::from_str_radix(&s[3..5], 16)?;
    let blue = u8::from_str_radix(&s[5..7], 16)?;
    Ok(color(red, green, blue))
}

pub fn strip_quotes(value: &str) -> String {
    let mut chars = value.chars();
    chars.next();
    let mut value = chars.as_str().to_owned();
    value.pop();
    value
}


pub fn color(red: u8, green: u8, blue: u8) -> Color {
    let red = OperationInput::Literal(Value::Number(red as f32));
    let green = OperationInput::Literal(Value::Number(green as f32));
    let blue = OperationInput::Literal(Value::Number(blue as f32));
    Color { red: Box::new(red), green: Box::new(green), blue: Box::new(blue) }
}