use crate::dom::{ElementData, Node, NodeType};
use crate::tokenizer::{Token, Tokenizer};

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser {
            tokenizer: Tokenizer::new(input),
        }
    }

    fn parse_nodes(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();
        while let Some(token) = self.tokenizer.next_token() {
            match token {
                Token::StartTag(tag_name, attributes) => {
                    let children = self.parse_nodes();
                    nodes.push(Node::element(tag_name, attributes, children));
                }
                Token::EndTag(_) => break,
                Token::Text(text) => {
                    nodes.push(Node::text(text));
                }
            }
        }
        nodes
    }

    pub fn parse(&mut self) -> Node {
        let children = self.parse_nodes();
        if children.len() == 1 {
            children.into_iter().next().unwrap()
        } else {
            Node::element(
                "html".to_string(),
                std::collections::HashMap::new(),
                children,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_parser_simple_html() {
        let html = "<div><p>Hello, world!</p></div>";
        let mut parser = Parser::new(html);
        let dom_tree = parser.parse();

        // Root should be a <div> element
        if let NodeType::Element(element_data) = &dom_tree.node_type {
            assert_eq!(element_data.tag_name, "div");

            // <div> should have one child: <p>
            let children = &dom_tree.children;
            assert_eq!(children.len(), 1);

            if let NodeType::Element(p_element_data) = &children[0].node_type {
                assert_eq!(p_element_data.tag_name, "p");

                // <p> should have one child: text node "Hello, world!"
                let p_children = &children[0].children;
                assert_eq!(p_children.len(), 1);

                if let NodeType::Text(text) = &p_children[0].node_type {
                    assert_eq!(text, "Hello, world!");
                } else {
                    panic!("Expected text node inside <p>");
                }
            } else {
                panic!("Expected <p> element as a child of <div>");
            }
        } else {
            panic!("Expected <div> as root node");
        }
    }
}
