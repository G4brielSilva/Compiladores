mod enum_token;
mod node;

use enum_token::Token;
use node::Node;
use std::collections::LinkedList;
use std::fs::File;
use std::io::prelude::*;

fn separate_file_content(content: &String) -> Vec<String> {
    let delimiters = vec!['\n', ' '];
    let subs = content.split(&delimiters[..])
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<&str>>();
    
    let mut result: Vec<String> = Vec::new();
    for sub in &subs {
        result.append(&mut break_token(sub));
    }

    result.retain(|s| !s.is_empty());
    result
}

fn read_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn is_string_alphanumeric_or_underscore(s: &str) -> bool {
    s.chars().all(|c| c.is_alphanumeric() || c == '_')
}

fn is_char_alphanumeric_or_underscore(c: &char) -> bool {
    c.is_alphanumeric() || *c == '_'
}

fn break_token(token: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let mut string = String::new();
    for c in token.chars() {
        if is_char_alphanumeric_or_underscore(&c) {
            string.push(c);
        } else {
            result.push(string.clone());
            result.push(c.to_string());
            string.clear();
        }
    }

    if !string.is_empty() {
        result.push(string.clone());
    }
    result
}


fn classificate_value(value: &str) -> Token {
    match value {
        "struct" => Token::Struct,
        "class" | "interface" => Token::Instance,
        "abstract" | "concrete" => Token::ClassType,
        "extends" | "implements" => Token::Inheritance,
        "public" | "private" | "protected" => Token::Visibility,
        "static" | "local" => Token::Scope,
        "base" | "final" => Token::Final,
        "int" | "float" | "double" | "char" | "void" => Token::Type,
        ";" => Token::Separator,
        "(" | ")" => Token::Parenthesis,
        "[" | "]" => Token::ArrayBracket,
        "{" | "}" => Token::Block,
        "while" | "do" | "if" | "for" | "switch" | "break" | "continue" | "return" => Token::Command,
        "=" => Token::Atrib, //revisar
        "else" => Token::Else,
        "case" | "default" => Token::Case, //revisar default
        "++" | "--" => Token::Operator,
        "," => Token::ParamList,
        ">" | "<" | ">=" | "<=" | "==" | "!=" => Token::LogicOperator,
        "+" | "-" | "*" | "/" => Token::MathOperator,
        "this" => Token::This,
        "." => Token::Field,
        ' " ' => Token::String,
        " ' " => Token::Char,
        default => Token::Identifier
    }
}

fn print_linked_list(list: &LinkedList<Node>) {
    for node in list {
        println!("{:?} {:?}", node.value, node.token);
    }
}

fn main() -> std::io::Result<()> {
    let mut list: LinkedList<Node> = LinkedList::new();
    
    let mut contents = read_file("./test.jaca")?; 

    let strings = separate_file_content(&contents); // Separando as strings do arquivo em tokens
    println!("{:?}", strings);

    // Transformando Objeto String em literal &str para facilitar comparação
    let parsed_strings: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();
    

    for value in parsed_strings {
        // Classificando valores no tipo de token
        let token = classificate_value(value);
        list.push_back(Node {
            value: value.to_string(),
            token
        });
    }

    print_linked_list(&list);

    Ok(())
}