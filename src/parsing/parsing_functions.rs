use crate::parsing::*;

use anyhow::Result;

pub fn parse_color(s: String) -> Result<Color> {
    let red = u8::from_str_radix(&s[1..3], 16)?;
    let green = u8::from_str_radix(&s[3..5], 16)?;
    let blue = u8::from_str_radix(&s[5..7], 16)?;
    Ok(Color { red, green, blue })
}

pub fn number_arg(name: String, value: f32) -> Arg {
    Arg {
        name: name,
        value: Literal::Number(value)
    }
}

pub fn point_arg(name: String, value: Point) -> Arg {
    Arg {
        name: name,
        value: Literal::Point(value)
    }
}

pub fn color_arg(name: String, value: Color) -> Arg {
    Arg {
        name: name,
        value: Literal::Color(value)
    }
}