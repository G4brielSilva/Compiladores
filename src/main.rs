mod enum_token;
mod node;

use enum_token::Token;
use node::Node;
use std::collections::LinkedList;

fn main() {
    let mut list: LinkedList<Node> = LinkedList::new();
    
    for value in 0..10 {
        list.push_back(Node {
            value: value.to_string(),
            token: Token::Struct,
        });
    }
    
    for elem in &list {
        println!("{} {}", elem.value, "Token Struct");
    }
}