mod enum_token;
mod node;
mod tree;

use tree::TreeNode;
use enum_token::Token;
use node::Node;
use std::collections::LinkedList;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

const EPSLON: &str = "ε";

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

    if valid_numeric_value(token) || has_more_then_one_decimal_point(token) || valid_string_value(token) || valid_char_value(token) {
        result.push(token.to_string());
        return result;
    }

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

fn valid_numeric_value(s: &str) -> bool {
    let re = Regex::new(r"^\d+(\.\d+)?$").unwrap();
    re.is_match(s)
}

fn valid_string_value(s: &str) -> bool {
    let re = regex::Regex::new(r#""([^"]*)""#).unwrap();
    re.is_match(s)
}

fn valid_char_value(s: &str) -> bool {
    let re = regex::Regex::new(r#"'([^']*)'"#).unwrap();
    re.is_match(s)
}

fn has_more_then_one_decimal_point(s: &str) -> bool {
    let re = Regex::new(r"^[^.]*\..*\..*$").unwrap();
    re.is_match(s)
}

fn classificate_identifier_number_or_error(value: &str) -> Token {
    if valid_numeric_value(value) {
        return Token::Number
    }
    if has_more_then_one_decimal_point(value) {
        return Token::Error
    }
    return Token::Identifier
}

fn classificate_value(value: &str) -> Token {
    if valid_string_value(value) {
        return Token::String
    }

    if valid_char_value(value) {
        return Token::Char
    }

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
        "=" => Token::Atrib,
        "else" => Token::Else,
        "case" | "default" => Token::Case,
        "++" | "--" => Token::Operator,
        "," => Token::ParamList,
        ">" | "<" | ">=" | "<=" | "==" | "!=" => Token::LogicOperator,
        "+" | "-" | "*" | "/" => Token::MathOperator,
        "this" => Token::This,
        "." => Token::Field,
        default => classificate_identifier_number_or_error(default)
    }
}

fn print_linked_list(list: &LinkedList<Node>) {
    let mut iter = list.iter();

    // Usando next para iterar sobre a lista
    while let Some(value) = iter.next() {
        println!("{} - {}", value.value, value.token);
    }
}

// fn give_grammatical_structure(tree: &mut TreeNode<&str>, list: &LinkedList<Node>) {
// fn give_grammatical_structure<'a>(tree: &mut TreeNode<&'a str>, list: &'a LinkedList<Node>) {
fn give_grammatical_structure<'a>(tree: &mut TreeNode<&'a str>, list_iter: &mut std::collections::linked_list::IterMut<'a, Node>) {
        match tree.value {
            "PROGRAM" => {
                tree.add_child(TreeNode::new("DECLARATION"));
                give_grammatical_structure(&mut tree.children[0], list_iter);

                tree.add_child(TreeNode::new("DECLARATIONS"));
                give_grammatical_structure(&mut tree.children[1], list_iter);
            }
            "DECLARATION" => {
                tree.add_child(TreeNode::new("STRUCT"));
                give_grammatical_structure(&mut tree.children[0], list_iter);
                
                tree.add_child(TreeNode::new("ID"));            
                give_grammatical_structure(&mut tree.children[1], list_iter);

                tree.add_child(TreeNode::new("INHERITANCE"));
                give_grammatical_structure(&mut tree.children[2], list_iter);

                tree.add_child(TreeNode::new("{"));

                tree.add_child(TreeNode::new("ITEM_DECLS"));
                give_grammatical_structure(&mut tree.children[4], list_iter);

                tree.add_child(TreeNode::new("}"));
            },
            "DECLARATIONS" => {
                if let Some(_) = list_iter.next() {
                    tree.add_child(TreeNode::new("DECLARATION"));
                    give_grammatical_structure(&mut tree.children[0], list_iter);

                    tree.add_child(TreeNode::new("DECLARATIONS"));
                    give_grammatical_structure(&mut tree.children[1], list_iter);   
                } else {
                    tree.add_child(TreeNode::new("ε"));
                    return;
                }
            },
            "STRUCT" => {
                if let Some(node_item) = list_iter.next() {
                    if node_item.value != "interface" {
                        tree.add_child(TreeNode::new("INSTANCE"));
                        give_grammatical_structure(&mut tree.children[0], list_iter);
                        
                        tree.add_child(TreeNode::new("class"));
                    } else {
                        tree.add_child(TreeNode::new("interface"));
                        return;
                    }
                }
            },
            "INSTANCE" => {
                if let Some(node_item) = list_iter.next() { 
                    if node_item.value == "abstract" || node_item.value == "concrete" {
                        tree.add_child(TreeNode::new(&node_item.value));
                    } 
                }
            },
            "INHERITANCE" => {
                if let Some(node_item) = list_iter.next() { 
                    if node_item.value == "extends" || node_item.value == "implements"{
                        tree.add_child(TreeNode::new(&node_item.value));
                        tree.add_child(TreeNode::new("ID"));
                        give_grammatical_structure(&mut tree.children[1], list_iter);
                    } else {
                        tree.add_child(TreeNode::new(EPSLON))
                    }
                }
            },
            "ITEM_DECLS" => { //[VISIBILITY] [SCOPE] [FINAL] [ITEM_DECL] (;) [ITEM_DECLS]
                !
                tree.add_child(TreeNode::new("VISIBILITY"));
                give_grammatical_structure(&mut tree.children[0], list_iter);
                
                tree.add_child(TreeNode::new("SCOPE"));
                give_grammatical_structure(&mut tree.children[1], list_iter);

                tree.add_child(TreeNode::new("FINAL"));
                give_grammatical_structure(&mut tree.children[2], list_iter);

                tree.add_child(TreeNode::new("ITEM_DECL"));
                give_grammatical_structure(&mut tree.children[3], list_iter);

                tree.add_child(TreeNode::new(";"));

                tree.add_child(TreeNode::new("ITEM_DECLS"));
                give_grammatical_structure(&mut tree.children[5], list_iter);
            },
            "VISIBILITY" => { // (public) | (protected) | (private)
                if let Some(node_item) = list_iter.next() { 
                    if node_item.value == "public" || node_item.value == "protected" || node_item.value == "private" {
                        tree.add_child(TreeNode::new(&node_item.value));
                    }
                }
            },
            "SCOPE" => { // (static) | (local)
                if let Some(node_item) = list_iter.next() { 
                    if node_item.value == "static" || node_item.value == "local" {
                        tree.add_child(TreeNode::new(&node_item.value));
                    }
                }
            },
            "FINAL" => { // (final) | (base)
                if let Some(node_item) = list_iter.next() { 
                    if node_item.value == "final" || node_item.value == "base" {
                        tree.add_child(TreeNode::new(&node_item.value));
                    }
                }
            },
            "ITEM_DECL" => { // [ATRIB_DECL] | [METHOD_DECL]
                // É necessária implementar a lógica para pegar ATRIB ou METHOD DECL    
                if let Some(node_item) = list_iter.next() {
                    if node_item.value == "ATRIB_DECL" || node_item.value == "METHOD_DECL" {
                        tree.add_child(TreeNode::new(&node_item.value));
                    } 
                }
            },
            "ATRIB_DECL" => { // [TYPE] [VAR] [VAR_LIST] (;)
                tree.add_child(TreeNode::new("TYPE"));
                give_grammatical_structure(&mut tree.children[0], list_iter);

                tree.add_child(TreeNode::new("VAR"));
                give_grammatical_structure(&mut tree.children[1], list_iter);

                tree.add_child(TreeNode::new("VAR_LIST"));
                give_grammatical_structure(&mut tree.children[2], list_iter);

                tree.add_child(TreeNode::new(";"));
                return;
            },
            "METHOD_DECL" => { // [INSTANCE] [TYPE] [METHOD]
                tree.add_child(TreeNode::new("INSTANCE"));
                give_grammatical_structure(&mut tree.children[0], list_iter);

                tree.add_child(TreeNode::new("TYPE"));
                give_grammatical_structure(&mut tree.children[1], list_iter);

                tree.add_child(TreeNode::new("METHOD"));
                give_grammatical_structure(&mut tree.children[2], list_iter);
            },
            "TYPE" => {
                if let Some(node_item) = list_iter.next() { 
                    if node_item.value == "int" || node_item.value == "float" || node_item.value == "double" || node_item.value == "char" || node_item.value == "void" {
                        tree.add_child(TreeNode::new(&node_item.value));
                    } else {
                        tree.add_child(TreeNode::new("ID"));
                        tree.add_child(TreeNode::new(&node_item.value));

                        tree.add_child(TreeNode::new("NAME"));
                        give_grammatical_structure(&mut tree.children[2], list_iter);
                    }
                }
            },
            "VAR" => {
                tree.add_child(TreeNode::new("ID"));
                give_grammatical_structure(&mut tree.children[0], list_iter);

                tree.add_child(TreeNode::new("ARRAY"));
                give_grammatical_structure(&mut tree.children[1], list_iter);

                tree.add_child(TreeNode::new("VALUE"));
                give_grammatical_structure(&mut tree.children[2], list_iter);

                tree.add_child(TreeNode::new(";"));
            },
            "VALUE" => {
                while let Some(node_item) = list_iter.next() { // Isso é apenas um stop do processamento, basta remover essa parte e implementar o resto
                    tree.add_child(TreeNode::new(&node_item.value));
                }
                return;

                if let Some(node_item) = list_iter.next() { 
                    if node_item.value == "=" {
                        tree.add_child(TreeNode::new("="));
                        tree.add_child(TreeNode::new("EXP"));
                        
                        give_grammatical_structure(&mut tree.children[1], list_iter);
                    }
                }
            },
            "VAR_LIST" => {
                if let Some(node_item) = list_iter.next() { 
                    if node_item.value == "," {
                        tree.add_child(TreeNode::new(","));
                        tree.add_child(TreeNode::new("VAR"));
                        give_grammatical_structure(&mut tree.children[1], list_iter);

                        tree.add_child(TreeNode::new("VAR_LIST"));
                        give_grammatical_structure(&mut tree.children[1], list_iter);
                    } else {
                        tree.add_child(TreeNode::new(EPSLON));
                        return;
                    }
                }
            },
            "ARRAY" => {
                if let Some(node_item) = list_iter.next() { 
                    if node_item.value == "[" {
                        tree.add_child(TreeNode::new("["));
                        tree.add_child(TreeNode::new("]"));
                        
                        list_iter.next();

                        tree.add_child(TreeNode::new("ARRAY"));
                        give_grammatical_structure(&mut tree.children[2], list_iter);
                    } else {
                        tree.add_child(TreeNode::new(EPSLON));
                        return;
                    }
                }
            },
            "METHOD" => {                
                tree.add_child(TreeNode::new("ID"));            
                give_grammatical_structure(&mut tree.children[0], list_iter);

                tree.add_child(TreeNode::new("("));

                tree.add_child(TreeNode::new("INHERITANCE"));
                give_grammatical_structure(&mut tree.children[2], list_iter);
                
                tree.add_child(TreeNode::new("ARGUMENT"));
                give_grammatical_structure(&mut tree.children[3], list_iter);

                tree.add_child(TreeNode::new(")"));

                tree.add_child(TreeNode::new("BLOC_COM"));
                give_grammatical_structure(&mut tree.children[5], list_iter);
            },
            "ARGUMENT" => {
                tree.add_child(TreeNode::new("TYPE"));
                give_grammatical_structure(&mut tree.children[0], list_iter);

                tree.add_child(TreeNode::new("VAR"));
                give_grammatical_structure(&mut tree.children[1], list_iter);

                tree.add_child(TreeNode::new("ARG_LIST"));
                give_grammatical_structure(&mut tree.children[2], list_iter);
            },
            "ARG_LIST" => {
                if let Some(_) = list_iter.next(){
                    tree.add_child(TreeNode::new(","));
                
                    tree.add_child(TreeNode::new("ARGUMENT"));
                    give_grammatical_structure(&mut tree.children[1], list_iter);
                }
                else{
                    tree.add_child(TreeNode::new(EPSLON));
                    return;
                }                    
            },
            "BLOC_COM" => {
                tree.add_child(TreeNode::new("{"));

                tree.add_child(TreeNode::new("COM_LIST"));
                give_grammatical_structure(&mut tree.children[1], list_iter);

                tree.add_child(TreeNode::new("}"));
            },
            "BLOC" => {
                if let Some(node_item) = list_iter.next(){
                    if node_item.value == "BLOC_COM" {

                        tree.add_child(TreeNode::new("COM_LIST"));
                        give_grammatical_structure(&mut tree.children[0], list_iter);

                    }else if node_item.value == "COMMAND"{

                        tree.add_child(TreeNode::new("COM_LIST"));
                        give_grammatical_structure(&mut tree.children[0], list_iter);

                        tree.add_child(TreeNode::new(";"));
                    }
                }
            },
            "COM_LIST" => {
                if let Some(_) = list_iter.next(){
                    tree.add_child(TreeNode::new("COMMAND"));
                    give_grammatical_structure(&mut tree.children[0], list_iter);

                    tree.add_child(TreeNode::new("COM_LIST"));
                    give_grammatical_structure(&mut tree.children[1], list_iter);
                }else{
                    tree.add_child(TreeNode::new(EPSLON));
                    return;
                }
            },
            "COMMAND" => {
            if let Some(node_item) = list_iter.next(){
                if node_item.value == "ATRIB"{
                    tree.add_child(TreeNode::new("ATRIB"));
                    give_grammatical_structure(&mut tree.children[0], list_iter);

                    tree.add_child(TreeNode::new(";"));
                }else if node_item.value == "while"{
                    tree.add_child(TreeNode::new("while"));

                    tree.add_child(TreeNode::new("("));

                    tree.add_child(TreeNode::new("EXP_LOGIC"));
                    give_grammatical_structure(&mut tree.children[2], list_iter);

                    tree.add_child(TreeNode::new(")"));

                    tree.add_child(TreeNode::new("BLOC"));
                    give_grammatical_structure(&mut tree.children[4], list_iter);

                }else if node_item.value == "do"{
                    tree.add_child(TreeNode::new("do"));

                    tree.add_child(TreeNode::new("BLOC"));
                    give_grammatical_structure(&mut tree.children[1], list_iter);

                    tree.add_child(TreeNode::new("while"));

                    tree.add_child(TreeNode::new("("));

                    tree.add_child(TreeNode::new("EXP_LOGIC"));
                    give_grammatical_structure(&mut tree.children[4], list_iter);

                    tree.add_child(TreeNode::new(")"));

                    tree.add_child(TreeNode::new(";"));

                }else if node_item.value == "if"{
                    tree.add_child(TreeNode::new("if"));

                    tree.add_child(TreeNode::new("("));

                    tree.add_child(TreeNode::new("EXP_LOGIC"));
                    give_grammatical_structure(&mut tree.children[2], list_iter);

                    tree.add_child(TreeNode::new(")"));                   

                    tree.add_child(TreeNode::new("BLOC"));
                    give_grammatical_structure(&mut tree.children[4], list_iter);

                    tree.add_child(TreeNode::new("ELSE"));
                    give_grammatical_structure(&mut tree.children[5], list_iter);

                }else if node_item.value == "for"{
                    tree.add_child(TreeNode::new("for"));

                    tree.add_child(TreeNode::new("("));

                    tree.add_child(TreeNode::new("EXP_LOGIC"));
                    give_grammatical_structure(&mut tree.children[2], list_iter);

                    tree.add_child(TreeNode::new(")"));                   

                    tree.add_child(TreeNode::new("BLOC"));
                    give_grammatical_structure(&mut tree.children[4], list_iter);

                }else if node_item.value == "switch"{
                    tree.add_child(TreeNode::new("switch"));

                    tree.add_child(TreeNode::new("("));

                    tree.add_child(TreeNode::new("ID"));
                    give_grammatical_structure(&mut tree.children[2], list_iter);

                    tree.add_child(TreeNode::new("NOME"));
                    give_grammatical_structure(&mut tree.children[3], list_iter);

                    tree.add_child(TreeNode::new(")"));

                    tree.add_child(TreeNode::new("{"));          

                    tree.add_child(TreeNode::new("SWITCH_CASE"));
                    give_grammatical_structure(&mut tree.children[6], list_iter);
                    
                    tree.add_child(TreeNode::new("}"));   
                }else if node_item.value == "break"{
                    tree.add_child(TreeNode::new("break"));

                    tree.add_child(TreeNode::new(";"));

                }else if node_item.value == "continue"{
                    tree.add_child(TreeNode::new("continue"));

                    tree.add_child(TreeNode::new(";"));

                }else if node_item.value == "return"{
                    tree.add_child(TreeNode::new("break"));

                    tree.add_child(TreeNode::new("EXP"));
                    give_grammatical_structure(&mut tree.children[1], list_iter);

                    tree.add_child(TreeNode::new(";"));
                }
            }
            },
            "ATRIB" => {
                tree.add_child(TreeNode::new("ID"));
                give_grammatical_structure(&mut tree.children[0], list_iter);

                tree.add_child(TreeNode::new("NAME"));
                give_grammatical_structure(&mut tree.children[1], list_iter);

                tree.add_child(TreeNode::new("="));

                tree.add_child(TreeNode::new("EXP"));
                give_grammatical_structure(&mut tree.children[3], list_iter);
            },
            "ELSE" => {
                if let Some(_) = list_iter.next(){
                    tree.add_child(TreeNode::new("else"));

                    tree.add_child(TreeNode::new("BLOC"));
                    give_grammatical_structure(&mut tree.children[1], list_iter);

                }else{
                    tree.add_child(TreeNode::new(EPSLON));
                    return;
                }
            },
            "FOR_EXP" => {
                if let Some(node_item) = list_iter.next(){
                    if node_item.value == "DECL_VAR" {
                        tree.add_child(TreeNode::new("DECL_VAR"));
                        give_grammatical_structure(&mut tree.children[0], list_iter);

                        tree.add_child(TreeNode::new(";"));

                        tree.add_child(TreeNode::new("EXP_LOGIC"));
                        give_grammatical_structure(&mut tree.children[2], list_iter);

                        tree.add_child(TreeNode::new(";"));

                        tree.add_child(TreeNode::new("ATRIB"));
                        give_grammatical_structure(&mut tree.children[4], list_iter);
                    }else if node_item.value == "TYPE"{
                        tree.add_child(TreeNode::new("TYPE"));
                        give_grammatical_structure(&mut tree.children[0], list_iter);

                        tree.add_child(TreeNode::new("ID"));
                        give_grammatical_structure(&mut tree.children[1], list_iter);

                        tree.add_child(TreeNode::new(":"));

                        tree.add_child(TreeNode::new("ID"));
                        give_grammatical_structure(&mut tree.children[3], list_iter);

                        tree.add_child(TreeNode::new("NAME"));
                        give_grammatical_structure(&mut tree.children[4], list_iter);
                    }
                }
            },
            "SWITCH_CASE" => {
                if let Some(node_item) = list_iter.next(){
                    if node_item.value == "case" {
                        tree.add_child(TreeNode::new("case"));

                        tree.add_child(TreeNode::new("CONST"));
                        give_grammatical_structure(&mut tree.children[1], list_iter);

                        tree.add_child(TreeNode::new(":"));

                        tree.add_child(TreeNode::new("BLOC"));
                        give_grammatical_structure(&mut tree.children[3], list_iter);

                        tree.add_child(TreeNode::new("SWITCH_CASE"));
                        give_grammatical_structure(&mut tree.children[4], list_iter);
                    }else if node_item.value == "default"{
                        tree.add_child(TreeNode::new("case"));

                        tree.add_child(TreeNode::new("BLOC"));
                        give_grammatical_structure(&mut tree.children[1], list_iter);
                    }
                }
            },
            "EXP" => {
                if let Some(node_item) = list_iter.next(){
                    if node_item.value == "case" {

                        tree.add_child(TreeNode::new("EXP_MATH"));
                        give_grammatical_structure(&mut tree.children[0], list_iter);

                    } else if node_item.value == "case" {

                        tree.add_child(TreeNode::new("EXP_LOGIC"));
                        give_grammatical_structure(&mut tree.children[0], list_iter);

                    } else if node_item.value == "case" {

                        tree.add_child(TreeNode::new("OPERATOR"));
                        give_grammatical_structure(&mut tree.children[0], list_iter);

                        tree.add_child(TreeNode::new("ID"));
                        give_grammatical_structure(&mut tree.children[1], list_iter);

                        tree.add_child(TreeNode::new("NAME"));
                        give_grammatical_structure(&mut tree.children[2], list_iter);
                        
                    }else if node_item.value == "case" {
                        
                        tree.add_child(TreeNode::new("new"));

                        tree.add_child(TreeNode::new("TYPE"));
                        give_grammatical_structure(&mut tree.children[1], list_iter);

                        tree.add_child(TreeNode::new("NAME"));
                        give_grammatical_structure(&mut tree.children[2], list_iter);

                    }
                }
                
            },
            "OPERATOR" => {
                if let Some(node_item) = list_iter.next() { 
                    if node_item.value == "++" || node_item.value == "--" {
                        tree.add_child(TreeNode::new(&node_item.value));
                    }
                }
            },
            "PARAMS" => {
                if let Some(_) = list_iter.next(){
                    tree.add_child(TreeNode::new("PARAM"));
                    give_grammatical_structure(&mut tree.children[0], list_iter);

                    tree.add_child(TreeNode::new("PARAM_LIST"));
                    give_grammatical_structure(&mut tree.children[1], list_iter);
                }else{
                    tree.add_child(TreeNode::new(EPSLON));
                    return;
                }
            },
            "PARAM_LIST" => {
                if let Some(_) = list_iter.next(){
                    tree.add_child(TreeNode::new(","));

                    tree.add_child(TreeNode::new("PARAM"));
                    give_grammatical_structure(&mut tree.children[1], list_iter);

                    tree.add_child(TreeNode::new("PARAM_LIST"));
                    give_grammatical_structure(&mut tree.children[2], list_iter);
                }else{
                    tree.add_child(TreeNode::new(EPSLON));
                    return;
                }
            },
            "EXP_LOGIC" => {
            },
            "EXP_MATH" => {
                !
            },
            "OP_LOGIC" => {
                if let Some(node_item) = list_iter.next() { 
                    if node_item.value == ">" || node_item.value == "<" || node_item.value == ">=" || node_item.value == "<=" || node_item.value == "==" || node_item.value == "!=" {
                        tree.add_child(TreeNode::new(&node_item.value));
                    }
                }
            },
            "OP_MATH" => { 
                if let Some(node_item) = list_iter.next(){
                    let teste = list_iter.next().next();
                    if node_item.value == "+" || node_item.value == "-" || node_item.value == "*" || node_item.value == "/" {
                        tree.add_child(TreeNode::new(&node_item.value));
                    }
                }
            },
            "PARAM" => {
                !
            },
            "ARRAY_SIZE" => {
                if let Some(_) = list_iter.next(){
                    tree.add_child(TreeNode::new("["));

                    tree.add_child(TreeNode::new("EXP_MATH"));
                    give_grammatical_structure(&mut tree.children[1], list_iter);

                    tree.add_child(TreeNode::new("]"));

                    tree.add_child(TreeNode::new("ARRAY_SIZE"));
                    give_grammatical_structure(&mut tree.children[3], list_iter);
                }else{
                    tree.add_child(TreeNode::new(EPSLON));
                    return;
                }
            },
            "NAME" => {
                while let Some(node_item) = list_iter.next() { // Isso é apenas um stop do processamento, basta remover essa parte e implementar o resto
                    tree.add_child(TreeNode::new(&node_item.value));
                }
            },
            "FIELD" => {
                if let Some(_) = list_iter.next(){
                    tree.add_child(TreeNode::new("."));

                    tree.add_child(TreeNode::new("ID"));
                    give_grammatical_structure(&mut tree.children[1], list_iter);

                    tree.add_child(TreeNode::new("NAME"));
                    give_grammatical_structure(&mut tree.children[2], list_iter);
                }else{
                    tree.add_child(TreeNode::new(EPSLON));
                    return;
                }
            },
            "CONST" => {
                !
            },
            "ID" => {
                if let Some(node_item) = list_iter.next() { 
                    tree.add_child(TreeNode::new(&node_item.value));
                    return;
                }
            },
            "NUMBER" => {
                !
            }
            _ => tree.add_child(TreeNode::new(EPSLON)),
        }
}

fn main() -> std::io::Result<()> {
    let mut list: LinkedList<Node> = LinkedList::new();
    
    let mut contents = read_file("./test.jaca")?; 

    let strings = separate_file_content(&contents); // Separando as strings do arquivo em tokens
    println!("{:?}", strings);

    // Transformando Objeto String em literal &str para facilitar comparação
    let parsed_strings: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();
    
    let mut tree: TreeNode<&str> = TreeNode::new("PROGRAM");

    for value in parsed_strings {
        // Classificando valores no tipo de token
        let token = classificate_value(value);

        list.push_back(Node {
            value: value.to_string(),
            token
        });
    }

    let mut list_iter = list.iter_mut();

    // Chama a função para iniciar a análise gramatical
    give_grammatical_structure(&mut tree, &mut list_iter);
    tree.list();

    // print_linked_list(&list);

    Ok(())
}