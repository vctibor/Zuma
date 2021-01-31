pub fn opacity(opacity: Option<f32>) -> String {
    if let Some(opacity) = opacity {
        format!("opacity:{}", opacity)
    } else {
        "".to_owned()
    }
}

pub fn fill_color(fill: Option<(u8, u8, u8)>) -> String {
    if let Some(fill) = fill {
        format!("fill:rgb({},{},{})", fill.0, fill.1, fill.2)
    } else {
        "".to_owned()
    }
}

pub fn stroke_width(width: Option<f32>) -> String {
    if let Some(width) = width {
        format!("stroke-width:{}", width)
    } else {
        "".to_owned()
    }
}

pub fn stroke_color(rgb: Option<(u8, u8, u8)>) -> String {
    if let Some(rgb) = rgb {
        format!("stroke:rgb({},{},{})", rgb.0, rgb.1, rgb.2)
    } else {
        "".to_owned()
    }
}