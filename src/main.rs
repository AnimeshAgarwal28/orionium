mod dom;
mod parser;
mod tokenizer;

use parser::Parser;

fn main() {
    let html = "<html><head><title>My Website</title></head><body><h1>Welcome!</h1><p>This is a test.</p></body></html>";

    let mut parser = Parser::new(html);
    let dom = parser.parse();

    println!("{:#?}", dom);
}
