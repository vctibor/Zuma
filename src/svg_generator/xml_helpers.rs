use std::collections::HashMap;

pub fn xml_tag(name: String, attrs: HashMap<String, String>) -> String {
    format!("<{} {}/>", name, attributes(attrs))
}

pub fn xml_element(name: String, attrs: HashMap<String, String>, content: String) -> String {
    format!("<{} {}>{}</{}>", name, attributes(attrs), content, name)
}

pub fn attributes(attributes: HashMap<String, String>) -> String {
    let mut attrs = vec!();

    for attr in attributes {
        attrs.push(format!("{}=\"{}\"", attr.0, attr.1));
    }

    attrs.sort();

    attrs.join(" ")
}