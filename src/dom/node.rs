use crate::dom::ElementData;

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

impl Node {
    pub fn element(
        tag_name: String,
        attributes: std::collections::HashMap<String, String>,
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
