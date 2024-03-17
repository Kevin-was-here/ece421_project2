use std::cell::RefCell;
use std::rc::Rc;
mod tree;
//edit this to use RC and RefCell instead of box

type AvlTree<T> = Option<Rc<RefCell<AvlTreeNode<T>>>>; // What is Box?

#[derive(Debug, PartialEq, Clone)]
struct AvlTreeNode<T: Ord> {
    key: T,
    left: AvlTreeNode<T>,
    right: AvlTreeNode<T>,
}

impl<T: Ord> AvlTreeNode<T> {
    fn new() -> Self {
        Self { root: None }
    }

    fn insert(&self, key: T) {

    }
}