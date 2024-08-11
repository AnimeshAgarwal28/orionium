use crate::tokenizer::token::Token;
use std::collections::HashMap;

pub struct Tokenizer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokenizer { input, position: 0 }
    }

    fn next_char(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    fn consume_char(&mut self) -> Option<char> {
        let mut iter = self.input[self.position..].char_indices();
        if let Some((_, current_char)) = iter.next() {
            self.position += current_char.len_utf8();
            Some(current_char)
        } else {
            None
        }
    }

    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while let Some(c) = self.next_char() {
            if !test(c) {
                break;
            }
            result.push(self.consume_char().unwrap());
        }
        result
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| c.is_alphanumeric())
    }

    fn parse_attributes(&mut self) -> HashMap<String, String> {
        let mut attributes = HashMap::new();
        while let Some(c) = self.next_char() {
            if c == '>' || c == '/' {
                break;
            }
            self.consume_whitespace();
            let name = self.parse_tag_name();
            self.consume_whitespace();
            self.consume_char(); // '='
            self.consume_whitespace();
            let value = self.parse_attribute_value();
            attributes.insert(name, value);
            self.consume_whitespace();
        }
        attributes
    }

    fn parse_attribute_value(&mut self) -> String {
        let open_quote = self.consume_char();
        let value = self.consume_while(|c| c != open_quote.unwrap());
        self.consume_char(); // closing quote
        value
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.consume_whitespace();
        if self.position >= self.input.len() {
            return None;
        }
        let c = self.next_char().unwrap();
        if c == '<' {
            self.consume_char(); // '<'
            if self.next_char() == Some('/') {
                self.consume_char(); // '/'
                let tag_name = self.parse_tag_name();
                self.consume_char(); // '>'
                Some(Token::EndTag(tag_name))
            } else {
                let tag_name = self.parse_tag_name();
                let attributes = self.parse_attributes();
                self.consume_char(); // '>'
                Some(Token::StartTag(tag_name, attributes))
            }
        } else {
            let text = self.consume_while(|c| c != '<');
            Some(Token::Text(text))
        }
    }
}
