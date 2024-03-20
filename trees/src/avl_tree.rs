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
    fn get_balance_factor(&self) -> i8 {

        //declare left_height and right_height
        let mut left_height: i8 = 0;
        let mut right_height: i8 = 0;

        if let Some(left) = self.get_child(Side::Left){
            left_height = (left.as_ref().borrow().get_height()) as i8;
        }else{left_height = 0;}

        if let Some(right) = self.get_child(Side::Right){
            right_height = (right.as_ref().borrow().get_height()) as i8;
        }else{right_height = 0;}
    
        return (left_height - right_height);
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
        //This function runs after a node is inserted, we're given the node that is inserted
        //We need to check if the tree is balanced and if not, fix it

        //First, we need to update the height of the node and all of its ancestors
        self.refresh_height(node.clone());
        
        //Next, we need to check if the tree is balanced
        //check balancing factor of the node and its acestors
        let mut current_node = node.clone();
        loop{
            let temp = current_node.clone();
            let n = current_node.as_ref().borrow();
            let balance_factor = n.get_balance_factor();
            println!("For node {} Balance factor: {}", n.key, balance_factor);
            if balance_factor > 1 || balance_factor < -1 {
                
                //Tree is unbalanced, we need to fix it
                println!("Tree is unbalanced, fixing it");

                //check for which of the 4 cases the tree is unbalanced

                //case 1: bf > 1 and key value of node is less than key value of left child
                if balance_factor > 1 && 
                n.get_child(Side::Left).as_ref().unwrap().as_ref().borrow().greater(n.key){
                    println!("Case 1: Right rotation");
                    //self.rotate(Side::Right, current_node.clone());
                }
                
                //case 2: bf > 1 and key value of node is greater than key value of left child
                else if balance_factor > 1 &&
                n.get_child(Side::Left).as_ref().unwrap().as_ref().borrow().less(n.key){
                    println!("Case 2: Left-Right rotation");
                    //self.rotate(Side::Left, n.get_child(Side::Left).as_ref().unwrap().clone());
                    //self.rotate(Side::Right, current_node.clone());
                }

                //case 3: bf < -1 and key value of node is greater than key value of right child
                else if balance_factor < -1 &&
                n.get_child(Side::Right).as_ref().unwrap().as_ref().borrow().greater(n.key){
                    println!("Case 3: Left rotation");
                    let temp_clone = temp.clone();
                    
                    self.rotate(Side::Left, temp_clone);
                }

                //case 4: bf < -1 and key value of node is less than key value of right child
                else if balance_factor < -1 &&
                n.get_child(Side::Right).as_ref().unwrap().as_ref().borrow().less(n.key){
                    println!("Case 4: Right-Left rotation");
                    //self.rotate(Side::Right, n.get_child(Side::Right).as_ref().unwrap().clone());
                    //self.rotate(Side::Left, current_node.clone());
                }
                
                //after fixing the tree, we need to update the height of the node and all of its ancestors
                //self.refresh_height(current_node.clone());
            }

            
            if let Some(p) = n.get_parent() {
                drop(n);
                current_node = p;
            } else {
                break;
            }
        }



        //lets return root of the node 
        drop(current_node);
        let mut current_node = node.clone();
        let temp_clone = current_node.clone();
        let a = current_node.as_ref().borrow_mut();
        let not_root = a.get_parent().is_some();
        drop(a);
        let root = if not_root { self.climb_to_root(temp_clone) } else { temp_clone };

        root
    }

    fn rotate(&mut self, side: Side, node: Rc<RefCell<AvlTreeNode<T>>>) {
        //here side means the direction of rotation

        let mut n = node.as_ref().borrow_mut();

        //first we check the side of rotation
        if side == Side::Left {
            //left rotation
            //we need to rotate the node to the left

            //first we need to get the right child of the node
            let right_child_ptr = n.get_child(Side::Right);
            let mut right_child = right_child_ptr.as_ref().unwrap().as_ref().borrow_mut();

            //next we need to save the left child of the right child if there is any
            if let Some(left_grandchild) = right_child.get_child(Side::Left){
                //if there is a left grandchild, we need to set it as the right child of the node
                n.set_child(Side::Right, Some(left_grandchild.clone()));

                //we also need to set the node as the parent of the left grandchild
                left_grandchild.as_ref().borrow_mut().set_parent(Some(Side::Right), Some(node.clone()));
            }

            drop(right_child);
            //next we need to set the right child as the parent of the node
            n.set_parent(Some(Side::Left), Some(right_child_ptr.as_ref().unwrap().clone()));
            //and set the node as the left child of the right child
            let mut right_child = right_child_ptr.as_ref().unwrap().as_ref().borrow_mut();
            right_child.set_child(Side::Left, Some(node.clone()));

            //finally we need to update the height of the node and the right child
            if let Some(grand_child) = n.get_child(Side::Right){
                grand_child.as_ref().borrow_mut().update_height();
            }else{
                n.update_height();
            }
        }
    }
}


impl<T> AvlTree<T>
where
T: Ord + Copy + std::fmt::Debug + std::fmt::Display
{

    pub fn refresh_height(&mut self, node: Rc<RefCell<AvlTreeNode<T>>>) {
        let mut current_node = node.clone();
        //travel up the tree to update the height of the all acestors nodes
        loop {
            let mut n = current_node.as_ref().borrow_mut();
            n.update_height();
            if let Some(p) = n.get_parent() {
                drop(n);
                current_node = p;
            } else {
                break;
            }
        }
    }

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