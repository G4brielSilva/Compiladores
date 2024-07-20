mod enum_token;
mod node;
mod tree;
mod linked_list;

use tree::TreeNode;
use enum_token::Token;
use node::Node;
use linked_list::LinkedList;
use linked_list::ListNode;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

const EPSLON: &str = "ε";

fn separate_file_content(content: &String) -> Vec<String> {
    let delimiters = vec!['\n', ' ', ';'];
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

fn ggsv<'a>(tree: &mut TreeNode<&'a str>, list: &'a [Node], index: usize) -> usize {
    let mut id = index;
    
    if list.len() <= id {
        tree.add_child(EPSLON);
        return id;
    }
    match tree.value {
        "PROGRAM" => {
            tree.add_child("DECLARATION");
            id = ggsv(&mut tree.children[0], list, id);
            
            tree.add_child("DECLARATIONS");
            id = ggsv(&mut tree.children[1], list, id);
            return id+1;
        }
        "DECLARATION" => {
            tree.add_child("STRUCT");
            id = ggsv(&mut tree.children[0], list, id);
            
            tree.add_child("ID");            
            id = ggsv(&mut tree.children[1], list, id);

            tree.add_child("INHERITANCE");
            id = ggsv(&mut tree.children[2], list, id);
            
            tree.add_child("{");
            id+=1;
            tree.add_child("ITEM_DECLS");
            id = ggsv(&mut tree.children[4], list, id);

            tree.add_child("}");
            return id+1;
        },
        "DECLARATIONS" => {
            if list.len() >= id {
                tree.add_child("DECLARATION");
                id = ggsv(&mut tree.children[0], list, id);

                tree.add_child("DECLARATIONS");
                id = ggsv(&mut tree.children[1], list, id);   

                return id;
            } else {
                tree.add_child(EPSLON);
                return id;
            }
        },
        "STRUCT" => {
            if list[id].value == "abstract" || list[id].value == "concrete" {
                tree.add_child("INSTANCE");
                id = ggsv(&mut tree.children[0], list, id);

                tree.add_child("class");
                return id+1;
            } else if list[id].value == "interface" {
                tree.add_child(&list[id].value);
                return id+1;
            }
            return id;
        },
        "INSTANCE" => {
            if list[index].value == "abstract" || list[index].value == "concrete" {
                tree.add_child(&list[index].value);
                return id+1;
            }
            return id;
        },
        "INHERITANCE" => {
            if list[id].value == "extends" {
                tree.add_child("interface");

                tree.add_child("ID");
                id = ggsv(&mut tree.children[1], list, id);
                return id+1;
            } else if list[id].value == "implements" {
                tree.add_child("implements");

                tree.add_child("ID");
                ggsv(&mut tree.children[1], list, id);
                return id+1;
            }
            tree.add_child(EPSLON);
            return id;
        },
        "ITEM_DECLS" => {
            if list.len() > id && (list[id].value == "public" || list[id].value == "private" || list[id].value == "protected") {
                tree.add_child("VISIBILITY");
                id = ggsv(&mut tree.children[0], list, id);

                tree.add_child("SCOPE");
                id = ggsv(&mut tree.children[1], list, id);

                tree.add_child("FINAL");
                id = ggsv(&mut tree.children[2], list, id);

                tree.add_child("ITEM_DECL");
                id = ggsv(&mut tree.children[3], list, id);

                tree.add_child("ITEM_DECLS");
                id = ggsv(&mut tree.children[4], list, id);
                return id;
            } else {
                tree.add_child(EPSLON);
                return id;
            }
        },
        "VISIBILITY" => {
            if list[id].value == "public" || list[id].value == "protected" || list[id].value == "private" {
                tree.add_child(&list[id].value);
                return id+1;
            }
            return id;
        },
        "SCOPE" => {
            if list[id].value == "static" || list[id].value == "local" {
                tree.add_child(&list[id].value);
                return id+1;
            }
            return id;
        },
        "FINAL" => {
            if list[id].value == "final" || list[id].value == "base" {
                tree.add_child(&list[id].value);
                return id+1;
            }
            return id;
        },
        "ITEM_DECL" => {
            if list[id].value == "abstract" || list[id].value == "concrete"{
                tree.add_child("METHOD_DECL");
                id = ggsv(&mut tree.children[0], list, id);
            } else {
                tree.add_child("ATRIB_DECL");
                id = ggsv(&mut tree.children[0], list, id);
            }
            return id;
        },
        "ATRIB_DECL" => {
            tree.add_child("TYPE");
            id = ggsv(&mut tree.children[0], list, id);

            tree.add_child("VAR");
            id = ggsv(&mut tree.children[1], list, id);

            tree.add_child("VAR_LIST");
            id = ggsv(&mut tree.children[2], list, id);

            tree.add_child(";");
            return id;
        },
        "TYPE" => {
            if list[id].value == "int" || list[id].value == "float" || list[id].value == "double" || list[id].value == "char" || list[id].value == "void" {
                tree.add_child(&list[id].value);
            } else {
                tree.add_child("ID");
                id = ggsv(&mut tree.children[0], list, id);

                tree.add_child("NAME");
                id = ggsv(&mut tree.children[1], list, id);
            }
            return id+1;
        },
        "VAR" => {
            tree.add_child("ID");
            id = ggsv(&mut tree.children[0], list, id);

            tree.add_child("ARRAY");
            id = ggsv(&mut tree.children[1], list, id);

            tree.add_child("VALUE");
            id = ggsv(&mut tree.children[2], list, id);
            return id;
        },
        "VALUE" => {
            if list[id].value == "=" {
                tree.add_child("=");

                tree.add_child("EXP");
                id = ggsv(&mut tree.children[1], list, id+1);
                return id;
            }
            tree.add_child(EPSLON);
            return id;
        },
        "VAR_LIST" => {
            if list[id].value == "," {
                tree.add_child(",");

                tree.add_child("VAR");
                id = ggsv(&mut tree.children[1], list, id+1);

                tree.add_child("VAR_LIST");
                id = ggsv(&mut tree.children[2], list, id+1);
                return id+1;
            }
            tree.add_child(EPSLON);
            return id;

        },
        "ARRAY" => {
            if list[id].value == "[" {
                tree.add_child(&list[id].value);
                id += 1;
                tree.add_child(&list[id].value);
                id += 1;
                tree.add_child("ARRAY");
                id = ggsv(&mut tree.children[2], list, id);
                return id;
            } 

            tree.add_child(EPSLON);
            return id;
        },
        "METHOD" => {
            tree.add_child("ID");
            id = ggsv(&mut tree.children[0], list, id);

            tree.add_child("(");

            tree.add_child("ARGUMENT");
            id = ggsv(&mut tree.children[2], list, id+1);

            tree.add_child(")");

            tree.add_child("BLOC_COM");
            id = ggsv(&mut tree.children[4], list, id+1);
            return id+1;
        },
        "ARGUMENT" => {
            tree.add_child("TYPE");
            id = ggsv(&mut tree.children[0], list, id);

            tree.add_child("VAR");
            id = ggsv(&mut tree.children[1], list, id);

            tree.add_child("ARG_LIST");
            id = ggsv(&mut tree.children[2], list, id);
            return id+1;
        },
        "ARG_LIST" => {
            if list[id].value == "," {
                tree.add_child(",");

                tree.add_child("ARGUMENT");
                id = ggsv(&mut tree.children[1], list, id+1);
                return id+1;
            }
            tree.add_child(EPSLON);
            return id;
        },
        "BLOC_COM" => {
            tree.add_child("{");
            tree.add_child("COM_LIST");
            id = ggsv(&mut tree.children[1], list, id+1);
            tree.add_child("}");
            return id+1;
        },
        "BLOC" => {
            if list[id].value == "{" {
                tree.add_child("BLOC_COM");
                id = ggsv(&mut tree.children[0], list, id);
            } else {
                tree.add_child("COMMAND");
                id = ggsv(&mut tree.children[0], list, id);
                tree.add_child(";");
            }
            return id+1;
        },
        "COMMAND" => {
            match &list[id].value as &str {
                "while" => {
                    tree.add_child("while");
                    id += 1;
                    tree.add_child("(");
                    id += 1;
                    tree.add_child("EXP_LOGIC");
                    id = ggsv(&mut tree.children[2], list, id);

                    tree.add_child(")");
                    id += 1;
                    tree.add_child("BLOCK");
                    id = ggsv(&mut tree.children[4], list, id);
                },
                "do" => {
                    tree.add_child("do");
                    id += 1;
                    tree.add_child("BLOCK");
                    id = ggsv(&mut tree.children[1], list, id);

                    tree.add_child("while");
                    id += 1;
                    tree.add_child("(");
                    id += 1;
                    tree.add_child("EXP_LOGIC");
                    id = ggsv(&mut tree.children[4], list, id);

                    tree.add_child(")");
                    id += 1;
                },
                "if" => {
                    tree.add_child("if");
                    id += 1;
                    tree.add_child("(");
                    id += 1;
                    tree.add_child("EXP_LOGIC");
                    id = ggsv(&mut tree.children[2], list, id);

                    tree.add_child(")");
                    id += 1;
                    tree.add_child("BLOCK");
                    id = ggsv(&mut tree.children[4], list, id);

                    tree.add_child("ELSE");
                    id = ggsv(&mut tree.children[5], list, id);
                },
                "for" => {
                    tree.add_child("for");
                    id += 1;
                    tree.add_child("(");
                    id += 1;
                    tree.add_child("FOR_EXP");
                    id = ggsv(&mut tree.children[2], list, id);

                    tree.add_child(")");
                    id += 1;
                    tree.add_child("BLOCK");
                    id = ggsv(&mut tree.children[4], list, id);
                },
                "switch" => {
                    tree.add_child("switch");
                    id += 1;
                    tree.add_child("(");
                    id += 1;
                    
                    tree.add_child("ID");
                    id = ggsv(&mut tree.children[1], list, id);
                    
                    tree.add_child("NAME");
                    id = ggsv(&mut tree.children[2], list, id);

                    tree.add_child(")");
                    id += 1;

                    tree.add_child("{");
                    id += 1;

                    tree.add_child("SWITCH_CASE");
                    id = ggsv(&mut tree.children[5], list, id );
                    tree.add_child("}");
                    id += 1;
                },
                "break" => {
                    tree.add_child("break");
                    id += 1;
                    tree.add_child(";");
                },
                "continue" => {
                    tree.add_child("continue");
                    id += 1;
                    tree.add_child(";");
                },
                "return" => {
                    tree.add_child("return");
                    
                    id += 1;
                    tree.add_child("EXP");

                    tree.add_child(";");
                },
                _ => {
                    tree.add_child("ATRIB");
                    id = ggsv(&mut tree.children[0], list, id);
                    tree.add_child(";");
                }
            }
        },
        "ATRIB" => {
            tree.add_child("ID");
            id = ggsv(&mut tree.children[0], list, id);

            tree.add_child("NAME");
            id = ggsv(&mut tree.children[1], list, id);

            tree.add_child("=");
            id += 1;
            
            tree.add_child("EXP");
            id = ggsv(&mut tree.children[3], list, id);
            return id;
        },
        "ELSE" => {
            if list[index].value == "else" {
                tree.add_child("else");
                tree.add_child("BLOC");
                id = ggsv(&mut tree.children[1], list, id+1);
                return id;
            } else {
                tree.add_child(EPSLON);
                return id;
            }
        },
        "FOR_EXP" => {
            tree.add_child("ATRIB_DECL");
            id = ggsv(&mut tree.children[0], list, id);
            tree.add_child(";");

            tree.add_child("EXP_LOGIC");
            id = ggsv(&mut tree.children[2], list, id);
            tree.add_child(";");

            tree.add_child("ATRIB");
            id = ggsv(&mut tree.children[4], list, id);
            return id;
        },
        "SWITCH_CASE" => {
            if list[id].value == "case" {
                tree.add_child("case");

                tree.add_child("CONST");
                id = ggsv(&mut tree.children[1], list, id);

                tree.add_child(":");
                tree.add_child("BLOC");
                id = ggsv(&mut tree.children[3], list, id);

                tree.add_child("SWITCH_CASE");
                id = ggsv(&mut tree.children[4], list, id);

            } else if list[id].value == "default" {
                tree.add_child("default");

                tree.add_child("BLOC");
                id = ggsv(&mut tree.children[1], list, id);
            }
            return id;
        },
        "EXP" => {
            if list[id].value == "new" {
                tree.add_child("new");

                tree.add_child("TYPE");
                id = ggsv(&mut tree.children[1], list, id+1);

                tree.add_child("NAME");
                id = ggsv(&mut tree.children[2], list, id+2);
            } else if list[id].value == "++" || list[id].value == "--" { 
                tree.add_child("OPERATOR");
                id = ggsv(&mut tree.children[0], list, id);

                tree.add_child("ID");
                id = ggsv(&mut tree.children[1], list, id+1);

                tree.add_child("NAME");
                id = ggsv(&mut tree.children[2], list, id+2);
            } else {
                let token = classificate_identifier_number_or_error(&list[id].value);
                if matches!(token, Token::Identifier | Token::Number) || list[id].value == "this" {
                    tree.add_child("EXP_MATH");
                    id = ggsv(&mut tree.children[0], list, id);
                } else {
                    tree.add_child("EXP_LOGIC");
                    id = ggsv(&mut tree.children[0], list, id);
                }
            }
            return id;
        },
        "OPERATOR" => {
            if list[id].value == "++" || list[id].value == "--" {
                tree.add_child(&list[id].value);
                return id+1;
            }
            return id;
        },
        "PARAMS" => {
            tree.add_child("PARAM");
            id = ggsv(&mut tree.children[0], list, id);

            tree.add_child("PARAM_LIST");
            id = ggsv(&mut tree.children[1], list, id);
            return id+1;
        },
        "PARAM_LIST" => {
            if list[id].value == "," {
                tree.add_child(",");

                tree.add_child("PARAM");
                id = ggsv(&mut tree.children[1], list, id);

                tree.add_child("PARAM_LIST");
                id = ggsv(&mut tree.children[2], list, id);
                return id;
            }
            tree.add_child(EPSLON);
            return id;
        },
        "EXP_LOGIC" => {
            tree.add_child("EXP");
            id = ggsv(&mut tree.children[0], list, id);

            if list[id].value == ">" || list[id].value == "<" || list[id].value == ">=" || list[id].value == "<=" || list[id].value == "==" || list[id].value == "!=" {
                tree.add_child("OP_LOGIC");
                id = ggsv(&mut tree.children[1], list, id);

                tree.add_child("EXP_LOGIC");
                id = ggsv(&mut tree.children[2], list, id);
            }
            return id+1;
        },
        "EXP_MATH" => {
            tree.add_child("PARAM");
            id = ggsv(&mut tree.children[0], list, id);

            if list[id].value == "+" || list[id].value == "-" || list[id].value == "*" || list[id].value == "/" {
                tree.add_child("OP_MATH");
                id = ggsv(&mut tree.children[1], list, id+1);

                tree.add_child("EXP_MATH");
                id = ggsv(&mut tree.children[2], list, id);
            }
            return id;
        },
        "OP_MATH" | "OP_LOGIC" => {
            tree.add_child(&list[id].value);
            return id+1;
        },
        "PARAM" => {
            if list[id].value == "this" {
                tree.add_child("this");

                tree.add_child("FIELD");
                id = ggsv(&mut tree.children[1], list, id+1);
            } else if list[id].value.parse::<i64>().is_ok() || list[id].value.parse::<f64>().is_ok() || list[id].value.contains('"') || list[id].value.contains("'") { 
                tree.add_child("CONST");
                id = ggsv(&mut tree.children[0], list, id);
            } else {
                tree.add_child("ID");
                id = ggsv(&mut tree.children[0], list, id);

                tree.add_child("NAME");
                id = ggsv(&mut tree.children[1], list, id);
            }
            return id;
        },
        "ARRAY_SIZE" => {
            if list[id].value == "[" {
                tree.add_child("[");
                id = ggsv(&mut tree.children[0], list, id);

                tree.add_child("EXP_MATH");
                id = ggsv(&mut tree.children[1], list, id);

                tree.add_child("]");
                id = ggsv(&mut tree.children[2], list, id);

                tree.add_child("ARRAY_SIZE");
                id = ggsv(&mut tree.children[3], list, id);
                return id;
            }
            tree.add_child(EPSLON);
            return id;
        },
        "NAME" => {
            if list[id].value == "(" {
                tree.add_child("(");
                id = ggsv(&mut tree.children[0], list, id);

                tree.add_child("PARAMS");
                id = ggsv(&mut tree.children[1], list, id);

                tree.add_child(")");
                id = ggsv(&mut tree.children[2], list, id);

                tree.add_child("NAME");
                id = ggsv(&mut tree.children[3], list, id);
                return id;
            } else if list[id].value.contains(".") {
                tree.add_child("FIELD");
                id = ggsv(&mut tree.children[0], list, id);
                return id;
            }
            tree.add_child(EPSLON);
            return id;
        },
        "FIELD" => {
            tree.add_child("ID");
            id = ggsv(&mut tree.children[0], list, id);

            tree.add_child("NAME");
            id = ggsv(&mut tree.children[1], list, id);
            return id;
        },
        "CONST" => {
            if list[id].value.parse::<i64>().is_ok() || list[id].value.parse::<f64>().is_ok() {
                tree.add_child("NUMBER");
                id = ggsv(&mut tree.children[0], list, id);
                return id;
            } else {
                tree.add_child(&list[id].value);
                return id+1;
            }
        },
        "ID" => {
            let result = classificate_identifier_number_or_error(&list[id].value);
            if matches!(result, Token::Identifier) {
                tree.add_child(&list[id].value);
                return id+1;
            }
            return id;
        },
        "NUMBER" => {
            if list[id].value.parse::<i64>().is_ok() || list[id].value.parse::<f64>().is_ok() {
                tree.add_child(&list[id].value);
                return id+1;
            }
            return id;
        },
        "}" => {
            tree.add_child(&list[id].value);
            return id+1;
        }
        _ => { 
            tree.add_child(EPSLON);
            return id;
        }
    }
}

fn main() -> std::io::Result<()> {
    // let mut list: LinkedList<Node> = LinkedList::new();
    let mut list:Vec<Node> = vec![];
    
    let contents = read_file("./test.jaca")?; 

    let strings = separate_file_content(&contents).into_iter().filter(|s| s!= "\r").collect::<Vec<String>>(); // Separando as strings do arquivo em tokens
    println!("{:?}", strings);

    // Transformando Objeto String em literal &str para facilitar comparação
    let parsed_strings: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();
    
    let mut tree: TreeNode<&str> = TreeNode::new("PROGRAM");

    for value in parsed_strings {
        // Classificando valores no tipo de token
        let token = classificate_value(value);

        list.push(Node {
            value: value.to_string(),
            token
        });
    }
    
    println!("\n >>> LIST <<< \n");

    println!("\n >>> TREE <<< \n");

    let test = &list[2];

    // let mut list_iter = list.iter_mut();
    // Chama a função para iniciar a análise gramatical
    ggsv(&mut tree, &list, 0);
    tree.list();

    Ok(())
}