use std::collections::HashMap;

#[derive(Debug)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: HashMap<String, String>,
}
