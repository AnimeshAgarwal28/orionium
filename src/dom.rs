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

    pub fn render(&self) -> String {
        match &self.node_type {
            NodeType::Text(text) => text.clone(),
            NodeType::Element(element_data) => {
                let mut result = String::new();
                result.push_str(&format!("<{}", element_data.tag_name));

                for (name, value) in &element_data.attributes {
                    result.push_str(&format!(" {}=\"{}\"", name, value));
                }

                result.push('>');

                for child in &self.children {
                    result.push_str(&child.render());
                }

                result.push_str(&format!("</{}>", element_data.tag_name));
                result
            }
        }
    }
}
