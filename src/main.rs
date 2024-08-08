mod enum_token;
mod node;
mod tree;
mod linked_list;
mod table_row;
mod utils;

use tree::TreeNode;
use enum_token::Token;
use node::Node;
use table_row::Row;
use utils::*;

#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

lazy_static! {
    static ref SCOPE: Mutex<String> = Mutex::new(String::from("global"));
}

const EPSLON: &str = "ε";

fn ggsv<'a>(tree: &mut TreeNode<&'a str>, list: &'a [Node], index: usize, table: &mut Vec<Row>) -> usize {
    let mut id = index;
    // println!("{} {}",list[id].value, tree.value);
    
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
            
            tree.add_child("ID");            
            id = ggsv(&mut tree.children[1], list, id,table);

            tree.add_child("INHERITANCE");
            id = ggsv(&mut tree.children[2], list, id,table);
            
            if check_final_token(id,list)&& list[id].value == "{"  {
               tree.add_child("{");
               id+=1;
            } else {
                println!("{} {} {}", id, list.len(), list[id].value);
                panic!("Erro: Token inesperado {}", list[id].value);
            }
            tree.add_child("ITEM_DECLS");
            id = ggsv(&mut tree.children[4], list, id,table);
            
            if check_final_token(id,list)&& list[id].value == "}" {
               tree.add_child("}");
               id+=1;
            } else {
                println!("{}",list[id]);
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
                id = ggsv(&mut tree.children[0], list, id,table);
                id = ggsv(&mut tree.children[1], list, id,table);
                id = ggsv(&mut tree.children[2], list, id,table);
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

            tree.add_child("EXP");
            id = ggsv(&mut tree.children[3], list, id,table);

            return id;
        },
        "ELSE" => {
            if list[index].value == "else" {
                tree.add_child("else");
                id +=1;
                tree.add_child("BLOC");
                id = ggsv(&mut tree.children[1], list, id+1,table);
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
                println!("{}", list[id].value);
                if check_final_token(id,list)&& list[id].value == ";" {
                    tree.add_child(";");
                    id+=1;
                } else {
                    panic!("Erro: Token inesperado {}", list[id].value);
                }

                tree.add_child("EXP_LOGIC");
                id = ggsv(&mut tree.children[2], list, id,table);

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
                id = ggsv(&mut tree.children[1], list, id+1,table);

                tree.add_child("NAME");
                id = ggsv(&mut tree.children[2], list, id+2,table);
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

                tree.add_child("EXP_LOGIC");
                id = ggsv(&mut tree.children[2], list, id,table);
            }
            return id;
        },
        "EXP_MATH" => {
            tree.add_child("PARAM");
            id = ggsv(&mut tree.children[0], list, id,table);

            if list[id].value == "+" || list[id].value == "-" || list[id].value == "*" || list[id].value == "/" {
                tree.add_child("OP_MATH");
                id = ggsv(&mut tree.children[1], list, id+1,table);

                tree.add_child("EXP_MATH");
                id = ggsv(&mut tree.children[2], list, id,table);
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
                id = ggsv(&mut tree.children[1], list, id+1,table);
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
            if check_final_token(id,list)&& list[id].value == "(" {
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
                    }

                    if rows.len() == 0 && !matches!(list[id-1].token, Token::Identifier | Token::Type | Token::Instance | Token::Inheritance ) {
                        if !matches!(list[id-1].token, Token::Final | Token::ClassType ) {
                            panic!("Erro: Não é possível acessar um valor não declarado anteriormente {}", list[id].value);
                        }
                    }
                }
                
                tree.add_child(&list[id].value);

                let name = list[id].value.to_owned();

                let data_type;
                if matches!(list[id-1].token, Token::Type) {
                    data_type = list[id-1].value.to_string();
                } else {
                    data_type = "void".to_string();
                }

                table.push(Row {
                    name: name,
                    classification: list[id].token,
                    data_type,
                    scope: SCOPE.lock().unwrap().to_string(),
                    qtd: 32,
                    ord: 12
                });
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

    let mut table:Vec<Row> = vec![];
    
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
    // tree.list();

    println!("\n >>> TABLE <<< \n");
    // for value in table {
    //     println!("{}", value);
    // }
    
    Ok(())
}