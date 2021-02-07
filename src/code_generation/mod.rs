use crate::interpretation::{Graphics, GraphicNode};

static INDENT_SIZE: usize = 4;

//static SVG_OPEN: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"500\" height=\"500\">";
static SVG_OPEN: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1000\" height=\"1000\">";
static SVG_CLOSE: &str = "</svg>";

pub fn generate(graphics: Graphics) -> String {

    let mut document = "".to_owned();

    document.push_str(SVG_OPEN);
    document.push_str("\n");

    for node in graphics.get_nodes() {
        let xml_element = generate_xml_element(node);

        let indent_chars = " ".repeat(INDENT_SIZE);
        document.push_str(&indent_chars);

        document.push_str(&xml_element);
        document.push_str("\n");
    }

    document.push_str(SVG_CLOSE);
    document
}

fn generate_xml_element(node: &GraphicNode) -> String {

    let name = node.get_name();
    let attrs = attributes(node.get_attributes());
    let content = node.get_content();
    format!("<{} {}>{}</{}>", name, attrs, content, name)
}

fn attributes(attributes: Vec<(String, String)>) -> String {
    let mut attrs = vec!();

    for attr in attributes {
        attrs.push(format!("{}=\"{}\"", attr.0, attr.1));
    }

    attrs.sort();

    attrs.join(" ")
}