use crate::dom::{Node, NodeType};

pub trait Render {
    fn render(&self) -> String;
}

impl Render for Node {
    fn render(&self) -> String {
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
