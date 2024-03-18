use std::cell::RefCell;
use std::rc::Rc;
use crate::tree::*;
use std::borrow::{Borrow, BorrowMut};

type MaybeAvlTree<T> = Option<Rc<RefCell<AvlTreeNode<T>>>>;

#[derive(Debug, PartialEq, Clone)]
struct AvlTreeNode<T> {
    key: T,
    left: MaybeAvlTree<T>,
    right: MaybeAvlTree<T>,
    parent: MaybeAvlTree<T>,
    height: i32,
    is_child: Option<Side>,
}

struct AvlTree<T> {
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

impl<T: Ord + Copy + std::fmt::Debug> AvlTree<T> {

    // Helper function to get the height of a node.
    fn height(node: &MaybeAvlTree<T>) -> i32 {
        match node {
            Some(n) => n.borrow().height,
            None => 0, // Consider changing this to -1 if you define height of `None` as -1.
        }
    }

    fn get_balance_factor(&self, root: AvlTree<T>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let left_height = root.left.height;
        let right_height = root.right.height;
        return (left_height - right_height) as i32;
    }

    fn min_value_node(&self, node: AvlTreeNode<T>) -> AvlTreeNode<T> {
        let mut current = node;

        if node.is_none() | node.left.is_none() {
            return node;
        }else{
            return self.min_value_node(node.left);
        }
    }   


    fn insert(&self, root:AvlTree<T>, key: T) {

        //if root is none
        
        //  return AvlTreeNode::new(key);
        
        //elif key < root.key
        
        //  root.left = root.left.insert(key);

        //else
        
        //  root.right = root.right.insert(key);

        //root.height = 1 + max(root.left.height, root.right.height);

        //get balance factor
        //let bf = self.get_balance_factor(root);

        //if bf > 1 and key < root.left.key

        //return self.right_rotate();

        //if bf < -1 and key > root.right.key

        //return self.left_rotate();

        //if bf > 1 and key > root.left.key

        //root.left = root.left.left_rotate();

        //return self.right_rotate();

        //if bf < -1 and key < root.right.key

        //root.right = root.right.right_rotate();

        //return self.left_rotate();

    }

    fn left_rotate(&self) {

        //let &mut cur_right = self.right;

        //let &mut cur_right_left_child = cur_right.left;

        //cur_right.left = self;  
        //self.right = cur_right_left_child;

        //self.height = 1 + max(self.left.height, self.right.height);
        //cur_right.height = 1 + max(cur_right.left.height, cur_right.right.height);

        //return cur_right;
    }

    fn right_rotate(&self) {

        //let &mut cur_left = self.left;

        //let &mut cur_left_right_child = cur_left.right;

        //cur_left.right = self;  
        //self.left = cur_left_right_child;

        //self.height = 1 + max(self.left.height, self.right.height);
        //cur_left.height = 1 + max(cur_left.left.height, cur_left.right.height);

        //return cur_left;
    }

    fn search(&self,root:AvlTree<T>, key: T) -> AvlTreeNode<T> {
        
        //let mut current = self.root;

        //while current is not none and key != current.key{

        //  if key < current.key{

        //      current = current.left;
        
        //  } else {

        //      current = current.right;

        //  }

        //return current;
    }

    ///Start from the root and traverse the tree to 
    /// find the node to be deleted
    fn delete(&self, root: AvlTree<T>, key: T) {

        //if root is none

        //  return root;

        //elif key < root.key

        //  root.left = root.left.delete(key);

        //else if key > root.key

        //  root.right = root.right.delete(key);

        //else

        //   if root.left is none

        //      temp = root.right;

        //      root = None;

        //      return temp;

        //    elif root.right is none
        
        //      temp = root.left;

        //      root = None;

        //      return temp;

        //    temp = self.min_value_node(root.right);

        //    root.key = temp.key;

        //    root.right = root.right.delete(temp.key);

        //  root.height = 1 + max(root.left.height, root.right.height);

        //  let bf = self.get_balance_factor(root);

        //  if bf > 1 and self.get_balance_factor(root.left) >= 0

        //      return self.right_rotate();

        //  if bf > 1 and self.get_balance_factor(root.left) < 0

        //      root.left = root.left.left_rotate();

        //      return self.right_rotate();

        //  if bf < -1 and self.get_balance_factor(root.right) <= 0

        //      return self.left_rotate();

        //  if bf < -1 and self.get_balance_factor(root.right) > 0

        //      root.right = root.right.right_rotate();

        //      return self.left_rotate();

        //  return root;
    }

}