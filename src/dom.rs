use std::collections::HashMap;

#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
}

#[derive(Debug)]
pub struct Node {
    pub node_type: NodeType,
    pub children: Vec<Node>,
}

#[derive(Debug)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: HashMap<String, String>,
}

impl Node {
    pub fn element(
        tag_name: String,
        attributes: HashMap<String, String>,
        children: Vec<Node>,
    ) -> Self {
        Node {
            node_type: NodeType::Element(ElementData {
                tag_name,
                attributes,
            }),
            children,
        }
    }

    pub fn text(data: String) -> Self {
        Node {
            node_type: NodeType::Text(data),
            children: Vec::new(),
        }
    }
}
