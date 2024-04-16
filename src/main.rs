mod linked_list;
mod enum_token;

use linked_list::LinkedList;
use enum_token::Token;

fn main() {
    let mut list = LinkedList::new((-1).to_string(), Token::Struct);
    
    for value in 0..10 {
        list.append(value.to_string(), Token::Final);
    }
    
    list.print();
}