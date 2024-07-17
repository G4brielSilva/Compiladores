use std::rc::Rc;
use std::cell::RefCell;

// Definição de um nó da lista duplamente encadeada
pub struct ListNode<T> {
    pub data: T,
    next: Option<Rc<RefCell<ListNode<T>>>>,
    prev: Option<Rc<RefCell<ListNode<T>>>>,
}

impl<T> ListNode<T> {
    fn new(data: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(ListNode {
            data,
            next: None,
            prev: None,
        }))
    }

    pub fn next_node(&self) -> Option<Rc<RefCell<ListNode<T>>>> {
        self.next.clone()
    }

    // Obtém a referência para o nó anterior
    pub fn prev_node(&self) -> Option<Rc<RefCell<ListNode<T>>>> {
        self.prev.clone()
    }

    // Exemplo de mutação dos dados do nó
    pub fn set_data(&mut self, new_data: T) {
        self.data = new_data;
    }
}

// Definição de uma lista duplamente encadeada
pub struct LinkedList<T> {
    head: Option<Rc<RefCell<ListNode<T>>>>,
    tail: Option<Rc<RefCell<ListNode<T>>>>,
}

impl<T: std::fmt::Display> LinkedList<T> {
    // Cria uma nova lista duplamente encadeada vazia
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    // Insere um novo elemento no final da lista
    pub fn insert(&mut self, data: T) {
        let new_node = ListNode::new(data);
        match self.tail.take() {
            Some(old_tail) => {
                new_node.borrow_mut().prev = Some(Rc::clone(&old_tail));
                old_tail.borrow_mut().next = Some(Rc::clone(&new_node));
            }
            None => {
                self.head = Some(Rc::clone(&new_node));
            }
        }
        self.tail = Some(new_node);
    }

    // Exemplo de método para iterar sobre os elementos da lista
    pub fn print(&self) {
        let mut current = self.head.clone();
        while let Some(node) = current {
            let borrow_node = node.borrow();
            println!("{}", borrow_node.data);
            current = borrow_node.next.clone();
        }
    }

    pub fn get_head(&self) -> Option<Rc<RefCell<ListNode<T>>>> {
        self.head.clone()
    }

    pub fn get(&self, index: usize) -> Option<Rc<RefCell<ListNode<T>>>> {
        let mut current = self.head.clone();
        let mut current_index = 0;

        while let Some(node) = current {
            if current_index == index {
                return Some(node);
            }
            let borrow_node = node.borrow();
            current = borrow_node.next.clone();
            current_index += 1;
        }

        None
    }
}
