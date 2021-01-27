#[derive(Debug, PartialEq)]
pub enum GeometricPrimitive {
    Line
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Boolean(bool),
    Number(f32),
    Color(Color),
    Point((f32, f32))
}

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

pub fn parse_color(s: String) -> Color {
    match s.as_str() {
        "black"  => Color { red: 0, green: 0, blue: 0 },
        "white"  => Color { red: 255, green: 255, blue: 255 },
        "red"    => Color { red: 255, green: 0, blue: 0 },
        "green"  => Color { red: 0, green: 255, blue: 0 },
        "blue"   => Color { red: 0, green: 0, blue: 255 },
        "yellow" => Color { red: 255, green: 255, blue: 0 },
        hexadecimal => {
            let r: u8 = u8::from_str_radix(&hexadecimal[1..3], 16).unwrap();
            let g: u8 = u8::from_str_radix(&hexadecimal[3..5], 16).unwrap();
            let b: u8 = u8::from_str_radix(&hexadecimal[5..7], 16).unwrap();
            Color { red: r, green: g, blue: b }
        }
    }
}