mod enum_token;
mod node;
mod tree;
mod linked_list;
mod table_row;
mod utils;
mod inter_code;

use tree::TreeNode;
use enum_token::Token;
use node::Node;
use table_row::Row;
use utils::*;
use core::panic;
use std::any::type_name;
use inter_code::*;


#[macro_use]
extern crate lazy_static;

use std::{any::Any, sync::Mutex};

lazy_static! {
    static ref SCOPE: Mutex<String> = Mutex::new(String::from("global"));
    static ref ORDEM: Mutex<u32> = Mutex::new(1);
    static ref QTD: Mutex<u32> = Mutex::new(0);
    static ref CLASSSCOPE: Mutex<String> = Mutex::new(String::from("null"));
}

const EPSLON: &str = "ε";

fn type_of<T>(_: &T) -> &str {
    type_name::<T>()
}

fn verifica_tipo<'a>(table: &mut Vec<Row>, list: &'a [Node],id:usize,tipo:String) -> bool{
    for row in table.iter().cloned(){
        if row.name == list[id].value && row.scope==SCOPE.lock().unwrap().to_string(){
            if tipo == row.data_type {
                return true
            }
        }
    }
    return false;
}

fn ggsv<'a>(tree: &mut TreeNode<&'a str>, list: &'a [Node], index: usize, table: &mut Vec<Row>) -> usize {
    let mut id = index;
    // if id<list.len() {
    //     println!("{} {}",list[id].value, tree.value);
    // }    
    if list.len() <= id {
        tree.add_child(EPSLON);
        return id;
    }
    match tree.value {
        "PROGRAM" => {
            tree.add_child("DECLARATION");
            id = ggsv(&mut tree.children[0], list, id,table);
            
            tree.add_child("DECLARATIONS");
            id = ggsv(&mut tree.children[1], list, id,table);
            return id+1;
        }
        "DECLARATIONS" => {
            if list.len() >= id {
                tree.add_child("DECLARATION");
                id = ggsv(&mut tree.children[0], list, id,table);

                tree.add_child("DECLARATIONS");
                id = ggsv(&mut tree.children[1], list, id,table);

                return id;
            } else {
                tree.add_child(EPSLON);
                return id;
            }
        },
        "DECLARATION" => {
            tree.add_child("STRUCT");
            id = ggsv(&mut tree.children[0], list, id,table);
            if list[id-1].value == "class"{
                *CLASSSCOPE.lock().unwrap() = list[id].value.to_string();
            }
            tree.add_child("ID");            
            id = ggsv(&mut tree.children[1], list, id,table);

            tree.add_child("INHERITANCE");
            id = ggsv(&mut tree.children[2], list, id,table);
            let mut old_scope: String = SCOPE.lock().unwrap().to_string();

            if check_final_token(id,list)&& list[id].value == "{"  {
                let name = &list[id-1].value;
                {
                    let mut escopo = SCOPE.lock().unwrap();
                    old_scope = escopo.to_string();
                    *escopo = String::from(name);
                } 
               tree.add_child("{");
               id+=1;
            } else {
                println!("{} {} {}", id, list.len(), list[id].value);
                panic!("Erro: Token inesperado {}", list[id].value);
            }
            tree.add_child("ITEM_DECLS");
            id = ggsv(&mut tree.children[4], list, id,table);
            if check_final_token(id,list)&& list[id].value == "}" {
                {
                    let mut escopo = SCOPE.lock().unwrap();
                    *escopo = String::from(old_scope);
                }
                *CLASSSCOPE.lock().unwrap() = "null".to_string();
               tree.add_child("}");
               id+=1;
            } else {
                panic!("Erro: Token inesperado {}", list[id].value);
            }
            
            return id;
        },
        "STRUCT" => {
            if list[id].value == "abstract" || list[id].value == "concrete" {
                tree.add_child("INSTANCE");
                id = ggsv(&mut tree.children[0], list, id,table);
                if  check_final_token(id,list)&& list[id].value == "class" {
                    tree.add_child("class");
                    id+=1;
                } else {
                    panic!("Erro: Token inesperado {}", list[id].value);
                }
                return id;
            } else if  check_final_token(id,list)&& list[id].value == "interface" {
                tree.add_child(&list[id].value);
                return id+1;
            } else {
                panic!("Erro: Token inesperado {}", list[id].value);
            }
        },
        "INSTANCE" => {
            if check_final_token(id,list)&&(list[index].value == "abstract" || list [index].value == "concrete")  {
                tree.add_child(&list[index].value);
                return id+1;
            }else {
                panic!("Erro: Token inesperado {}", list[id].value);
            }
        },
        "INHERITANCE" => {
            if  check_final_token(id,list)&& list[id].value == "extends" {

                tree.add_child("extends");
                id +=1;
                
                tree.add_child("ID");
                id = ggsv(&mut tree.children[1], list, id,table);
                return id;
            } else if  check_final_token(id,list)&& list[id].value == "implements" {
                tree.add_child("implements");
                id +=1;
                tree.add_child("ID");
                id = ggsv(&mut tree.children[1], list, id,table);
                return id;
            }
            tree.add_child(EPSLON);
            return id;
        },
        "ITEM_DECLS" => {
            if list.len() > id && ((list[id].value == "public" || list[id].value == "private" || list[id].value == "protected")) {
                tree.add_child("VISIBILITY");
                id = ggsv(&mut tree.children[0], list, id,table);

                tree.add_child("SCOPE");
                id = ggsv(&mut tree.children[1], list, id,table);

                tree.add_child("FINAL");
                id = ggsv(&mut tree.children[2], list, id,table);

                tree.add_child("ITEM_DECL");
                id = ggsv(&mut tree.children[3], list, id,table);

                // !!!! voltar
                tree.add_child("ITEM_DECLS");
                id = ggsv(&mut tree.children[4], list, id,table);
                return id;
            } else {
                tree.add_child(EPSLON);
                return id;
            }
        },
        "VISIBILITY" => {
            if check_final_token(id,list)&& (list[id].value == "public" || list[id].value == "protected" || list  [id].value == "private")  {
                tree.add_child(&list[id].value);
                return id+1;
            } else {
                panic!("Erro: Token inesperado {}", list[id].value);
            }
        },
        "SCOPE" => {
            if check_final_token(id,list)&& (list[id].value == "static" || list [id].value == "local")  {
                tree.add_child(&list[id].value);
                return id+1;
            }else {
                panic!("Erro: Token inesperado {}", list[id].value);
            }
        },
        "FINAL" => {
            if check_final_token(id,list)&& (list[id].value == "final" || list [id].value == "base")  {
                tree.add_child(&list[id].value);
                return id+1;
            }else {
                panic!("Erro: Token inesperado {}", list[id].value);
            }
        },
        "ITEM_DECL" => {
            if list[id].value == "abstract" || list[id].value == "concrete"{
                tree.add_child("METHOD_DECL");
                id = ggsv(&mut tree.children[0], list, id,table);
            } else {
                tree.add_child("ATRIB_DECL");
                id = ggsv(&mut tree.children[0], list, id,table);
            }
            return id;
        },
        "ATRIB_DECL" => {
            tree.add_child("TYPE");
            id = ggsv(&mut tree.children[0], list, id,table);

            tree.add_child("VAR");
            id = ggsv(&mut tree.children[1], list, id,table);

            tree.add_child("VAR_LIST");
            id = ggsv(&mut tree.children[2], list, id,table);

            
            if check_final_token(id,list)&& list[id].value == ";"  {
                tree.add_child(";");
                id+=1;
            } else {
                panic!("Erro: Token inesperado {}", list[id].value);
            }
            
            return id;
        },
        "METHOD_DECL" => {
            if list[id].value == "abstract" || list[id].value == "concrete" {
                tree.add_child("INSTANCE");
                id = ggsv(&mut tree.children[0], list, id,table); 

                tree.add_child("TYPE");
                id = ggsv(&mut tree.children[1], list, id,table);
                
                tree.add_child("METHOD");
                id = ggsv(&mut tree.children[2], list, id,table);
            }
            return id;
        },
        "TYPE" => {
            if check_final_token(id,list) && (list[id].value == "int" || list[id].value == "float" || list[id].value == "double" || list[id].value == "char" || list [id].value == "void")  {
                tree.add_child(&list[id].value);
                return id+1;
            } else {
                tree.add_child("ID");
                id = ggsv(&mut tree.children[0], list, id,table);

                tree.add_child("NAME");
                id = ggsv(&mut tree.children[1], list, id,table);
                return id;
            }
        },
        "VAR" => {
            tree.add_child("ID");
            id = ggsv(&mut tree.children[0], list, id,table);

            tree.add_child("ARRAY");
            id = ggsv(&mut tree.children[1], list, id,table);

            tree.add_child("VALUE");
            id = ggsv(&mut tree.children[2], list, id,table);
            return id;
        },
        "VALUE" => {
            if check_final_token(id,list)&& list[id].value == "=" {
                tree.add_child("=");
                id += 1;

                tree.add_child("EXP");
                id = ggsv(&mut tree.children[1], list, id,table);
                return id;
            }
            tree.add_child(EPSLON);
            return id;
        },
        "VAR_LIST" => {
            if check_final_token(id,list)&& list[id].value == ","  {
                tree.add_child(",");
                id += 1;
                tree.add_child("VAR");
                id = ggsv(&mut tree.children[1], list, id,table);

                tree.add_child("VAR_LIST");
                id = ggsv(&mut tree.children[2], list, id,table);
                return id;
            }
            tree.add_child(EPSLON);
            return id;

        },
        "ARRAY" => {
            if check_final_token(id,list)&& list[id].value == "[" {
                if let Some(last) = table.last_mut() {
                    if last.name == list[id-1].value && last.scope == *SCOPE.lock().unwrap() {
                        last.data_type = "array".to_string();
                    }
                }
                tree.add_child(&list[id].value);
                id += 1;
                if check_final_token(id,list)&& list[id].value == "]" {
                    tree.add_child("]");
                    id+=1;
                } else {
                    panic!("Erro: Token inesperado {}", list[id].value);
                }
                tree.add_child("ARRAY");
                id = ggsv(&mut tree.children[2], list, id,table);
                return id;
            } 
            tree.add_child(EPSLON);
            return id;
        },
        "METHOD" => {
            tree.add_child("ID");
            let name = &list[id].value;
            id = ggsv(&mut tree.children[0], list, id,table);
            let old_scope: String;
            {
                let mut escopo = SCOPE.lock().unwrap();
                old_scope = escopo.to_string();
                *escopo = String::from(name);
            }            
            if check_final_token(id,list)&& list[id].value == "(" {
                tree.add_child("(");
                id+=1;
            } else {
                panic!("Erro: Token inesperado {}", list[id].value);
            }
            
            tree.add_child("ARGUMENT");
            id = ggsv(&mut tree.children[2], list, id,table);

            if check_final_token(id,list)&& list[id].value == ")" {
                tree.add_child(")");
                id+=1;
            } else {
                panic!("Erro: Token inesperado {}", list[id].value);
            }
            tree.add_child("BLOC_COM");
            id = ggsv(&mut tree.children[4], list, id,table);
            {
                let mut escopo = SCOPE.lock().unwrap();
                *escopo = String::from(old_scope);
            }
            return id;
        },
        "ARGUMENT" => {
            tree.add_child("TYPE");

            tree.add_child("VAR");
            
            tree.add_child("ARG_LIST");

            if list[id].value != ")" {
                for row in table.iter_mut(){
                    if row.name == *SCOPE.lock().unwrap() {
                        let mut qtd = QTD.lock().unwrap();
                        *qtd = *qtd+1;
                        row.qtd = *qtd;
                        row.classification = "Function".to_string();
                    }
                }                
                id = ggsv(&mut tree.children[0], list, id,table);
                id = ggsv(&mut tree.children[1], list, id,table);
                if let Some(last)= table.last_mut(){ 
                    let mut ordem = ORDEM.lock().unwrap();
                    last.ord = *ordem;
                    *ordem = *ordem+1;
                    last.classification = "Parameter".to_string();
                }
                id = ggsv(&mut tree.children[2], list, id,table);
                let mut ordem = ORDEM.lock().unwrap();
                let mut qtd = QTD.lock().unwrap();
                *ordem = 1;
                *qtd = 0;
            }

            return id;
        },
        "ARG_LIST" => {
            if check_final_token(id,list)&& list[id].value == "," {
                tree.add_child(",");
                id += 1;
                tree.add_child("ARGUMENT");
                id = ggsv(&mut tree.children[1], list, id,table);
                return id;
            }
            tree.add_child(EPSLON);
            return id;
        },
        "BLOC_COM" => {
            if check_final_token(id,list)&& list[id].value == "{" {
                tree.add_child("{");
                id+=1;
            } else {
                panic!("Erro: Token inesperado {}", list[id].value);
            }

            tree.add_child("COM_LIST");
            id = ggsv(&mut tree.children[1], list, id,table);

            if check_final_token(id,list)&& list[id].value == "}" {
                tree.add_child("}");
                return id+1;
            } else {
                panic!("Erro: Token inesperado {}", list[id].value);
            }
        },
        "BLOC" => {
            if list[id].value == "{" {
                tree.add_child("BLOC_COM");
                id = ggsv(&mut tree.children[0], list, id,table);
            } else {
                tree.add_child("COMMAND");
                id = ggsv(&mut tree.children[0], list, id,table);
            }
            return id;
        },
        "COM_LIST" => {
            if list[id].value != "}" {
                tree.add_child("COMMAND");
                id = ggsv(&mut tree.children[0], list, id,table);

                tree.add_child("COM_LIST");
                id = ggsv(&mut tree.children[1], list, id,table);
                return id;
            }
            tree.add_child(EPSLON);
            return id;
        },
        "COMMAND" => {
            match &list[id].value as &str {
                "while" => {
                    tree.add_child("while");
                    id += 1;
                    if check_final_token(id,list)&& list[id].value == "(" {
                        tree.add_child("(");
                        id+=1;
                    } else {
                        panic!("Erro: Token inesperado {}", list[id].value);
                    }
                    tree.add_child("EXP_LOGIC");
                    id = ggsv(&mut tree.children[2], list, id,table);

                    if check_final_token(id,list)&& list[id].value == ")" {
                        tree.add_child(")");
                        id+=1;
                    } else {
                        panic!("Erro: Token inesperado {}", list[id].value);
                    }

                    tree.add_child("BLOC");
                    id = ggsv(&mut tree.children[4], list, id,table);
                },
                "do" => {
                    tree.add_child("do");
                    id += 1;

                    tree.add_child("BLOC");
                    id = ggsv(&mut tree.children[1], list, id,table);

                    if  check_final_token(id,list)&& list[id].value == "while" {
                        tree.add_child("while");
                        id+=1;
                    } else {
                        panic!("Erro: Token inesperado {}", list[id].value);
                    }

                    if check_final_token(id,list)&& list[id].value == "(" {
                        tree.add_child("(");
                        id+=1;
                    } else {
                        panic!("Erro: Token inesperado {}", list[id].value);
                    }

                    tree.add_child("EXP_LOGIC");
                    id = ggsv(&mut tree.children[4], list, id,table);

                    if check_final_token(id,list)&& list[id].value == ")" {
                        tree.add_child(")");
                        id+=1;
                    } else {
                        panic!("Erro: Token inesperado {}", list[id].value);
                    }
                },
                "if" => {
                    tree.add_child("if");
                    id += 1;

                    if check_final_token(id,list)&& list[id].value == "(" {
                        tree.add_child("(");
                        id+=1;
                    } else {
                        panic!("Erro: Token inesperado {}", list[id].value);
                    }

                    tree.add_child("EXP_LOGIC");
                    id = ggsv(&mut tree.children[2], list, id,table);

                    if check_final_token(id,list)&& list[id].value == ")" {
                        tree.add_child(")");
                        id+=1;
                    } else {
                        panic!("Erro: Token inesperado {}", list[id].value);
                    }

                    tree.add_child("BLOC");
                    id = ggsv(&mut tree.children[4], list, id,table);

                    tree.add_child("ELSE");
                    id = ggsv(&mut tree.children[5], list, id,table);
                },
                "for" => {
                    tree.add_child("for");
                    id += 1;

                    if list[id].value == "(" && check_final_token(id,list){
                        tree.add_child("(");
                        id+=1;
                    } else {
                        panic!("Erro: Token inesperado {}", list[id].value);
                    }

                    tree.add_child("FOR_EXP");
                    id = ggsv(&mut tree.children[2], list, id,table);

                    if list[id].value == ")" && check_final_token(id,list){
                        tree.add_child(")");
                        id+=1;
                    } else {
                        panic!("Erro: Token inesperado {}", list[id].value);
                    }

                    tree.add_child("BLOC");
                    id = ggsv(&mut tree.children[4], list, id,table);
                },
                "switch" => {
                    tree.add_child("switch");
                    id += 1;

                    if check_final_token(id,list)&& list[id].value == "(" {
                        tree.add_child("(");
                        id+=1;
                    } else {
                        panic!("Erro: Token inesperado {}", list[id].value);
                    }
                    
                    tree.add_child("ID");
                    id = ggsv(&mut tree.children[1], list, id,table);
                    
                    tree.add_child("NAME");
                    id = ggsv(&mut tree.children[2], list, id,table);

                    if check_final_token(id,list)&& list[id].value == ")" {
                        tree.add_child(")");
                        id+=1;
                    } else {
                        panic!("Erro: Token inesperado {}", list[id].value);
                    }

                    if check_final_token(id,list)&& list[id].value == "{"  {
                        tree.add_child("{");
                        id+=1;
                    } else {
                        panic!("Erro: Token inesperado {}", list[id].value);
                    }

                    tree.add_child("SWITCH_CASE");
                    id = ggsv(&mut tree.children[5], list, id ,table);

                    if check_final_token(id,list)&& list[id].value == "}" {
                        tree.add_child("}");
                        id+=1;
                    } else {
                        panic!("Erro: Token inesperado {}", list[id].value);
                    }
                },
                "break" => {
                    tree.add_child("break");
                    id += 1;

                    if check_final_token(id,list)&& list[id].value == ";" {
                        tree.add_child(";");
                        id+=1;
                    } else {
                        panic!("Erro: Token inesperado {}", list[id].value);
                    }
                },
                "continue" => {
                    tree.add_child("continue");
                    id += 1;

                    if check_final_token(id,list)&& list[id].value == ";" {
                        tree.add_child(";");
                        id+=1;
                    } else {
                        panic!("Erro: Token inesperado {}", list[id].value);
                    }
                },
                "return" => {
                    tree.add_child("return");
                    id += 1;

                    tree.add_child("EXP");
                    id = ggsv(&mut tree.children[1], list, id,table);

                    if list[id].value == ";" && check_final_token(id,list) {
                        tree.add_child(";");
                        id+=1;
                    } else {
                        panic!("Erro: Token inesperado {}", list[id].value);
                    }
                },
                _ => {
                    tree.add_child("ATRIB");
                    id = ggsv(&mut tree.children[0], list, id,table);

                    if check_final_token(id,list)&& list[id].value == ";" {
                        tree.add_child(";");
                        id+=1;
                    } else {
                        panic!("Erro: Token inesperado {}", list[id].value);
                    }
                }
            }
            return id;
        },
        "ATRIB" => {
            tree.add_child("ID");
            id = ggsv(&mut tree.children[0], list, id,table);
            
            tree.add_child("NAME");
            id = ggsv(&mut tree.children[1], list, id,table);

            if check_final_token(id,list)&& list[id].value == "=" {
                tree.add_child("=");
                id+=1;
            } else {
                panic!("Erro: Token inesperado {}", list[id].value);
            }
            if list[id].value == "new" || list[id].value == "++" || list[id].value == "--" {
                tree.add_child("EXP");
                id = ggsv(&mut tree.children[3], list, id,table);
            }else{
                if verifica_tipo(table, list, id-2, "int".to_string()) || list[id-2].value.parse::<i64>().is_ok() {
                    if list[id].value.parse::<i64>().is_ok() || verifica_tipo(table, list, id, "int".to_string()) {
                        tree.add_child("EXP");
                        id = ggsv(&mut tree.children[3], list, id,table);
                    }else{
                        panic!("Tipo inesperado {}",list[id]);
                    }
                }else if verifica_tipo(table, list, id-2, "float".to_string()) || list[id-2].value.parse::<f64>().is_ok() {
                    if !list[id].value.parse::<f64>().is_ok() || !verifica_tipo(table, list, id, "float".to_string()) {
                        tree.add_child("EXP");
                        id = ggsv(&mut tree.children[3], list, id,table);
                    }else{
                        panic!("Tipo inesperado {}",list[id]);
                    }
                } else {
                    tree.add_child("EXP");
                    id = ggsv(&mut tree.children[3], list, id,table);
                }
            }
            return id;
        },
        "ELSE" => {
            if list[index].value == "else" {
                tree.add_child("else");
                id +=1;
                tree.add_child("BLOC");
                id = ggsv(&mut tree.children[1], list, id,table);
                return id;
            } else {
                tree.add_child(EPSLON);
                return id;
            }
        },
        "FOR_EXP" => {
            if list[id+2].value == ":" {
                tree.add_child("TYPE");
                id = ggsv(&mut tree.children[0], list, id,table);

                tree.add_child("ID");
                id = ggsv(&mut tree.children[1], list, id,table);

                tree.add_child(":");
                id += 1;

                tree.add_child("ID");
                id = ggsv(&mut tree.children[3], list, id,table);

                tree.add_child("NAME");
                id = ggsv(&mut tree.children[4], list, id,table);
            } else {
                tree.add_child("ATRIB_DECL");
                id = ggsv(&mut tree.children[0], list, id,table);
                
                /*if check_final_token(id,list)&& list[id].value == ";" {
                    tree.add_child(";");
                    id+=1;
                } else {
                    panic!("Erro: Token inesperado {}", list[id].value);
                }*/

                tree.add_child("EXP_LOGIC");
                id = ggsv(&mut tree.children[1], list, id,table);

                if check_final_token(id,list)&& list[id].value == ";" {
                    tree.add_child(";");
                    id+=1;
                } else {
                    panic!("Erro: Token inesperado {}", list[id].value);
                }
                tree.add_child("ATRIB");
                id = ggsv(&mut tree.children[3], list, id,table);
            }
        
            return id;
        },
        "SWITCH_CASE" => {
            if list[id].value == "case" {
                tree.add_child("case");
                id +=1;

                tree.add_child("CONST");
                id = ggsv(&mut tree.children[1], list, id,table);

                if check_final_token(id,list)&& list[id].value == ":" {
                    tree.add_child(":");
                    id+=1;
                } else {
                    panic!("Erro: Token inesperado {}", list[id].value);
                }

                tree.add_child("BLOC");
                id = ggsv(&mut tree.children[3], list, id,table);

                tree.add_child("SWITCH_CASE");
                id = ggsv(&mut tree.children[4], list, id,table);

            } else if list[id].value == "default" {
                tree.add_child("default");
                id +=1;

                tree.add_child("BLOC");
                id = ggsv(&mut tree.children[1], list, id,table);
            } else {
                panic!("Erro: Token inesperado {}", list[id].value);
            }
            return id;
        },
        "EXP" => {
            if list[id].value == "new" {
                tree.add_child("new");
                id += 1;

                tree.add_child("TYPE");
                id = ggsv(&mut tree.children[1], list, id,table);

                tree.add_child("NAME");
                id = ggsv(&mut tree.children[2], list, id,table);
            } else if list[id].value == "++" || list[id].value == "--" { 
                tree.add_child("OPERATOR");
                id = ggsv(&mut tree.children[0], list, id,table);
                
                tree.add_child("ID");
                id = ggsv(&mut tree.children[1], list, id,table);

                tree.add_child("NAME");
                id = ggsv(&mut tree.children[2], list, id,table);
            } else {
                if is_valid_const_value(&list[id].value) || list[id].value == "this" {
                    tree.add_child("EXP_MATH");
                    id = ggsv(&mut tree.children[0], list, id,table);
                } else {
                    tree.add_child("EXP_LOGIC");
                    id = ggsv(&mut tree.children[0], list, id,table);
                }
            }
            return id;
        },
        "OPERATOR" => {
            if list[id].value == "++" || list[id].value == "--" && check_final_token(id,list){
                tree.add_child(&list[id].value);
                return id+1;
            } else {
                panic!("Erro: Token inesperado {}", list[id].value);
            }
        },
        "PARAMS" => {
            tree.add_child("PARAM");
            id = ggsv(&mut tree.children[0], list, id,table);

            tree.add_child("PARAM_LIST");
            id = ggsv(&mut tree.children[1], list, id,table);
            return id;
        },
        "PARAM_LIST" => {
            if list[id].value == "," {
                tree.add_child(",");
                id+=1;

                tree.add_child("PARAM");
                id = ggsv(&mut tree.children[1], list, id,table);

                tree.add_child("PARAM_LIST");
                id = ggsv(&mut tree.children[2], list, id,table);
                return id;
            }
            tree.add_child(EPSLON);
            return id;
        },
        "EXP_LOGIC" => {
            tree.add_child("EXP");
            id = ggsv(&mut tree.children[0], list, id,table);

            if list[id].value == ">" || list[id].value == "<" || list[id].value == ">=" || list[id].value == "<=" || list[id].value == "==" || list[id].value == "!=" {
                tree.add_child("OP_LOGIC");
                id = ggsv(&mut tree.children[1], list, id,table );
                if verifica_tipo(table, list, id-2, "int".to_string()) || list[id-2].value.parse::<i64>().is_ok() {
                    if list[id].value.parse::<i64>().is_ok() || verifica_tipo(table, list, id, "int".to_string()) {
                        tree.add_child("EXP_LOGIC");
                        id = ggsv(&mut tree.children[2], list, id,table);
                    }else{
                        panic!("Tipo inesperado {}",list[id]);
                    }
                    
                }else if verifica_tipo(table, list, id-2, "float".to_string()) || list[id-2].value.parse::<f64>().is_ok() {
                    if !list[id].value.parse::<f64>().is_ok() || !verifica_tipo(table, list, id, "float".to_string()) {
                        tree.add_child("EXP_LOGIC");
                        id = ggsv(&mut tree.children[2], list, id,table);
                    }else{
                        panic!("Tipo inesperado {}",list[id]);
                    }
                }
            }
            return id;
        },
        "EXP_MATH" => {
            tree.add_child("PARAM");
            id = ggsv(&mut tree.children[0], list, id,table);

            if list[id].value == "+" || list[id].value == "-" || list[id].value == "*" || list[id].value == "/" {
                tree.add_child("OP_MATH");
                id = ggsv(&mut tree.children[1], list, id,table);
                
                if verifica_tipo(table, list, id-2, "int".to_string()) || list[id-2].value.parse::<i64>().is_ok() {
                    if list[id].value.parse::<i64>().is_ok() || verifica_tipo(table, list, id, "int".to_string()) {
                        tree.add_child("EXP_MATH");
                        id = ggsv(&mut tree.children[2], list, id,table);
                    }else{
                        panic!("Tipo inesperado {}",list[id]);
                    }
                    
                }else if verifica_tipo(table, list, id-2, "float".to_string()) || list[id-2].value.parse::<f64>().is_ok() {
                    if !list[id].value.parse::<f64>().is_ok() || !verifica_tipo(table, list, id, "float".to_string()) {
                        tree.add_child("EXP_MATH");
                        id = ggsv(&mut tree.children[2], list, id,table);
                    }else{
                        panic!("Tipo inesperado {}",list[id]);
                    }
                }
            }
            return id;
        },
        "OP_LOGIC" => {
            if check_final_token(id,list) && (list[id].value == ">" || list[id].value == "<" || list[id].value == ">=" || list[id].value == "<=" || list[id].value == "==" || list[id].value == "!=")  {
                tree.add_child(&list[id].value);
                return id+1;
            } else {
                panic!("Erro: Token inesperado {}", list[id].value);
            }
        },
        "OP_MATH" =>{
            if check_final_token(id,list) && (list[id].value == "+" || list[id].value == "-" || list[id].value == "*" || list[id].value == "/")  {
                tree.add_child(&list[id].value);
                return id+1;
            } else {
                panic!("Erro: Token inesperado {}", list[id].value);
            }
        },
        "PARAM" => {
            if list[id].value == "this" {
                tree.add_child("this");
                id +=1;
                tree.add_child("FIELD");
                id = ggsv(&mut tree.children[1], list, id,table);
            } else if list[id].value.parse::<i64>().is_ok() || list[id].value.parse::<f64>().is_ok() || list[id].value.contains('"') || list[id].value.contains("'") { 
                tree.add_child("CONST");
                id = ggsv(&mut tree.children[0], list, id,table);
            } else if list[id].value != ")" {
                tree.add_child("ID");
                id = ggsv(&mut tree.children[0], list, id,table);

                tree.add_child("NAME");
                id = ggsv(&mut tree.children[1], list, id,table);
            }
            return id;
        },
        "ARRAY_SIZE" => {
            if list[id].value == "[" {
                let lastValue = &list[id-1].value;
                for (_,row) in table.iter().enumerate(){
                    if row.name == lastValue.to_string() && (row.scope == *SCOPE.lock().unwrap() || row.scope == "global") && row.data_type != "array".to_string() {
                        panic!("Erro: Variável {} já declarada como {}. Não é possível utilizar [].", row.name, row.data_type);
                    }
                }
                tree.add_child("[");
                id += 1;
                tree.add_child("EXP_MATH");
                id = ggsv(&mut tree.children[1], list, id,table);

                if check_final_token(id,list)&& list[id].value == "]" {
                    tree.add_child("]");
                    id+=1;
                } else {
                    panic!("Erro: Token inesperado {}", list[id].value);
                }

                tree.add_child("ARRAY_SIZE");
                id = ggsv(&mut tree.children[3], list, id,table);
                return id;
            }
            tree.add_child(EPSLON);
            return id;
        },
        "NAME" => {
            if check_final_token(id,list) && list[id].value == "(" {
                tree.add_child("(");
                id += 1;

                tree.add_child("PARAMS");
                id = ggsv(&mut tree.children[1], list, id,table);

                if check_final_token(id,list)&& list[id].value == ")" {
                    tree.add_child(")");
                    id+=1;
                } else {
                    panic!("Erro: Token inesperado {}", list[id].value);
                }

                tree.add_child("NAME");
                id = ggsv(&mut tree.children[3], list, id,table);
                return id;
            } else if list[id].value == "." {
                tree.add_child("FIELD");
                id = ggsv(&mut tree.children[0], list, id,table);
                return id;
            } else if list[id].value == "[" {
                tree.add_child("ARRAY_SIZE");
                id = ggsv(&mut tree.children[0], list, id,table);
                
                tree.add_child("NAME");
                id = ggsv(&mut tree.children[1], list, id,table);

                return id;
            }
            tree.add_child(EPSLON);
            return id;
        },
        "FIELD" => {
            if check_final_token(id,list) && list[id].value == "."{
                tree.add_child(".");
                id+=1;
                let lastValue = &list[id-2].value;
                if lastValue == "this" {
                    let class = CLASSSCOPE.lock().unwrap();
                    let rows = find_on_table_by(table, &class, "scope");
                    let in_scope_rows: Vec<_> = rows
                            .iter()
                            .filter(|row| row.scope == class.to_string() && row.name == list[id].value)
                            .cloned()
                            .collect();
                    if in_scope_rows.len() == 0 {
                        panic!("Erro: Não é possível acessar um método não declarado anteriormente {}", list[id].value);
                    }
                }
                for (_,row) in table.iter().enumerate(){
                    if row.name == lastValue.to_string() && row.data_type != "class".to_string() {
                        println!("testeaaaaaaaaaaaaaaaaaaa");
                    }
                }

                tree.add_child("ID");
                id = ggsv(&mut tree.children[1], list, id,table);

                tree.add_child("NAME");
                id = ggsv(&mut tree.children[2], list, id,table);
                return id;
            } else {
                panic!("Erro: Token inesperado {}", list[id].value);
            }
        },
        "CONST" => {
            if list[id].value.parse::<i64>().is_ok() || list[id].value.parse::<f64>().is_ok() {
                tree.add_child("NUMBER");
                id = ggsv(&mut tree.children[0], list, id,table);
                return id;
            } else {
                tree.add_child(&list[id].value);
                return id+1;
            }
        },
        "ID" => {
            let mut adicionar = false;
            if matches!(list[id].token, Token::Identifier) {
                if !matches!(list[id+1].token, Token::Identifier) {
                    let rows = find_on_table_by(table, &list[id].value, "name");
                    let in_scope_rows: Vec<_> = rows
                            .iter()
                            .filter(|row| row.scope == SCOPE.lock().unwrap().to_string())
                            .cloned()
                            .collect();
                    
                    if  in_scope_rows.len() > 0 {
                        if matches!(list[id-1].token, Token::Identifier | Token::Type) {
                            panic!("Erro: Não é possível redeclarar {}", list[id].value);
                        }
                    }else{
                        adicionar = true;
                    }

                    if rows.len() == 0 {
                        let mut a = 1;
                        while list[id - a].value == "[" || list[id - a].value == "]" { 
                            a += 1;
                        }

                        if !matches!(list[id - a].token, Token::Identifier | Token::Type | Token::Instance | Token::Inheritance ) {
                            if !matches!(list[id - a].token, Token::Final | Token::ClassType ) {
                                panic!("Erro: Não é possível acessar um valor não declarado anteriormente {}", list[id].value);
                            }
                        }
                    }
                }
                
                tree.add_child(&list[id].value);

                if adicionar{
                    let name = list[id].value.to_owned();

                    let data_type;
                    let mut a = 1;
                    let mut classification = "Atribute".to_string();
                    while list[id - a].value == "[" || list[id - a].value == "]" { 
                        a += 1;
                    }

                    if list[id-1].value == "class" {
                        classification = "class".to_string()
                    }

                    if matches!(list[id - a].token, Token::Identifier | Token::Type) {
                        data_type = list[id-a].value.to_string();
                    } else {
                        data_type = "void".to_string();
                    }
    
                    table.push(Row {
                        name: name,
                        classification,
                        data_type,
                        scope: SCOPE.lock().unwrap().to_string(),
                        qtd: 0,
                        ord: 0
                    });
                }
                
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

const VALID_VALUES: &[&str] = &["VAR", "COMMAND"];

fn process_inter_code<'a>(tree: &mut TreeNode<&'a str>, previous: &str, row: &mut InterCodeRow) {
    if previous == EPSLON {
        return;
    }
    
    if previous == "VALUE" && tree.value == "EXP_MATH" {
        return process_inter_code(tree, "EXP_MATH", row);
    }

    if previous == "EXP_MATH" && tree.value == previous {
        if tree.children.len() == 1 {
            return process_inter_code(tree, "NUMBER2", row);
        }
    }
    if previous == "OP_MATH" && tree.value == "EXP_MATH" {
        println!("{}", tree.children[1].children[0].value);
        match tree.children[1].children[0].value {
            "+" => { row.op = OP::ADD },
            "-" => { row.op = OP::SUB },
            "/" => { row.op = OP::DIV },
            "*" => { row.op = OP::MULT },
            &_ => todo!(),
        }
        return process_inter_code(&mut tree.children[0], "NUMBER3", row);
    }

    if previous == "NUMBER2" && tree.value == "NUMBER" {
        row.end2 = Some(tree.children[0].value.to_string());
        return;
    }

    // println!("{} {}", previous, tree.value);
    if previous == "NUMBER1" && tree.value == "NUMBER" {
        row.end1 = Some(tree.children[0].value.to_string());
        return;
    }

    if previous == "NUMBER3" && tree.value == "NUMBER" {
        row.end3 = Some(tree.children[0].value.to_string());
        return;
    }
    
    if previous == "ID" && tree.value == previous {
        row.end1 = Some(tree.children[0].value.to_string());
    }

    for child in &mut tree.children {
        process_inter_code(child, previous, row);
    }
}

fn code_inter<'a>(tree: &mut TreeNode<&'a str>, table: &mut Vec<InterCodeRow>, previous: &str, index: &mut usize) {
    if previous == EPSLON && VALID_VALUES.contains(&tree.value) && tree.children[0].value != EPSLON {
        if tree.value == "VAR" && tree.children[2].children[0].value != EPSLON {
            let value = &tree.children[2];
            
            if value.children.len() == 2 {
                let mut inter_code_row = InterCodeRow { 
                    op: OP::ATRIB,
                    end1: Some(tree.children[0].children[0].value.to_string()),
                    end2: None,
                    end3: None,
                };
                table.push(inter_code_row);
                return code_inter(tree, table, "EXP_MATH", index);
            }
        }
    }

    if previous == "EXP_MATH" && tree.value == previous {
        if tree.children.len() > 1 {
            let mut a = table[*index].clone();
            table[0].end1 = Some("tmp".to_owned()+&index.to_string());
            code_inter(&mut tree.children[2], table, "EXP_MATH", index);
            
            let end2; 
            if  table.len() > *index {
                end2 = table[*index].end1.clone();
            } else {
                end2 = table[*index-1].end1.clone();
            }
            table.push(InterCodeRow { 
                op: OP::ATRIB,
                end1: Some("tmp".to_owned()+&index.to_string()),
                end2,
                end3: None,
            });
            process_inter_code(tree, "OP_MATH", &mut table[*index]);
            a.end2 = table[*index].end1.clone();
            *index += 1;
            table.push(a);
            return code_inter(&mut tree.children[0], table, "EXP_MATH", index);
        } else {
            process_inter_code(tree, "EXP_MATH", &mut table[*index]);
            return *index += 1;
        }
    }

    for child in &mut tree.children {
        code_inter(child, table, previous, index);
    }
}

fn main() -> std::io::Result<()> {
    let mut list:Vec<Node> = vec![];

    let mut table:Vec<Row> = vec![];
    
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
            token: token
        });
    }
    
    println!("\n >>> LIST <<< \n");
    // for value in &list {
    //     println!("{}", value);
    // }

    // Chama a função para iniciar a análise gramatical
    ggsv(&mut tree, &list, 0,&mut table);

    println!("\n >>> TREE <<< \n");
     tree.list();

    // println!("\n >>> TABLE <<< \n");
    // for value in &table {
    //     println!("{}", value);
    // }

    println!("\n");
    let mut inter_code_table:Vec<InterCodeRow> = vec![];
    code_inter(&mut tree, &mut inter_code_table, EPSLON, &mut 0);
    println!("{}",inter_code_table.len());

    for val in inter_code_table {
        println!("{:?}", val);
    }

    Ok(())
}