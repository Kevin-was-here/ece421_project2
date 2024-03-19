use crate::tree::*;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Copy)]
enum NodeColor {
    Red,
    Black,
}

type MaybeRedBlackTree<T> = Option<Rc<RefCell<RedBlackTreeNode<T>>>>;

#[derive(Debug)]
struct RedBlackTreeNode<T> {
    pub color: NodeColor,
    pub key: T,
    pub parent: MaybeRedBlackTree<T>,
    is_child: Option<Side>,
    left: MaybeRedBlackTree<T>,
    right: MaybeRedBlackTree<T>,
}

pub struct RedBlackTree<T> {
    root: MaybeRedBlackTree<T>,
    size: usize,
}

impl<T: Ord> Traversible<T> for RedBlackTreeNode<T> {
    // fn left_mut(&mut self) -> &mut Option<Rc<RefCell<Self>>> {
    //     return self.left.borrow_mut();
    // }
    // fn right_mut(&mut self) -> &mut Option<Rc<RefCell<Self>>> {
    //     return self.right.borrow_mut();
    // }
    fn left(&self) -> &Option<Rc<RefCell<Self>>> {
        return &self.left;
    }
    fn right(&self) -> &Option<Rc<RefCell<Self>>> {
        return &self.right;
    }
}

impl<T: Ord> Node<T> for RedBlackTreeNode<T> {
    fn new(key: T) -> Self {
        Self {
            color: NodeColor::Red,
            key: key,
            parent: None,
            is_child: None, // left or right child of its parent
            left: None,
            right: None,
        }
    }

    fn get_key(&self) -> &T {
        &self.key
    }

    fn greater(&self, val: T) -> bool {
        self.key > val
    }

    fn equal(&self, val: T) -> bool {
        self.key == val
    }

    fn less(&self, val: T) -> bool {
        self.key < val
    }

    fn get_child(&self, side: Side) -> MaybeRedBlackTree<T> {
        match side {
            Side::Left => self.left.clone(),
            Side::Right => self.right.clone(),
        }
    }

    fn take_child(&mut self, side: Side) -> MaybeRedBlackTree<T> {
        match side {
            Side::Left => self.left.take(),
            Side::Right => self.right.take(),           
        }
    }

    fn set_child(&mut self, side: Side, child: MaybeRedBlackTree<T>) {
        match side {
            Side::Left => self.left = child,
            Side::Right => self.right = child,
        }
    }

    fn get_parent(&self) -> MaybeRedBlackTree<T> {
        self.parent.clone()
    }

    fn get_parent_mut(&mut self) -> &mut MaybeRedBlackTree<T> {
        self.parent.borrow_mut()
    }

    fn set_parent(&mut self, is_child: Option<Side>, parent: MaybeRedBlackTree<T>) {
        self.parent = parent;
        self.is_child = is_child;
    }   

}

impl<T: Ord> RedBlackTreeNode<T> {
    fn is_red(&self) -> bool {
        self.color == NodeColor::Red
    }

    fn set_color(&mut self, color: NodeColor) {
        self.color = color
    }

    fn get_color(&self) -> NodeColor {
        self.color
    }

    fn swap_color(&mut self, node: Rc<RefCell<RedBlackTreeNode<T>>>) {
        let mut n = node.as_ref().borrow_mut();
        let temp = n.get_color();
        n.set_color(self.get_color());
        self.set_color(temp); 
    }

    fn is_child(&self, side: Side) -> bool {
        match &self.is_child { 
            None => false,
            Some(val) => {
                if val == &side { true } else { false }
            }
        }
    }

    fn get_is_child(&self) -> &Option<Side> {
        &self.is_child
    }

    fn get_sibling(&self) -> MaybeRedBlackTree<T> {
        if let Some(p) = self.get_parent() {
            let parent = p.as_ref().borrow_mut();
            if self.is_child(Side::Left) {
                parent.get_child(Side::Right)
            } else if self.is_child(Side::Right) {
                parent.get_child(Side::Left)
            } else { None }
        }
        else { None }
    }

    fn get_uncle(&self) -> MaybeRedBlackTree<T> {
        if let Some(p) = self.get_parent() {
            p.as_ref().borrow().get_sibling()
        } else { None }
    } 

    fn get_grandparent(&self) -> MaybeRedBlackTree<T> {
        if let Some(p) = self.get_parent() {
            p.as_ref().borrow().get_parent()
        } else { None }
    } 

    fn uncle_is_red(&self) -> bool {
        if let Some(u) = self.get_uncle() {
            u.as_ref().borrow().is_red()
        }
        else { false }
    }

    fn parent_is_red(&self) -> bool {
        if let Some(p) = self.get_parent() {
            p.as_ref().borrow().is_red()
        }
        else { false }
    }

}

impl<T: Ord + Copy + std::fmt::Debug> RedBlackTree<T> {
    pub fn new() -> Self {
        Self { root: None, size: 0 }
    }

    fn get_root(&self) -> &MaybeRedBlackTree<T> {
        &self.root
    }

    fn set_root(&mut self, node: Rc<RefCell<RedBlackTreeNode<T>>>) {
        self.root = Some(node);
    }

    pub fn insert(&mut self, key: T) {
        let root = self.get_root();
        let (mut new_root, inserted_node, fix_tree) = bst_insert(root.clone(), key);
        if fix_tree {
            new_root = self.insert_fix(inserted_node); // replace with actual fix function
            self.size += 1;
        }
        new_root.as_ref().borrow_mut().set_color(NodeColor::Black);
        self.set_root(new_root);
    } 

    fn climb_to_root(&self, node: Rc<RefCell<RedBlackTreeNode<T>>>) -> Rc<RefCell<RedBlackTreeNode<T>>> {
        let parent = node.as_ref().borrow_mut().get_parent();
        if parent.is_none() {
            node
        } else {
            let mut p = parent.unwrap();
            let mut not_root = true;
            while not_root {
                let temp = p;
                not_root = temp.as_ref().borrow().get_parent().is_some();
                p = temp;
            }
            p
        }
    }
    fn insert_fix(&mut self, node: Rc<RefCell<RedBlackTreeNode<T>>>) -> Rc<RefCell<RedBlackTreeNode<T>>> {
        // get parent: is parent the root?
        let mut current_node = node.clone();
        let mut n = current_node.as_ref().borrow();
        let mut not_root = n.get_parent().is_some();
        let mut red_parent = n.parent_is_red();

        while not_root && red_parent {
            drop(n);
            let temp = current_node.clone();

            if temp.as_ref().borrow().uncle_is_red() {
                self.recolor_rbtree(temp.clone());
                if let Some(grandparent) = temp.as_ref().borrow().get_grandparent() {
                    current_node = grandparent.clone();
                }
                n = current_node.as_ref().borrow();
                red_parent = n.parent_is_red();
                not_root = n.get_parent().is_some();

            }
            else {
                let temp_clone = temp.clone();
                self.rotate_rbtree(temp_clone);
                n = current_node.as_ref().borrow();
                red_parent = current_node.as_ref().borrow().parent_is_red();
            }
        }
        drop(n);
        let temp_clone = current_node.clone();
        let root = if not_root { self.climb_to_root(temp_clone) } else { temp_clone };

        root
    }

    fn recolor_rbtree(&self, node: Rc<RefCell<RedBlackTreeNode<T>>>) {
        let n = node.as_ref().borrow();
        if n.get_grandparent().is_some() && n.get_uncle().is_some() {
            n.get_grandparent().unwrap().as_ref().borrow_mut().set_color(NodeColor::Red);
            n.get_parent().unwrap().as_ref().borrow_mut().set_color(NodeColor::Black);
            n.get_uncle().unwrap().as_ref().borrow_mut().set_color(NodeColor::Black);
        }
    }

    fn rotate_rbtree(&mut self, node: Rc<RefCell<RedBlackTreeNode<T>>>) {
        let node_side;
        let parent_side;
        let parent;
        let grandparent;
        let mut double_rotation = false;
        {
            let n = node.as_ref().borrow();
            node_side = n.get_is_child().unwrap();

            parent = n.get_parent().unwrap();
            let p = parent.as_ref().borrow();
            parent_side = p.get_is_child().unwrap();

            grandparent = n.get_grandparent().unwrap();
        }

        if parent_side != node_side {
            self.rotate(parent_side, parent.clone());
            double_rotation = true;
        }
        
        if double_rotation {
            let mut n = node.as_ref().borrow_mut();
            n.swap_color(grandparent.clone());
        } else {
            let mut p = parent.as_ref().borrow_mut();
            p.swap_color(grandparent.clone());
        }

        self.rotate(!parent_side, grandparent);
    }
    
    fn rotate(&mut self, side: Side, node: Rc<RefCell<RedBlackTreeNode<T>>>) {
        // assume this is left rotation (side = left)
        let mut n = node.as_ref().borrow_mut();
        // 1. get child = n.right 
        if let Some(child_rc) = n.get_child(!side) {
            // 2. Turn child's left subtree into n's right subtree
            // get left subtree
            let mut child = child_rc.as_ref().borrow_mut();
            // child.left = x.right
            let other_child = n.get_child(side);
            n.set_child(!side, other_child);
            if let Some(val) = n.get_child(side) {
                let mut grandchild = val.as_ref().borrow_mut();
                grandchild.set_parent(Some(side), Some(node.clone()));
            }
            child.set_parent(n.get_is_child().clone(), n.get_parent().clone());
            if n.get_parent().is_none() {
                self.set_root(child_rc.clone());
            } else {
                // child is now left child of n's parent
                let n_parent_rc = n.get_parent().clone().unwrap();
                let mut n_parent = n_parent_rc.as_ref().borrow_mut();
                if n.is_child(side) {
                    n_parent.set_child(side, Some(child_rc.clone()));
                } else {
                    n_parent.set_child(!side, Some(child_rc.clone()));
                }
            }
            child.set_child(side, Some(node.clone()));
            n.set_parent(Some(side), Some(child_rc.clone()));
        }   
    }

    // fn delete(&mut self, k: T) {
    //     let root = self.get_root();
    //     if root.is_none() { return; }
    //     let (mut new_root, fix_tree) = bst_delete(root.clone(), k);
    //     if fix_tree {
    //         // new_root = self.delete_fix(inserted_node); // replace with actual fix function
    //         self.size -= 1;
    //     }
    //     new_root.as_ref().borrow_mut().set_color(NodeColor::Black);
    //     self.set_root(new_root);
    // }
}
