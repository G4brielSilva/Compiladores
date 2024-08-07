use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use crate::Row;
use crate::Node;
use crate::Token;

pub fn super_split(content: &String) -> Vec<&str> {
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

pub fn separate_file_content(content: &String) -> Vec<String> {

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

pub fn read_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn is_string_alphanumeric_or_underscore(s: &str) -> bool {
    s.chars().all(|c| c.is_alphanumeric() || c == '_')
}

pub fn is_char_alphanumeric_or_underscore(c: &char) -> bool {
    c.is_alphanumeric() || *c == '_'
}

pub fn is_logic_operator(s: &str) -> bool {
    s == "==" || s == ">=" || s == "<=" || s == "!=" || s == "<" || s == ">"
}

pub fn is_math_operator(s: &str) -> bool {
    s == "++" || s == "--"
}

pub fn contains_an_operator_or_equal(s: &str) -> bool {
    s.contains("++") || s.contains("--") || s.contains("+") || s.contains("-") || s.contains("*") || s.contains("/") || s.contains("==") || s.contains("!=") || s.contains(">=") || s.contains("<=") || s.contains(">") || s.contains("<") || s.contains("=")
}

pub fn break_by_operator_or_equal(s: &str) -> Vec<String> {
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

pub fn break_token(token: &str) -> Vec<String> {
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

pub fn valid_numeric_value(s: &str) -> bool {
    let re = Regex::new(r"^\d+(\.\d+)?$").unwrap();
    re.is_match(s)
}

pub fn valid_string_value(s: &str) -> bool {
    let re = regex::Regex::new(r#""([^"]*)""#).unwrap();
    re.is_match(s)
}

pub fn valid_char_value(s: &str) -> bool {
    let re = regex::Regex::new(r#"'([^']*)'"#).unwrap();
    re.is_match(s)
}

pub fn has_more_then_one_decimal_point(s: &str) -> bool {
    let re = Regex::new(r"^[^.]*\..*\..*$").unwrap();
    re.is_match(s)
}

pub fn is_string_with_more_then_one_decimal_point_numeric(s: &str) -> bool {
    let re = Regex::new(r"^\d+(\.\d+)?$").unwrap();   
    re.is_match(s)
}

pub fn check_final_token<'a>(id:usize, list: &'a [Node]) -> bool{
    if id >= list.len() {
        panic!("Final de programa inesperado");
    }
    return true;
}

pub fn find_on_table_by(table: &Vec<Row>, value: &str, field: &str) -> Vec<Row> {
    let mut rows: Vec<Row> = Vec::new();

    for row in table.iter().cloned() {
        if field == "name" && row.name == value {
            rows.push(row.clone());
        }

        if field == "classification" && row.classification == value {
            rows.push(row.clone());
        }

        if field == "data_type" && row.data_type == value {
            rows.push(row.clone());
        }

        if field == "scope" && row.scope == value {
            rows.push(row.clone());
        }

        if field == "qtd" && row.qtd == value.parse::<u32>().unwrap() {
            rows.push(row.clone());
        }

        if field == "ord" && row.ord == value.parse::<u32>().unwrap() {
            rows.push(row.clone());
        }
    }
    return rows;
}

pub fn classificate_identifier_number_or_error(value: &str) -> Token {
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

pub fn is_valid_const_value(s: &str) -> bool {
    let token = classificate_identifier_number_or_error(s);
    matches!(token, Token::Identifier | Token::Number) || s.contains("'") || s.contains('"')
}

pub fn classificate_value(value: &str) -> Token {
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