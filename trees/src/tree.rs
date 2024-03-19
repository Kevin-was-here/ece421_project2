use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::rc::Rc;
use crate::node::*;

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
        let (mut new_root, inserted_node, fix_tree) = self.bst_insert(root.clone(), key);

        if fix_tree {
            new_root = self.insert_fix(inserted_node);
        }
        self.set_root(Some(new_root.clone()));
    } 

    fn delete(&mut self, k: T) {
        // similar to insert
        let root = self.get_root();
        // let (mut new_root, fix_tree) = self.bst_delete(root.clone(), k); // should return root here 
        self.bst_delete(root.clone(), k);
        // if fix_tree {
        //     // new_root = self.delete_fix(inserted_node); // replace with actual fix function
        // }
        // self.set_root(new_root);
    }

    fn bst_find(&self, root: Option<Rc<RefCell<Self::Node>>>, k: T) -> Option<Rc<RefCell<Self::Node>>> {
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

    fn bst_insert(&self, node: Option<Rc<RefCell<Self::Node>>>, k: T) -> (Rc<RefCell<Self::Node>>, Rc<RefCell<Self::Node>>, bool) {
        match node {
            None => {
                let new_node = Self::Node::new(k); 
                let rc = Rc::new(RefCell::new(new_node)); 
                (rc.clone(), rc, true)
            },
            Some(n) => {
                let insert_side = self.insert_side(n.clone(), k);
                match insert_side {
                    None => (n.clone(), n.clone(), false),
                    Some(side) => {
                        let mut node = n.as_ref().borrow_mut();
                        let old_subtree = node.take_child(side);
                        let (new_subtree, new_node, fix_tree) = self.bst_insert(old_subtree, k);
    
                        // update links between current node and its child
                        node.set_child(side, Some(new_subtree.clone()));
                        new_subtree.as_ref().borrow_mut().set_parent(Some(side), Some(n.clone())); 
                        (n.clone(), new_node, fix_tree)
                    },
                }
            },
        }
    }

    fn bst_delete(&mut self, root: Option<Rc<RefCell<Self::Node>>>, k: T) {
        match self.bst_find(root, k) {
            None => (),
            Some(n) => {
                self.bst_replace(n);
            },
        }
    }

    fn bst_replace(&mut self, node: Rc<RefCell<Self::Node>>) {
        let mut n = node.as_ref().borrow_mut();
        if n.child_count() == 0 && n.get_parent().is_none() {
            
        } else if n.child_count() == 0 && n.get_parent().is_some() {
    
        } else if n.child_count() == 1 && n.get_child(Side::Left).is_none() {
    
        } else if n.child_count() == 1 && n.get_child(Side::Right).is_none() {
    
        } else {}
    }
    
    fn insert_side(&self, node: Rc<RefCell<Self::Node>>, k: T) -> Option<Side> {
        let n = node.as_ref().borrow();
        if n.greater(k) {
            Some(Side::Left)
        } else if n.less(k) {
            Some(Side::Right)
        } else {
            None
        }
    }
    
}
