use std::cell::RefCell;
use std::rc::Rc;

//edit this to use RC and RefCell instead of box
#[derive(Debug, PartialEq, Clone)]
struct AvlNode<T: Ord> {
    value: T,
    left: AvlTree<T>,
    right: AvlTree<T>,
}

type AvlTree<T> = Option<Box<AvlNode<T>>>; // What is Box?

#[derive(Debug, PartialEq, Clone)]
struct AvlTreeSet<T: Ord> {
    root: AvlTree<T>,
}

impl<T: Ord> AvlTreeSet<T> {
    fn new() -> Self {
        Self { root: None }
    }
}