use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;
use crate::tree::*;
use crate::node::*;
use std::borrow::{Borrow, BorrowMut};

type MaybeAvlTree<T> = Option<Rc<RefCell<AvlTreeNode<T>>>>;

#[derive(Debug)]
pub struct AvlTreeNode<T> {
    pub key: T,
    pub parent: MaybeAvlTree<T>,
    pub height: usize,
    pub left: MaybeAvlTree<T>,
    pub right: MaybeAvlTree<T>,
    pub is_child: Option<Side>,
}

pub struct AvlTree<T> {
    root: MaybeAvlTree<T>,
}

impl<T: Ord> Traversible<T> for AvlTreeNode<T> {
    fn left(&self) -> &Option<Rc<RefCell<Self>>> {
        return &self.left;
    }
    fn right(&self) -> &Option<Rc<RefCell<Self>>> {
        return &self.right;
    }
}

impl<T: Ord> Node<T> for AvlTreeNode<T>{

    fn new(key: T) -> Self {
        Self{
            key: key,
            left: None,
            right: None,
            parent: None,
            height: 1,
            is_child: None,
            }
    }

    fn get_key(&self) -> &T {
        &self.key
    }

    fn set_key(&mut self, val: T) {
        self.key = val;
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

    fn get_child(&self, side: Side) -> MaybeAvlTree<T> {
        match side {
            Side::Left => self.left.clone(),
            Side::Right => self.right.clone(),
        }
    }

    // check if the node is a left or right child of another node
    fn is_child(&self, side: Side) -> bool {
        match &self.is_child {
            None => false,
            Some(val) => {
                if val == &side { true } else { false }
            }
        }
    }

    // if node has a parent, return the side it is on
    fn get_is_child(&self) -> &Option<Side> {
        &self.is_child
    }

    fn take_child(&mut self, side: Side) -> MaybeAvlTree<T> {
        match side {
            Side::Left => self.left.take(),
            Side::Right => self.right.take(),           
        }
    }

    fn set_child(&mut self, side: Side, child: MaybeAvlTree<T>) {
        match side {
            Side::Left => self.left = child,
            Side::Right => self.right = child,
        }
    }

    fn is_leaf(&self) -> bool {
        // check left and right pointers to determine if this node is a leaf node
        if let None = self.left {
            if let None = self.right {
                return true;
            }
        }
        return false;
    }

    fn get_sibling(&self) -> MaybeAvlTree<T> {
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

    fn get_uncle(&self) -> MaybeAvlTree<T> {
        if let Some(p) = self.get_parent() {
            p.as_ref().borrow().get_sibling()
        } else { None }
    } 

    fn get_grandparent(&self) -> MaybeAvlTree<T> {
        if let Some(p) = self.get_parent() {
            p.as_ref().borrow().get_parent()
        } else { None }
    } 

    fn get_parent(&self) -> MaybeAvlTree<T> {
        self.parent.clone()
    }

    fn get_parent_mut(&mut self) -> &mut MaybeAvlTree<T> {
        self.parent.borrow_mut()
    }

    fn set_parent(&mut self, is_child: Option<Side>, parent: MaybeAvlTree<T>) {
        self.parent = parent;
        self.is_child = is_child;
    }   
}

impl<T: Ord + std::fmt::Debug + std::fmt::Display >  AvlTreeNode<T> {

    fn get_height(&self) -> usize {
        self.height
    }

    fn set_height(&mut self, height: usize) {
        self.height = height;
    }

    fn update_height(&mut self) {
        let left_height: usize;
        let right_height: usize;

        if let Some(left) = self.get_child(Side::Left){
            left_height = left.as_ref().borrow().get_height();
        }else{left_height = 0;}

        if let Some(right) = self.get_child(Side::Right){
            right_height = right.as_ref().borrow().get_height();
        }else{right_height = 0;}
        
        let calculated_height = 1 + max(left_height, right_height);
        self.set_height(calculated_height);
    }

    // Helper function to get the balance factor of a node.
    fn get_balance_factor(&self) -> usize {

        //declare left_height and right_height
        let mut left_height: usize = 0;
        let mut right_height: usize = 0;

        if let Some(left) = self.get_child(Side::Left){
            left_height = left.as_ref().borrow().get_height();
        }else{left_height = 0;}

        if let Some(right) = self.get_child(Side::Right){
            right_height = right.as_ref().borrow().get_height();
        }else{right_height = 0;}
    
        return left_height - right_height;
    }

    fn print_inorder_node(&self) {
        // function called recursively to traverse nodes in order and print values
        // if this is a leaf node, print its value
        if self.is_leaf() {
            println!("{:?}", self.get_key());
            return;
        }
        // otherwise, first go left for lower values
        if let Some(ptr) = &self.left {
            ptr.as_ref().borrow().print_inorder_node();
        }
        // then print this node's value
        println!("{:?}", self.get_key());
        // then go right for higher values
        if let Some(ptr) = &self.right {
            ptr.as_ref().borrow().print_inorder_node();
        }
    }

    fn print_structure_node(&self, depth: usize) {
        unimplemented!()
    }

}

impl<T: Ord + Copy + std::fmt::Debug + std::fmt::Display> Tree<T> for AvlTree<T> {
    type Node = AvlTreeNode<T>;

    fn new() -> Self {
        Self {root: None}
    }

    fn get_root(&self) -> &MaybeAvlTree<T> {
        &self.root
    }

    fn set_root(&mut self, node: Option<Rc<RefCell<AvlTreeNode<T>>>>) {
        self.root = node.clone();
        match node{
            None => {},
            Some(ptr) => {
                let mut n = ptr.as_ref().borrow_mut();
                n.update_height();
            }
        }

    }

    fn insert_fix(&mut self, node: Rc<RefCell<AvlTreeNode<T>>>) -> Rc<RefCell<AvlTreeNode<T>>> {
        unimplemented!()
    }

    fn rotate(&mut self, side: Side, node: Rc<RefCell<AvlTreeNode<T>>>) {
        unimplemented!()
    }
}

impl<T> AvlTree<T>
where
T: Ord + Copy + std::fmt::Debug + std::fmt::Display
{
    pub fn print_inorder(&self) {
        // PART 2.5 print in-order traversal of tree
        println!("-------- Tree In-Order -------");
        if let Some(ptr) = &self.root {
            let root = ptr.as_ref().borrow();
            root.print_inorder_node();
        }
        else {
            println!("Empty tree");
        }
        println!("------------------------------");
    }

    pub fn print_structure(&self) {
        // PART 2.7 print tree showing structure and colours
       println!("------- Tree Structure -------");
        if let Some(ptr) = &self.root {
            let root = ptr.as_ref().borrow();
            //root.print_structure_node(0, NodeIsFrom::Neither);
        }
        else {
            println!("Empty tree");
        }       
        println!("------------------------------");
    }
}