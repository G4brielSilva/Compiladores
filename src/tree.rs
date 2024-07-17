use std::fmt::Debug;


#[derive(Debug)]
pub struct TreeNode<T> {
    pub value: T,
    pub children: Vec<TreeNode<T>>,
}

impl<T: Debug> TreeNode<T> {
    pub fn new(value: T) -> Self {
        TreeNode {
            value,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, data: T) {
        self.children.push(TreeNode::new(data));
    }

    pub fn list(&self) {
        println!("{:?}", self.value);

        for child in &self.children {
            child.list();
        }
    }
}