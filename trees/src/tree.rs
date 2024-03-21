use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::iter::{successors, Successors};
use std::rc::Rc;
use super::node::*;
use std::cmp::max;

pub trait Tree<T: Ord + Copy + std::fmt::Debug + std::fmt::Display> {
    type Node: Node<T>;
    fn new() -> Self;

    // ========== functions used by CLI

    fn insert(&mut self, key: T) {
        // first insert node as though in a BST
        let root = self.get_root().clone();
        let new_node = self.bst_insert(root.clone(), key);
        let new_root = match new_node.clone() {
            None => root.clone(),
            Some(n) => Some(self.insert_fix(new_node.clone().unwrap()).clone()),
        };
        self.set_root(new_root.clone());
    }

    fn get_height(&self) -> usize;

    fn print_inorder(&self);

    fn is_empty(&self) -> bool;
    
    fn print_structure(&self);

    fn count_leaves(&self) -> usize;

    // ========== other functions

    fn get_root(&self) -> &Option<Rc<RefCell<Self::Node>>>;
    fn set_root(&mut self, node: Option<Rc<RefCell<Self::Node>>>);
    // fn get_height(&self) -> usize;
    // fn is_empty(&self) -> bool;
    fn insert_fix(&mut self, node: Rc<RefCell<Self::Node>>) -> Rc<RefCell<Self::Node>>;
    // fn insert_fix(&mut self, node: Rc<RefCell<Self::Node>>);
    // fn delete_fix(&mut self, node: Rc<RefCell<Self::Node>>);   

    fn rotate(&mut self, side: Side, node: Rc<RefCell<Self::Node>>);

    fn bst_insert(&mut self, root: Option<Rc<RefCell<Self::Node>>>, k: T) ->  Option<Rc<RefCell<Self::Node>>> {
        let mut node = root.clone();
        let mut parent = None;
        let mut n;
        
        while node.is_some() {
            parent = node.clone();
            n = node.clone().unwrap();
            if n.as_ref().borrow().greater(k) {
                node = self.left(n.clone());
            } else if n.as_ref().borrow().less(k) {
                node = self.right(n.clone());
            } else {
                return None
            } 
        }

        let rc = Some(Rc::new(RefCell::new(Self::Node::new(k))));
        let new_node = rc.clone().unwrap();
        if parent.is_none() {
            self.set_root(rc.clone());
        } else {
            let p_ptr = parent.clone().unwrap();
            let mut p = p_ptr.as_ref().borrow_mut();
            if p.greater(k) {
                p.set_child(Side::Left, rc.clone());
                new_node.as_ref().borrow_mut().set_parent(Some(Side::Left), parent.clone());
            } else {
                p.set_child(Side::Right, rc.clone());
                new_node.as_ref().borrow_mut().set_parent(Some(Side::Right), parent.clone());
            }
        }
        rc.clone()
    }

    fn bst_search(&self, key:T) -> bool{
        let root = self.get_root().clone();
        let node = self.bst_find(root.clone(), key);
        node.is_some()
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

    fn get_parent(&self, node: Rc<RefCell<Self::Node>>) -> Option<Rc<RefCell<Self::Node>>> {
        node.as_ref().borrow().get_parent().clone()
    }

    fn get_child(&self, node: Rc<RefCell<Self::Node>>, side: Side) -> Option<Rc<RefCell<Self::Node>>> {
        node.as_ref().borrow().get_child(side).clone()
    }

    fn take_child(&mut self, node: Rc<RefCell<Self::Node>>, side: Side) -> Option<Rc<RefCell<Self::Node>>> {
        node.as_ref().borrow_mut().take_child(side)
    }

    fn is_leaf(&self, node: Rc<RefCell<Self::Node>>) -> bool {
        node.as_ref().borrow().is_leaf()
    }

    fn right(&self, node: Rc<RefCell<Self::Node>>) -> Option<Rc<RefCell<Self::Node>>> {
        node.as_ref().borrow().right().clone()
    }

    fn left(&self, node: Rc<RefCell<Self::Node>>) -> Option<Rc<RefCell<Self::Node>>> {
        node.as_ref().borrow().left().clone()
    }

    fn get_is_child(&self, node: Rc<RefCell<Self::Node>>) -> Option<Side> {
        node.as_ref().borrow().get_is_child().clone()
    }

    fn get_key(&self, node: Rc<RefCell<Self::Node>>) -> T {
        node.as_ref().borrow().get_key().clone()
    }

    fn set_key(&self, node: Rc<RefCell<Self::Node>>, key: T) {
        node.as_ref().borrow_mut().set_key(key);
    }

    fn set_child(&self, parent: Rc<RefCell<Self::Node>>, side: Side, child: Option<Rc<RefCell<Self::Node>>>) {
        parent.as_ref().borrow_mut().set_child(side, child.clone());
    }

    
    fn set_parent(&self, child: Rc<RefCell<Self::Node>>, side: Option<Side>, parent: Option<Rc<RefCell<Self::Node>>>) {
        child.as_ref().borrow_mut().set_parent(side, parent.clone());
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

    fn climb_to_root(&self, node: Rc<RefCell<Self::Node>>) -> Rc<RefCell<Self::Node>> {
        let parent = node.as_ref().borrow().get_parent();
        if parent.is_none() {{}
            node
        } else {
           self.climb_to_root(parent.clone().unwrap())
        }
    }

    fn find_min(&self, node: Rc<RefCell<Self::Node>>) -> Rc<RefCell<Self::Node>> {
        let n = node.as_ref().borrow();
        match n.left() {
            None => node.clone(),
            Some(child) => self.find_min(n.left().clone().unwrap().clone()),
        }
    }
    
}
