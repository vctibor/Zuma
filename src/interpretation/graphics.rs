//! Implements Graphics type and related structures to represent result of ZUMA interpretation.

/// This is content of XML element, like this:
/// <text>this is content</text>
#[derive(Debug, PartialEq, Clone)]
pub enum ElementContent {
    Empty,
    Text(String),
    Elements(Box<Graphics>),
}

/// Represents ( ZUMA interpretation result / SVG document) as a set of graphical primitives.
#[derive(Debug, PartialEq, Clone)]
pub struct Graphics {
    nodes: Vec<GraphicNode>
}

/// Represents single graphical primitive (line, rectangle, Bezier...) by string used
/// to declare this specific primitive in SVG and dictionary of attributes.
#[derive(Debug, PartialEq, Clone)]
pub struct GraphicNode {
    name: String,
    content: ElementContent,
    attributes: Vec<(String, String)>,
}

impl ElementContent {
    pub fn from_str(content: &str) -> ElementContent {
        ElementContent::Text(content.clone().to_owned())
    }

    pub fn from_graphics(content: Graphics) -> ElementContent {
        ElementContent::Elements(Box::new(content))
    }
}

impl GraphicNode {
    pub fn empty_element(name: &str) -> GraphicNode {
        GraphicNode { name: name.to_string(), content: ElementContent::Empty, attributes: Vec::new() }
    }

    pub fn element(name: &str, content: ElementContent) -> GraphicNode {
        GraphicNode { name: name.to_string(), content, attributes: Vec::new() }
    }

    pub fn insert(mut self, attr_name: &str, attr_value: String) -> GraphicNode {
        self.attributes.push((attr_name.to_string(), attr_value));
        self
    }

    pub fn get_attributes(&self) -> Vec<(String, String)> {
        self.attributes.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_content(&self) -> ElementContent {
        self.content.clone()
    }
}

impl Graphics {
    pub fn new() -> Graphics {
        Graphics { nodes: vec!() }
    }

    pub fn add_many(mut self, mut nodes: Vec<GraphicNode>) -> Graphics {
        self.nodes.append(&mut nodes);
        self
    }

    pub fn get_nodes(&self) -> &Vec<GraphicNode> {
        &self.nodes
    }
}