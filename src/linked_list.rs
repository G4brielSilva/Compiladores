// Definição de um nó da lista duplamente encadeada
pub struct ListNode<T> {
    pub data: T,
    pub next: Option<Box<ListNode<T>>>,
    pub prev: Option<*mut ListNode<T>>, // Ponteiro bruto opcional para o nó anterior
}

impl<T> ListNode<T> {
    fn new(data: T) -> Self {
        ListNode {
            data,
            next: None,
            prev: None,
        }
    }
}

// Definição de uma lista duplamente encadeada
pub struct LinkedList<T> {
    head: Option<Box<ListNode<T>>>,
    tail: Option<*mut ListNode<T>>, // Ponteiro bruto opcional para o último nó
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
        let mut new_node = Box::new(ListNode::new(data));
        let new_node_ptr = new_node.as_mut() as *mut ListNode<T>;

        if let Some(tail_ptr) = self.tail {
            unsafe {
                let tail_ref = &mut *tail_ptr;
                new_node.prev = Some(tail_ptr);
                tail_ref.next = Some(new_node);
            }
        } else {
            self.head = Some(new_node);
        }
        self.tail = Some(new_node_ptr);
    }

    // Exemplo de método para iterar sobre os elementos da lista
    pub fn print(&self) {
        let mut current = self.head.as_ref().map(|node| &**node);
        while let Some(node) = current {
            println!("{}", node.data);
            current = node.next.as_ref().map(|next| &**next);
        }
    }

    // Método para obter o nó da cabeça (head) da lista
    pub fn get_head(&self) -> Option<&ListNode<T>> {
        self.head.as_ref().map(|node| &**node)
    }

    // Método para obter o nó em um índice específico da lista
    pub fn get(&self, index: usize) -> Option<&ListNode<T>> {
        let mut current = self.head.as_ref().map(|node| &**node);
        let mut current_index = 0;

        while let Some(node) = current {
            if current_index == index {
                return Some(node);
            }
            current = node.next.as_ref().map(|next| &**next);
            current_index += 1;
        }

        None
    }
}