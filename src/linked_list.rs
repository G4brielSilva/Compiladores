use crate::enum_token::Token;

pub struct LinkedList {
    value: String,
    token: Token,
    next: Option<LinkedList>,
    previous: Option<LinkedList>
}

impl LinkedList {
    pub fn new(value: String, token: Token) -> LinkedList {
        LinkedList {
            value,
            token,
            next: None,
            previous: None
        }
    }

    pub fn append(&mut self, value: String, token: Token) {
        let mut current = self;
        while let Some(ref mut next) = current.next {
            current = next;
        }

        let new_node = Box::new(LinkedList {
            value,
            token,
            next: None,
            previous: Some(Box::new(current.clone())),
        });

        current.next = Some(new_node);
    }

    pub fn print(&self) {
        let mut current = Some(self);
        while let Some(node) = current {
            println!("{} - {}", node.value, node.token);
            current = node.next.as_ref();
        }
    }
}