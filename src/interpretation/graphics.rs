//! Implements Graphics type and related structures to represent result of ZUMA interpretation.

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

    /// This is content of XML element, like this:
    /// <text>this is content</text>
    content: String,
    
    attributes: Vec<(String, String)>,
}

impl GraphicNode {
    pub fn tag(name: &str) -> GraphicNode {
        GraphicNode { name: name.to_string(), content: "".to_owned(), attributes: Vec::new() }
    }

    pub fn element(name: &str, content: &str) -> GraphicNode {
        GraphicNode { name: name.to_string(), content: content.to_owned(), attributes: Vec::new() }
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

    pub fn get_content(&self) -> String {
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