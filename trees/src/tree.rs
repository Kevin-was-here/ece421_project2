use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::iter::{successors, Successors};
use std::rc::Rc;
use super::node::*;
use std::cmp::max;

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
        let temp = root.clone();
        let (new_root, successor, fix_tree) = self.bst_delete(temp, k);
        // if fix_tree {
        //     // new_root = self.delete_fix(inserted_node); // replace with actual fix function
        // }
        self.set_root(new_root);
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

    fn bst_delete(&mut self, root: Option<Rc<RefCell<Self::Node>>>, k: T) -> (Option<Rc<RefCell<Self::Node>>>, Option<Rc<RefCell<Self::Node>>>, bool) {
        match self.bst_find(root.clone(), k) {
            None => (root.clone(), None, false),
            Some(n) => {
                let successor = self.bst_replace(n);
                match successor.clone() {
                    None => (self.get_root().clone(), None, false),
                    Some(s) => {
                        let temp = s.clone();
                        let root = self.climb_to_root(temp);
                        (Some(root.clone()), successor.clone(), true)
                    }
                }   
            },
        }
    }

    // return successor
    fn bst_replace(&mut self, node: Rc<RefCell<Self::Node>>) -> Option<Rc<RefCell<Self::Node>>> {
        // let mut n = node.as_ref().borrow_mut();
        let is_root = self.get_parent(node.clone()).is_none();
        // find successor
        let successor = {
            if self.is_leaf(node.clone()) {
                None

            } else if self.get_child(node.clone(), Side::Left).is_none() && self.get_child(node.clone(), Side::Right).is_some() {

                self.take_child(node.clone(), Side::Right).clone()
                
            } else if self.get_child(node.clone(), Side::Left).is_none() && self.get_child(node.clone(), Side::Right).is_none() {

                self.take_child(node.clone(), Side::Left).clone()

            } else {
                let (min_val, min_right_child, min_parent_ptr, node_key) = {
                    let right = self.right(node.clone()).unwrap();
                    let min_ptr = self.find_min(right);
                    let min_val = self.get_key(min_ptr.clone());
                    let min_parent_ptr = self.get_parent(min_ptr.clone()).unwrap();
                    let min_right_child = self.right(min_ptr.clone());
                    let node_key = self.get_key(node.clone()).clone();

                    (min_val, min_right_child, min_parent_ptr, node_key)
                };

                {
                    let mut min_parent = min_parent_ptr.as_ref().borrow_mut();

                    if min_parent.equal(node_key.clone()) {
                        min_parent.set_child(Side::Right, min_right_child.clone());
                    } else {
                        min_parent.set_child(Side::Left, min_right_child.clone());
                    }
                } 

                self.set_key(node.clone(), min_val.clone());

                Some(node.clone())
            }

        };

        // update parent
        if is_root {
            self.set_root(successor.clone());
        } else {
            let parent= self.get_parent(node.clone()).unwrap();
            let node_side = self.get_is_child(node.clone());
            parent.as_ref().borrow_mut().set_child(node_side.unwrap(), successor.clone());
            let s = successor.clone().unwrap();
            s.as_ref().borrow_mut().set_parent(node_side, Some(parent.clone()));
        }

        successor.clone()

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

    fn get_key(&self, node: Rc<RefCell<Self::Node>>) -> T {
        node.as_ref().borrow().get_key().clone()
    }

    fn set_key(&self, node: Rc<RefCell<Self::Node>>, key: T) {
        node.as_ref().borrow_mut().set_key(key);
    }

    fn get_is_child(&self, node: Rc<RefCell<Self::Node>>) -> Option<Side> {
        node.as_ref().borrow().get_is_child().clone()
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

    // fn set_height(&self, node: Rc<RefCell<Self::Node>>) {
    //     //given the node, calculate the height of the node and set it in the node
    //     if self.is_leaf(node.clone()){
    //         node.as_ref().borrow_mut().set_height(0);
    //     } else {
    //         let left_height = match self.get_child(node.clone(), Side::Left) {
    //             None => 0,
    //             Some(child) => self.get_height(child.clone()),
    //         };
    //         let right_height = match self.get_child(node.clone(), Side::Right) {
    //             None => 0,
    //             Some(child) => self.get_height(child.clone()),
    //         };
    //         let height = max(left_height, right_height) + 1;
    //         node.as_ref().borrow_mut().set_height(height);
    //     }
    // }

    // fn get_height(&self, node: Rc<RefCell<Self::Node>>) -> usize {
    //     node.as_ref().borrow().get_height()
    // }
    
}
