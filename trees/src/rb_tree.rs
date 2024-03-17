use crate::tree;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black,
}

type RedBlackTree<T> = Option<Rc<RefCell<RedBlackTreeNode<T>>>>;

struct RedBlackTreeNode<T> {
    pub color: NodeColor,
    pub key: T,
    pub parent: RedBlackTree<T>,
    left: RedBlackTree<T>,
    right: RedBlackTree<T>,
}

impl<T> tree::Traversible for RedBlackTreeNode<T> {
    fn left_mut(&mut self) -> &mut Option<Rc<RefCell<Self>>> {
        return self.left.borrow_mut();
    }
    fn right_mut(&mut self) -> &mut Option<Rc<RefCell<Self>>> {
        return self.right.borrow_mut();
    }
    fn left(&self) -> &Option<Rc<RefCell<Self>>> {
        return &self.left;
    }
    fn right(&self) -> &Option<Rc<RefCell<Self>>> {
        return &self.right;
    }
}