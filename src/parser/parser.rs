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
