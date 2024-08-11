use std::collections::HashMap;

#[derive(Debug)]
pub enum Token {
    Text(String),
    StartTag(String, HashMap<String, String>),
    EndTag(String),
}
