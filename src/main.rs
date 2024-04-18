mod enum_token;
mod node;

use enum_token::Token;
use node::Node;
use std::collections::LinkedList;
use std::fs::File;
use std::io::prelude::*;

fn separate_string(contents: &String) -> Vec<&str> {
    let delimiters = vec!['\n', ' '];
    let subs = contents.split(&delimiters[..])
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<&str>>();
    subs
}

fn main() -> std::io::Result<()> {
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

    let mut file = File::open("./test.jaca")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let subs = separate_string(&contents);
    println!("{:?}", subs);
    Ok(())
}