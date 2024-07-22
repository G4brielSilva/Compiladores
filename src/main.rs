mod enum_token;
mod node;
mod tree;
mod linked_list;

use tree::TreeNode;
use enum_token::Token;
use node::Node;
//use linked_list::LinkedList; !!review
//use linked_list::ListNode; !!review
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

const EPSLON: &str = "ε";

fn super_split(content: &String) -> Vec<&str> {
    let re = Regex::new(r"([,;: \n()--\+\+])").unwrap();

    // Dividir a string e manter os delimitadores
    let mut parts: Vec<&str> = Vec::new();
    let mut last_end = 0;

    for mat in re.find_iter(&content) {
        let start = mat.start();
        let end = mat.end();

        // Adicionar a parte da string antes do delimitador
        if start != last_end {
            parts.push(&content[last_end..start]);
        }

        // Adicionar o delimitador
        if &content[start..end] != "\n" && &content[start..end] != " " {
            parts.push(&content[start..end]);
        }

        last_end = end;
    }

    // Adicionar a parte final da string, se houver
    if last_end < content.len() {
        parts.push(&content[last_end..]);
    }

    // Imprimir as partes divididas
    return parts;
}

fn separate_file_content(content: &String) -> Vec<String> {

    let subs = super_split(content)
        .clone()
        .into_iter()
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

//fn is_string_alphanumeric_or_underscore(s: &str) -> bool {
 //   s.chars().all(|c| c.is_alphanumeric() || c == '_')
//} !!review

fn is_char_alphanumeric_or_underscore(c: &char) -> bool {
    c.is_alphanumeric() || *c == '_'
}

fn is_logic_operator(s: &str) -> bool {
    s == "==" || s == ">=" || s == "<=" || s == "!=" || s == "<" || s == ">"
}

fn is_math_operator(s: &str) -> bool {
    s == "++" || s == "--"
}

fn contains_an_operator_or_equal(s: &str) -> bool {
    s.contains("++") || s.contains("--") || s.contains("+") || s.contains("-") || s.contains("*") || s.contains("/") || s.contains("==") || s.contains("!=") || s.contains(">=") || s.contains("<=") || s.contains(">") || s.contains("<") || s.contains("=")
}

fn break_by_operator_or_equal(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '+' | '-' | '*' | '/' | '=' | '!' | '>' | '<' => {
                if !current.is_empty() {
                    result.push(current.clone());
                    current.clear();
                }
                current.push(c);
                if let Some(&next) = chars.peek() {
                    if (c == '+' && next == '+')
                        || (c == '-' && next == '-')
                        || (c == '=' && next == '=')
                        || (c == '!' && next == '=')
                        || (c == '>' && next == '=')
                        || (c == '<' && next == '=')
                    {
                        current.push(chars.next().unwrap());
                    }
                }
                result.push(current.clone());
                current.clear();
            }
            _ => {
                if !current.is_empty() && "+-*/=!><".contains(current.chars().last().unwrap()) {
                    result.push(current.clone());
                    current.clear();
                }
                current.push(c);
            }
        }
    }

    if !current.is_empty() {
        result.push(current);
    }

    result
}

fn break_token(token: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let mut string = String::new();
    if is_math_operator(token) || is_logic_operator(token) || valid_numeric_value(token) || (has_more_then_one_decimal_point(token) && is_string_with_more_then_one_decimal_point_numeric(token)) || valid_string_value(token) || valid_char_value(token) {
        result.push(token.to_string());
        return result;
    }

    if contains_an_operator_or_equal(token) {
        let breaked_token = break_by_operator_or_equal(token);
        
        for bk in breaked_token {
            result.push(bk.to_string());
        }
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

fn is_string_with_more_then_one_decimal_point_numeric(s: &str) -> bool {
    let re = Regex::new(r"^\d+(\.\d+)?$").unwrap();   
    re.is_match(s)
}

fn classificate_identifier_number_or_error(value: &str) -> Token {
    if valid_numeric_value(value) {
        return Token::Number
    }

    if is_string_alphanumeric_or_underscore(value) {
        return Token::Identifier
    }

    if has_more_then_one_decimal_point(value) {
        return Token::Error
    }
    return Token::Error;
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
        "{" | "}" => Token::Bloc,
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

fn is_valid_const_value(s: &str) -> bool {
    let token = classificate_identifier_number_or_error(s);
    matches!(token, Token::Identifier | Token::Number) || s.contains("'") || s.contains('"')
}

fn ggsv<'a>(tree: &mut TreeNode<&'a str>, list: &'a [Node], index: usize) -> usize {
    let mut id = index;
    // println!("{} {}",list[id].value, tree.value);
    
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
        "DECLARATION" => {
            tree.add_child("STRUCT");
            id = ggsv(&mut tree.children[0], list, id);
            
            tree.add_child("ID");            
            id = ggsv(&mut tree.children[1], list, id);

            tree.add_child("INHERITANCE");
            id = ggsv(&mut tree.children[2], list, id);

            if list[id].value == "{" {
               tree.add_child("{");
               id+=1;
            } else {
                panic!("Erro: Token inesperado");
            }
            tree.add_child("ITEM_DECLS");
            id = ggsv(&mut tree.children[4], list, id);
            
            if list[id].value == "{" {
               tree.add_child("{");
               id+=1;
            } else {
                panic!("Erro: Token inesperado");
            }
            
            return id;
        },
        "STRUCT" => {
            if list[id].value == "abstract" || list[id].value == "concrete" {
                tree.add_child("INSTANCE");
                id = ggsv(&mut tree.children[0], list, id);
                if list[id].value == "class" {
                    tree.add_child("class");
                    id+=1;
                } else {
                    panic!("Erro: Token inesperado");
                }
                return id;
            } else if list[id].value == "interface" {
                tree.add_child(&list[id].value);
                return id+1;
            } else {
                panic!("Erro: Token inesperado");
            }
        },
        "INSTANCE" => {
            if list[index].value == "abstract" || list[index].value == "concrete" {
                tree.add_child(&list[index].value);
                return id+1;
            }else {
                panic!("Erro: Token inesperado");
            }
        },
        "INHERITANCE" => {
            if list[id].value == "extends" {

                tree.add_child("extends");
                id +=1;
                
                tree.add_child("ID");
                id = ggsv(&mut tree.children[1], list, id);
                return id;
            } else if list[id].value == "implements" {
                tree.add_child("implements");
                id +=1;
                tree.add_child("ID");
                ggsv(&mut tree.children[1], list, id);
                return id;
            }
            tree.add_child(EPSLON);
            return id;
        },
        "ITEM_DECLS" => {
            if list.len() > id && ((list[id].value == "public" || list[id].value == "private" || list[id].value == "protected")) {
                tree.add_child("VISIBILITY");
                id = ggsv(&mut tree.children[0], list, id);

                tree.add_child("SCOPE");
                id = ggsv(&mut tree.children[1], list, id);

                tree.add_child("FINAL");
                id = ggsv(&mut tree.children[2], list, id);

                tree.add_child("ITEM_DECL");
                id = ggsv(&mut tree.children[3], list, id);

                // !!!! voltar
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
            } else {
                panic!("Erro: Token inesperado");
            }
        },
        "SCOPE" => {
            if list[id].value == "static" || list[id].value == "local" {
                tree.add_child(&list[id].value);
                return id+1;
            }else {
                panic!("Erro: Token inesperado");
            }
        },
        "FINAL" => {
            if list[id].value == "final" || list[id].value == "base" {
                tree.add_child(&list[id].value);
                return id+1;
            }else {
                panic!("Erro: Token inesperado");
            }
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

            if list[id].value == ";" {
                tree.add_child(";");
                id+=1;
            } else {
                panic!("Erro: Token inesperado");
            }
            
            return id;
        },
        "TYPE" => {
            if list[id].value == "int" || list[id].value == "float" || list[id].value == "double" || list[id].value == "char" || list[id].value == "void" {
                tree.add_child(&list[id].value);
                return id+1;
            } else {
                tree.add_child("ID");
                id = ggsv(&mut tree.children[0], list, id);

                tree.add_child("NAME");
                id = ggsv(&mut tree.children[1], list, id);
                return id;
            }
            return id;
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
                id += 1;

                tree.add_child("EXP");
                id = ggsv(&mut tree.children[1], list, id);
                return id;
            }
            tree.add_child(EPSLON);
            return id;
        },
        "VAR_LIST" => {
            if list[id].value == "," {
                tree.add_child(",");
                id += 1;
                tree.add_child("VAR");
                id = ggsv(&mut tree.children[1], list, id);

                tree.add_child("VAR_LIST");
                id = ggsv(&mut tree.children[2], list, id);
                return id;
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
        "METHOD_DECL" => {
            if list[id].value == "abstract" || list[id].value == "concrete" {
                tree.add_child("INSTANCE");
                id = ggsv(&mut tree.children[0], list, id);    

                tree.add_child("TYPE");
                id = ggsv(&mut tree.children[1], list, id);
                
                tree.add_child("METHOD");
                id = ggsv(&mut tree.children[2], list, id);
            }
            return id;
        },
        "METHOD" => {
            tree.add_child("ID");
            id = ggsv(&mut tree.children[0], list, id);

            tree.add_child("(");
            id += 1;
            
            tree.add_child("ARGUMENT");
            id = ggsv(&mut tree.children[2], list, id);

            tree.add_child(")");
            id += 1;

            tree.add_child("BLOC_COM");
            id = ggsv(&mut tree.children[4], list, id);
            return id;
        },
        "ARGUMENT" => {
            tree.add_child("TYPE");
            tree.add_child("VAR");
            tree.add_child("ARG_LIST");

            if list[id].value != ")" {
                id = ggsv(&mut tree.children[0], list, id);
                id = ggsv(&mut tree.children[1], list, id);
                id = ggsv(&mut tree.children[2], list, id);
            }

            return id;
        },
        "ARG_LIST" => {
            if list[id].value == "," {
                tree.add_child(",");
                id += 1;
                tree.add_child("ARGUMENT");
                id = ggsv(&mut tree.children[1], list, id);
                return id;
            }
            tree.add_child(EPSLON);
            return id;
        },
        "BLOC_COM" => {
            tree.add_child("{");
            id += 1;

            tree.add_child("COM_LIST");
            id = ggsv(&mut tree.children[1], list, id);

            tree.add_child("}");
            return id+1;
        },
        "COM_LIST" => {
            if list[id].value != "}" {
                tree.add_child("COMMAND");
                id = ggsv(&mut tree.children[0], list, id);

                tree.add_child("COM_LIST");
                id = ggsv(&mut tree.children[1], list, id);
                return id;
            }
            tree.add_child(EPSLON);
            return id;
        }
        "BLOC" => {
            if list[id].value == "{" {
                tree.add_child("BLOC_COM");
                id = ggsv(&mut tree.children[0], list, id);
            } else {
                tree.add_child("COMMAND");
                id = ggsv(&mut tree.children[0], list, id);
            }
            return id;
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

                    tree.add_child("BLOC");
                    id = ggsv(&mut tree.children[4], list, id);
                },
                "do" => {
                    tree.add_child("do");
                    id += 1;

                    tree.add_child("BLOC");
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

                    tree.add_child("BLOC");
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

                    tree.add_child("BLOC");
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
                    id+=1
                },
                "continue" => {
                    tree.add_child("continue");
                    id += 1;

                    tree.add_child(";");
                    id+=1;
                },
                "return" => {
                    tree.add_child("return");
                    id += 1;

                    tree.add_child("EXP");
                    id = ggsv(&mut tree.children[1], list, id);

                    tree.add_child(";");
                    id+=1;
                },
                _ => {
                    tree.add_child("ATRIB");
                    id = ggsv(&mut tree.children[0], list, id);

                    tree.add_child(";");
                    id+=1;
                }
            }
            return id;
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
            if list[id+2].value == ":" {
                tree.add_child("TYPE");
                id = ggsv(&mut tree.children[0], list, id);

                tree.add_child("ID");
                id = ggsv(&mut tree.children[1], list, id);

                tree.add_child(":");
                id += 1;

                tree.add_child("ID");
                id = ggsv(&mut tree.children[3], list, id);

                tree.add_child("NAME");
                id = ggsv(&mut tree.children[4], list, id);
            } else {
                tree.add_child("ATRIB_DECL");
                id = ggsv(&mut tree.children[0], list, id);

                tree.add_child("EXP_LOGIC");
                id = ggsv(&mut tree.children[1], list, id);

                tree.add_child(";");
                id+=1;

                tree.add_child("ATRIB");
                id = ggsv(&mut tree.children[3], list, id);
            }
        
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

                id += 1;
                tree.add_child("TYPE");
                id = ggsv(&mut tree.children[1], list, id);

                tree.add_child("NAME");
                id = ggsv(&mut tree.children[2], list, id);
            } else if list[id].value == "++" || list[id].value == "--" { 
                tree.add_child("OPERATOR");
                id = ggsv(&mut tree.children[0], list, id);

                tree.add_child("ID");
                id = ggsv(&mut tree.children[1], list, id+1);

                tree.add_child("NAME");
                id = ggsv(&mut tree.children[2], list, id+2);
            } else {
                if is_valid_const_value(&list[id].value) || list[id].value == "this" {
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
            return id;
        },
        "PARAM_LIST" => {
            if list[id].value == "," {
                tree.add_child(",");
                id+=1;
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
            return id;
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
            } else if list[id].value != ")" {
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
                id += 1;
                tree.add_child("EXP_MATH");
                id = ggsv(&mut tree.children[1], list, id);

                tree.add_child("]");
                id += 1;

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
                id += 1;

                tree.add_child("PARAMS");
                id = ggsv(&mut tree.children[1], list, id);

                tree.add_child(")");
                id += 1;

                tree.add_child("NAME");
                id = ggsv(&mut tree.children[3], list, id);
                return id;
            } else if list[id].value == "." {
                tree.add_child("FIELD");
                id = ggsv(&mut tree.children[0], list, id);
                return id;
            } else if list[id].value == "[" {
                tree.add_child("ARRAY_SIZE");
                id = ggsv(&mut tree.children[0], list, id);
                
                tree.add_child("NAME");
                id = ggsv(&mut tree.children[1], list, id);

                return id;
            }
            tree.add_child(EPSLON);
            return id;
        },
        "FIELD" => {
            tree.add_child(".");
            id += 1;

            tree.add_child("ID");
            id = ggsv(&mut tree.children[1], list, id);

            tree.add_child("NAME");
            id = ggsv(&mut tree.children[2], list, id);
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
        _ => { 
            tree.add_child(EPSLON);
            return id;
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut list:Vec<Node> = vec![];
    let contents = read_file("./testa.jaca")?;
    
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
    
    // println!("\n >>> LIST <<< \n");

    println!("\n >>> TREE <<< \n");

    // Chama a função para iniciar a análise gramatical
    ggsv(&mut tree, &list, 0);
    tree.list();

    Ok(())
}