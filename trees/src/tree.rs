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
    // fn left_mut(&mut self) -> &mut Option<Rc<RefCell<Self>>>;
    // fn right_mut(&mut self) -> &mut Option<Rc<RefCell<Self>>>;
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
    fn get_is_child(&self) -> &Option<Side>;
    fn is_child(&self, side: Side) -> bool;
    fn take_child(&mut self, side: Side) -> Option<Rc<RefCell<Self>>>;
    fn set_child(&mut self, side: Side, node: Option<Rc<RefCell<Self>>>);
    fn child_count(&self) -> usize;

    fn set_parent(&mut self, is_child: Option<Side>, node: Option<Rc<RefCell<Self>>>);
    fn get_parent(&self) -> Option<Rc<RefCell<Self>>>;
    fn get_parent_mut(&mut self) -> &mut Option<Rc<RefCell<Self>>>;

    fn get_sibling(&self) -> Option<Rc<RefCell<Self>>>;
    fn get_uncle(&self) -> Option<Rc<RefCell<Self>>>;
    fn get_grandparent(&self) -> Option<Rc<RefCell<Self>>>;
    
    fn is_leaf(&self) -> bool;

}

pub trait Tree<T: Ord + Copy + std::fmt::Debug + std::fmt::Display> {
    type Node: Node<T>;
    fn new() -> Self;
    fn get_root(&self) -> &Option<Rc<RefCell<Self::Node>>>;
    fn set_root(&mut self, node: Option<Rc<RefCell<Self::Node>>>);
    // fn get_height(&self) -> usize;
    // fn is_empty(&self) -> bool;
    fn insert_fix(&mut self, node: Rc<RefCell<Self::Node>>) -> Rc<RefCell<Self::Node>>;
    // fn delete_fix(&mut self, key: T);   

    fn rotate(&mut self, side: Side, node: Rc<RefCell<Self::Node>>);
    
    fn insert(&mut self, key: T) {
        // first insert node as though in a BST
        let root = self.get_root();
        let (mut new_root, inserted_node, fix_tree) = bst_insert(root.clone(), key);

        if fix_tree {
            new_root = self.insert_fix(inserted_node);
        }
        self.set_root(Some(new_root.clone()));
    } 

    fn delete(&mut self, k: T) {
        // similar to insert
        let root = self.get_root();
        let (mut new_root, fix_tree) = bst_delete(root.clone(), k); // should return root here 
        if fix_tree {
            // new_root = self.delete_fix(inserted_node); // replace with actual fix function
        }
        self.set_root(new_root);
    }
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

pub fn bst_find<T: Ord + Copy, N: Node<T>>(root: Option<Rc<RefCell<N>>>, k: T) -> Option<Rc<RefCell<N>>> {
    let mut current_node = root;
    // Find then replace
    loop {
        if current_node.is_none() {
            return None
        }
        let n = current_node.clone().unwrap();
        if n.as_ref().borrow().equal(k) {
            return current_node.clone()
        }
        else if n.as_ref().borrow().greater(k) {
            current_node = n.as_ref().borrow().get_child(Side::Left);
        }
        else {
            current_node = n.as_ref().borrow().get_child(Side::Right);
        }
    }
}

pub fn bst_delete<T: Ord + Copy, N: Node<T>>(root: Option<Rc<RefCell<N>>>, k: T) {
    match bst_find(root, k) {
        None => (),
        Some(n) => {
            bst_replace(n);
        },
    }
}

pub fn bst_replace<T: Ord + Copy, N: Node<T>>(node: Rc<RefCell<N>>) {
    let mut n = node.as_ref().borrow_mut();
    if n.child_count() == 0 && n.get_parent().is_none() {
        
    } else if n.child_count() == 0 && n.get_parent().is_some() {

    } else if n.child_count() == 1 && n.get_child(Side::Left).is_none() {

    } else if n.child_count() == 1 && n.get_child(Side::Right).is_none() {

    } else {}
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