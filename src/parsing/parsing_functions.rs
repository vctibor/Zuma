use crate::parsing::*;

use anyhow::Result;

pub fn parse_color(s: String) -> Result<Color> {
    let red = u8::from_str_radix(&s[1..3], 16)?;
    let green = u8::from_str_radix(&s[3..5], 16)?;
    let blue = u8::from_str_radix(&s[5..7], 16)?;
    Ok(Color { red, green, blue })
}

pub fn strip_quotes(value: &str) -> String {
    let mut chars = value.chars();
    chars.next();
    let mut value = chars.as_str().to_owned();
    value.pop();
    value
}