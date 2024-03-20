use crate::tree::*;
use crate::node::*;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Copy)]
enum NodeColor {
    Red,
    Black,
}

// used by print_node_structure to print branches nicely
enum NodeIsFrom {
    Left,
    Right,
    Neither
}

type MaybeRedBlackTree<T> = Option<Rc<RefCell<RedBlackTreeNode<T>>>>;

#[derive(Debug)]
pub struct RedBlackTreeNode<T> {
    color: NodeColor,
    pub key: T,
    pub parent: MaybeRedBlackTree<T>,
    is_child: Option<Side>,
    left: MaybeRedBlackTree<T>,
    right: MaybeRedBlackTree<T>,
}

pub struct RedBlackTree<T> {
    root: MaybeRedBlackTree<T>,
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

    // return the child from the given side
    fn get_child(&self, side: Side) -> MaybeRedBlackTree<T> {
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

    fn take_child(&mut self, side: Side) -> MaybeRedBlackTree<T> {
        match side {
            Side::Left => self.left.take(),
            Side::Right => self.right.take(),           
        }
    }

    // attach a child node to its parent
    fn set_child(&mut self, side: Side, child: MaybeRedBlackTree<T>) {
        match side {
            Side::Left => self.left = child,
            Side::Right => self.right = child,
        }
    }

    fn child_count(&self) -> usize {
        if self.get_child(Side::Left).is_none() && self.get_child(Side::Left).is_none() {
            0    
        } else if self.get_child(Side::Left).is_some() && self.get_child(Side::Right).is_some() {
            2
        } else {
            1
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

    fn get_parent(&self) -> MaybeRedBlackTree<T> {
        self.parent.clone()
    }

    fn get_parent_mut(&mut self) -> &mut MaybeRedBlackTree<T> {
        self.parent.borrow_mut()
    }

    // attach a parent node to its child
    fn set_parent(&mut self, is_child: Option<Side>, parent: MaybeRedBlackTree<T>) {
        self.parent = parent;
        self.is_child = is_child;
    }   

}

impl<T: Ord + std::fmt::Debug + std::fmt::Display> RedBlackTreeNode<T> {
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

    fn print_structure_node(&self, depth: usize, from: NodeIsFrom) {
        // first go left...
        if let Some(ptr) = &self.left {
            ptr.as_ref().borrow().print_structure_node(depth + 1, NodeIsFrom::Left);            
        }
        
        // print this node with prefix
        // get R/B as char 
        let colour = if self.is_red() { 'R' } else { 'B' };
        // couple characters that make tree look 'smooth'
        let smooth = match from {
            NodeIsFrom::Left => "┌───────┘",
            NodeIsFrom::Neither => "",
            NodeIsFrom::Right => "└───────┐"
        };
        // case for depth to make lines line up nice
        let space = if depth == 0 { 0 } else { (depth - 1) * 8 };
        // print line representing this node
        println!("{: <1$}{2}{3} ({4})", "", space, smooth, self.get_key(), colour);
        
        // then go right...
        if let Some(ptr) = &self.right {
            ptr.as_ref().borrow().print_structure_node(depth + 1, NodeIsFrom::Right);
        }
    }


    fn count_leaves_node(&self) -> usize {
        if self.is_leaf() {
            return 1;
        }
        let mut count: usize = 0;
        // otherwise, first go left for lower values
        if let Some(ptr) = &self.left {
            count += ptr.as_ref().borrow().count_leaves_node();
        }
        // then go right for higher values
        if let Some(ptr) = &self.right {
            count += ptr.as_ref().borrow().count_leaves_node();
        }
        return count;
    }

    fn get_height_node(&self, depth: usize) -> usize {
        // recursive helper function for get_height of tree
        if self.is_leaf() {
            return depth;
        }
        // find max depth of either branch
        let mut m: usize = usize::MIN;
        // otherwise, first go left for lower values
        if let Some(ptr) = &self.left {
            m = std::cmp::max(m, ptr.as_ref().borrow().get_height_node(depth + 1));
        }
        // then go right for higher values
        if let Some(ptr) = &self.right {
            m = std::cmp::max(m, ptr.as_ref().borrow().get_height_node(depth + 1));
        }
        return m;
    }
}

impl<T: Ord + Copy + std::fmt::Debug + std::fmt::Display> Tree<T> for RedBlackTree<T> {
    type Node = RedBlackTreeNode<T>;

    fn new() -> Self {
        Self { root: None }
    }

    fn get_root(&self) -> &MaybeRedBlackTree<T> {
        &self.root
    }

    fn set_root(&mut self, node: Option<Rc<RefCell<RedBlackTreeNode<T>>>>) {
        self.root = node.clone();
        match node {
            None => (),
            Some(ptr) => {
                let mut n = ptr.as_ref().borrow_mut();
                n.set_color(NodeColor::Black);
            }
        }
    }

    // PART 1.1 insert a node
    // fn insert(&mut self, key: T) {
    //     // first insert node as though in a BST
    //     let root = self.get_root();
    //     let (mut new_root, inserted_node, fix_tree) = bst_insert(root.clone(), key);

    //     if fix_tree {
    //         new_root = self.insert_fix(inserted_node);
    //         self.size += 1;
    //     }
    //     new_root.as_ref().borrow_mut().set_color(NodeColor::Black);
    //     self.set_root(new_root);
    // } 

    // rebalance the tree after insertion
    fn insert_fix(&mut self, node: Rc<RefCell<RedBlackTreeNode<T>>>) -> Rc<RefCell<RedBlackTreeNode<T>>> {
        let mut current_node = node.clone();
        let mut n = current_node.as_ref().borrow();
        let mut not_root = n.get_parent().is_some();
        let mut red_parent = n.parent_is_red();

        while not_root && red_parent {
            drop(n);
            let temp = current_node.clone();

            if temp.as_ref().borrow().uncle_is_red() {
                self.recolor_ins(temp.clone());
                if let Some(grandparent) = temp.as_ref().borrow().get_grandparent() {
                    current_node = grandparent.clone();
                }
                n = current_node.as_ref().borrow();
                red_parent = n.parent_is_red();
                not_root = n.get_parent().is_some();

            }
            else {
                let temp_clone = temp.clone();
                self.rotate_ins(temp_clone);
                n = current_node.as_ref().borrow();
                red_parent = current_node.as_ref().borrow().parent_is_red();
            }
        }
        drop(n);
        let temp_clone = current_node.clone();
        let root = if not_root { self.climb_to_root(temp_clone) } else { temp_clone };

        root
    }
    
    fn rotate(&mut self, side: Side, node: Rc<RefCell<RedBlackTreeNode<T>>>) {

        let mut n = node.as_ref().borrow_mut();

        if let Some(child_ptr) = n.get_child(!side) {
            {
                let mut child = child_ptr.as_ref().borrow_mut();
                let other_child = n.get_child(side);
                n.set_child(!side, other_child);
                child.set_parent(n.get_is_child().clone(), n.get_parent().clone());
            }

            if let Some(ptr) = n.get_child(side) {
                let mut grandchild = ptr.as_ref().borrow_mut();
                grandchild.set_parent(Some(side), Some(node.clone()));
            }
            

            if n.get_parent().is_none() {
                self.set_root(Some(child_ptr.clone()));
            } else {
                let parent_ptr = n.get_parent().clone().unwrap();
                let mut parent = parent_ptr.as_ref().borrow_mut();
                if n.is_child(side) {
                    parent.set_child(side, Some(child_ptr.clone()));
                } else {
                    parent.set_child(!side, Some(child_ptr.clone()));
                }
            }

            let mut child = child_ptr.as_ref().borrow_mut();
            child.set_child(side, Some(node.clone()));
            n.set_parent(Some(side), Some(child_ptr.clone()));
        }   
    }

    // fn delete(&mut self, k: T) {
        // similar to insert
        // let root = self.get_root();
        // bst_delete(root.clone(), k); // should return root here 
        // if fix_tree {
        //     // new_root = self.delete_fix(inserted_node); // replace with actual fix function
        //     self.size -= 1;
        // }
        // new_root.as_ref().borrow_mut().set_color(NodeColor::Black);
        // self.set_root(new_root);
    // }

}

impl<T> RedBlackTree<T> 
where 
    T: Ord + Copy + std::fmt::Debug + std::fmt::Display
{
    pub fn print_inorder(&self) {
        // PART 1.5 print in-order traversal of tree
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
        // PART 1.7 print tree showing structure and colours
        println!("------- Tree Structure -------");
        if let Some(ptr) = &self.root {
            let root = ptr.as_ref().borrow();
            root.print_structure_node(0, NodeIsFrom::Neither);
        }
        else {
            println!("Empty tree");
        }       
        println!("------------------------------");
    }

    pub fn count_leaves(&self) -> usize {
        // PART 1.3 count leaves in tree
        if let Some(ptr) = &self.root {
            let root = ptr.as_ref().borrow();
            return root.count_leaves_node();
        }
        else {
            return 0;
        }
    }

    pub fn is_empty(&self) -> bool {
        // PART 1.6 check if tree empty
        if let None = &self.root {
            return true;
        } else {
            return false;
        }
    } 

    pub fn get_height(&self) -> usize {
        // PART 1.4 get height of tree
        if let Some(ptr) = &self.root {
            let root: std::cell::Ref<'_, RedBlackTreeNode<T>> = ptr.as_ref().borrow();
            return root.get_height_node(1);
        }
        else {
            return 0;
        }
    }

    // traverse up the tree from the given node and return the root
    fn climb_to_root(&self, node: Rc<RefCell<RedBlackTreeNode<T>>>) -> Rc<RefCell<RedBlackTreeNode<T>>> {
        let parent = node.as_ref().borrow().get_parent();
        if parent.is_none() {{}
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

    
    fn rotate_ins(&mut self, node: Rc<RefCell<RedBlackTreeNode<T>>>) {
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

    fn recolor_ins(&self, node: Rc<RefCell<RedBlackTreeNode<T>>>) {
        let n = node.as_ref().borrow();
        if n.get_grandparent().is_some() && n.get_uncle().is_some() {
            n.get_grandparent().unwrap().as_ref().borrow_mut().set_color(NodeColor::Red);
            n.get_parent().unwrap().as_ref().borrow_mut().set_color(NodeColor::Black);
            n.get_uncle().unwrap().as_ref().borrow_mut().set_color(NodeColor::Black);
        }
    }

}