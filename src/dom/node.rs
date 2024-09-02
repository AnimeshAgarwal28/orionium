use std::collections::HashMap;

pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
    // TODO: Several other node types to be defined later.
}

#[derive(Debug)]
pub struct Node {
    pub node_type: NodeType, // this is specific to each node
    pub children: Vec<Node>, // this is common to all nodes
}

#[derive(Debug)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
}


impl Node {
    pub fn element(
        tag_name: String,
        attributes: AttrMap,
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
