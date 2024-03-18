use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Not;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Side {
    Left,
    Right,
}

impl Not for Side {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

pub trait Traversible<T> {
    fn left_mut(&mut self) -> &mut Option<Rc<RefCell<Self>>>;
    fn right_mut(&mut self) -> &mut Option<Rc<RefCell<Self>>>;
    fn left(&self) -> &Option<Rc<RefCell<Self>>>;
    fn right(&self) -> &Option<Rc<RefCell<Self>>>;
}
pub trait Node<T>: Traversible<T> {
    fn new(key: T) -> Self;
    fn get_key(&self) -> &T; 

    fn greater(&self, val: T) -> bool;
    fn equal(&self, val: T) -> bool;
    fn less(&self, val: T) -> bool;

    fn get_child(&self, side: Side) -> Option<Rc<RefCell<Self>>>;
    fn take_child(&mut self, side: Side) -> Option<Rc<RefCell<Self>>>;
    fn set_child(&mut self, side: Side, node: Option<Rc<RefCell<Self>>>);

    fn set_parent(&mut self, is_child: Option<Side>, node: Option<Rc<RefCell<Self>>>);
    fn get_parent(&self) -> Option<Rc<RefCell<Self>>>;
    fn get_parent_mut(&mut self) -> &mut Option<Rc<RefCell<Self>>>;
}

pub fn bst_insert<T: Ord + Copy, N: Node<T>>(node: Option<Rc<RefCell<N>>>, k: T) -> (Rc<RefCell<N>>, Rc<RefCell<N>>, bool) {
    match node {
        None => {
            let new_node = N::new(k); 
            let rc = Rc::new(RefCell::new(new_node)); 
            (rc.clone(), rc, true)
        },
        Some(n) => {
            let insert_side = insert_side(n.clone(), k);
            match insert_side {
                None => (n.clone(), n.clone(), false),
                Some(side) => {
                    let mut node = n.as_ref().borrow_mut();
                    let old_subtree = node.take_child(side);
                    let (new_subtree, new_node, fix_tree) = bst_insert(old_subtree, k);
                    // update links between current node and its child
                    node.set_child(side, Some(new_subtree.clone()));
                    new_subtree.as_ref().borrow_mut().set_parent(Some(side), Some(n.clone())); 
                    (n.clone(), new_node, fix_tree)
                },
            }
        },
    }
}

pub fn bst_delete<T: Ord + Copy, N: Node<T>>(root: Option<Rc<RefCell<N>>>, k: T) -> (Option<Rc<RefCell<N>>>, bool) {
    let mut current_node = root;
    loop {
        if current_node.is_none() {
            return (None, false)
        }
        let n = current_node.unwrap();
        if n.as_ref().borrow().equal(k) {
            // bst_replace(current_node);
            return (current_node, true)
        }
        else if n.as_ref().borrow().greater(k) {
            current_node = n.as_ref().borrow().get_child(Side::Left);
        }
        else {
            current_node = n.as_ref().borrow().get_child(Side::Right);
        }
    }
}

pub fn insert_side<T: Ord + Copy, N: Node<T>>(node: Rc<RefCell<N>>, k: T) -> Option<Side> {
    let n = node.as_ref().borrow();
    if n.greater(k) {
        Some(Side::Left)
    } else if n.less(k) {
        Some(Side::Right)
    } else {
        None
    }
}