use std::cell::RefCell;
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

    fn set_height(&mut self, height: usize) {
        self.height = height;
    }

    fn get_height(&self) -> usize {
        self.height
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

impl<T: Ord + std::fmt::Debug + std::fmt::Display>  AvlTreeNode<T> {

    // Helper function to get the balance factor of a node.
    fn get_balance_factor(&self) -> i32 {

        //declare left_height and right_height
        let left_height: i32=0;
        let right_height: i32=0;

        //if left is not none
    
        return left_height - right_height;
    }

}

//     fn insert(&self, root:AvlTree<T>, key: T) {

//         //if root is none
        
//         //  return AvlTreeNode::new(key);
        
//         //elif key < root.key
        
//         //  root.left = root.left.insert(key);

//         //else
        
//         //  root.right = root.right.insert(key);

//         //root.height = 1 + max(root.left.height, root.right.height);

//         //get balance factor
//         //let bf = self.get_balance_factor(root);

//         //if bf > 1 and key < root.left.key

//         //return self.right_rotate();

//         //if bf < -1 and key > root.right.key

//         //return self.left_rotate();

//         //if bf > 1 and key > root.left.key

//         //root.left = root.left.left_rotate();

//         //return self.right_rotate();

//         //if bf < -1 and key < root.right.key

//         //root.right = root.right.right_rotate();

//         //return self.left_rotate();

//     }

//     fn left_rotate(&self) {

//         //let &mut cur_right = self.right;

//         //let &mut cur_right_left_child = cur_right.left;

//         //cur_right.left = self;  
//         //self.right = cur_right_left_child;

//         //self.height = 1 + max(self.left.height, self.right.height);
//         //cur_right.height = 1 + max(cur_right.left.height, cur_right.right.height);

//         //return cur_right;
//     }

//     fn right_rotate(&self) {

//         //let &mut cur_left = self.left;

//         //let &mut cur_left_right_child = cur_left.right;

//         //cur_left.right = self;  
//         //self.left = cur_left_right_child;

//         //self.height = 1 + max(self.left.height, self.right.height);
//         //cur_left.height = 1 + max(cur_left.left.height, cur_left.right.height);

//         //return cur_left;
//     }

    //fn search(&self,root:AvlTree<T>, key: T) -> AvlTreeNode<T> {
        
//         //let mut current = self.root;

//         //while current is not none and key != current.key{

//         //  if key < current.key{

//         //      current = current.left;
        
//         //  } else {

//         //      current = current.right;

//         //  }

        //return current;
    //}

//     ///Start from the root and traverse the tree to 
//     /// find the node to be deleted
//     fn delete(&self, root: AvlTree<T>, key: T) {

//         //if root is none

//         //  return root;

//         //elif key < root.key

//         //  root.left = root.left.delete(key);

//         //else if key > root.key

//         //  root.right = root.right.delete(key);

//         //else

//         //   if root.left is none

//         //      temp = root.right;

//         //      root = None;

//         //      return temp;

//         //    elif root.right is none
        
//         //      temp = root.left;

//         //      root = None;

//         //      return temp;

//         //    temp = self.min_value_node(root.right);

//         //    root.key = temp.key;

//         //    root.right = root.right.delete(temp.key);

//         //  root.height = 1 + max(root.left.height, root.right.height);

//         //  let bf = self.get_balance_factor(root);

//         //  if bf > 1 and self.get_balance_factor(root.left) >= 0

//         //      return self.right_rotate();

//         //  if bf > 1 and self.get_balance_factor(root.left) < 0

//         //      root.left = root.left.left_rotate();

//         //      return self.right_rotate();

//         //  if bf < -1 and self.get_balance_factor(root.right) <= 0

//         //      return self.left_rotate();

//         //  if bf < -1 and self.get_balance_factor(root.right) > 0

//         //      root.right = root.right.right_rotate();

//         //      return self.left_rotate();

//         //  return root;
//     }

// }

// impl<T: Ord> AvlTree<T> where 
// T: Ord + Copy + std::fmt::Debug + std::fmt::Display
// {
    
//     fn update_height(&self, node: Rc<RefCell<AvlTreeNode<T>>>) {
//         // let mut node = node.as_ref().borrow_mut();
//         let mut current_node = node.clone();
//         loop{
//             if let Some(ptr) = current_node.as_ref().borrow().get_parent() {
//                 let parent = ptr.as_ref().borrow();
//                 println!("Updating height of node: {:?}", parent.key);
//                 let mut left_height: i32 = 0;
//                 let mut right_height: i32 = 0;

// impl<T: Ord + std::fmt::Debug + std::fmt::Display> AvlTree<T> {
//     pub fn new() -> Self {
//         Self {
//             root: None,
//         }
//     }

//     fn get_root(&self) -> MaybeAvlTree<T> {
//         self.root.clone()
//     }

//     fn set_root(&mut self, node: MaybeAvlTree<T>) {
//         self.root = node;
//     }

// }