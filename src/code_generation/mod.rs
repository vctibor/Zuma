use crate::interpretation::*;

mod tests;

static INDENT_SIZE: usize = 4;

//static SVG_OPEN: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"500\" height=\"500\">";
static SVG_OPEN: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1000\" height=\"1000\">";
static SVG_CLOSE: &str = "</svg>";

pub fn generate(graphics: &Graphics) -> String {

    let mut document = "".to_owned();

    document.push_str(SVG_OPEN);
    document.push_str("\n");

    let graphics_result = gen_graphics(graphics, 1);
    document.push_str(&graphics_result);

    document.push_str(SVG_CLOSE);
    document
}

fn gen_graphics(graphics: &Graphics, indent_level: usize) -> String {
    let mut document = "".to_owned();
    for node in graphics.get_nodes() {
        let xml_element = element(node, indent_level);

        let indent_chars = " ".repeat(indent_level * INDENT_SIZE);
        document.push_str(&indent_chars);

        document.push_str(&xml_element);
        document.push_str("\n");
    }
    document
}

fn element(node: &GraphicNode, indent_level: usize) -> String {

    let name = node.get_name();
    let attrs = gen_attributes(node.get_attributes());
    let mut content = content(node.get_content(), indent_level + 1);

    if content != "" {
        let indent_chars = " ".repeat(indent_level * INDENT_SIZE);
        content = format!("\n{}{}", content, indent_chars);
    }

    match attrs {
        Some(a) => format!("<{} {}>{}</{}>", name, a, content, name),
        None => format!("<{}>{}</{}>", name, content, name)
    }
    
}

fn gen_attributes(attributes: Vec<(String, String)>) -> Option<String> {
    
    if attributes.len() == 0 {
        return None
    }
    
    let mut attrs = vec!();

    for attr in attributes {
        attrs.push(format!("{}=\"{}\"", attr.0, attr.1));
    }

    attrs.sort();
    Some(attrs.join(" "))
}

fn content(content: ElementContent, indent_level: usize) -> String {
    use ElementContent::*;
    match content {
        Empty => "".to_owned(),
        Text(t) => t,
        Elements(e) => gen_graphics(e.as_ref(), indent_level),
    }
}