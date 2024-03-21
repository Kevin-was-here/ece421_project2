use super::tree::*;
use super::node::*;
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
    pub key: Option<T>,
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

impl<T: Ord + Clone> Node<T> for RedBlackTreeNode<T> {
    fn new(key: T) -> Self {
        Self {
            color: NodeColor::Red,
            key: Some(key),
            parent: None,
            is_child: None, // left or right child of its parent
            left: None,
            right: None,
        }
    }

    fn get_key(&self) -> T {
        self.key.clone().unwrap()
    }

    fn set_key(&mut self, val: T) {
        self.key = Some(val);
    }
    
    fn greater(&self, val: T) -> bool {
        self.get_key() > val
    }

    fn equal(&self, val: T) -> bool {
        self.get_key() == val
    }

    fn less(&self, val: T) -> bool {
        self.get_key() < val
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

impl<T: Ord + std::fmt::Debug + std::fmt::Display + Clone> RedBlackTreeNode<T> {
    fn nil() -> Self {
        Self {
            color: NodeColor::Black,
            key: None,
            parent: None,
            is_child: None, // left or right child of its parent
            left: None,
            right: None,
        }
    }

    
    fn is_red(&self) -> bool {
        self.color == NodeColor::Red
    }

    fn is_nil(&self) -> bool {
        self.key.is_none()
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

    fn rotate(&mut self, side: Side, x: Rc<RefCell<RedBlackTreeNode<T>>>) {
        // LEFT ROTATE
        // y = x.right
        if self.get_child(x.clone(), !side).is_none() {
            return
        }
        let y = self.get_child(x.clone(), !side).unwrap();

        // x.right = y.left (A)
        let y_left = self.get_child(y.clone(), side);
        self.set_child(x.clone(), !side, y_left.clone());

        // if (y.left) != NULL
        //     (y.left).parent = x
        if let Some(yl) = y_left.clone() {
            self.set_parent(yl.clone(), Some(!side), Some(x.clone()));
        }

        // y.parent = x.parent
        let x_parent = self.get_parent(x.clone());
        let x_is_child = self.get_is_child(x.clone());
        self.set_parent(y.clone(), x_is_child.clone(), x_parent.clone());

        // if x.parent == NULL //x is root
        //     T.root = y
        if x_parent.is_none() {
            self.set_root(Some(y.clone()));
        } else {
            // elseif x == x.parent.left // x is left child
            //     x.parent.left = y
            // else // x is right child
            //     x.parent.right = y
            let child_side = x_is_child.clone().unwrap();
            self.set_child(x_parent.unwrap().clone(), child_side.clone(), Some(y.clone()));
        }

        // y.left = x
        // x.parent = y
        self.set_child(y.clone(), side, Some(x.clone()));
        self.set_parent(x.clone(), x_is_child, Some(y.clone()));

    }


    fn insert_fix(&mut self, node: Rc<RefCell<RedBlackTreeNode<T>>>) -> Rc<RefCell<RedBlackTreeNode<T>>> {
        let p = self.get_parent(node.clone());
        if p.is_none() || !self.is_red(p.clone()) {
            return self.climb_to_root(node.clone())
            // return
        }

        let mut parent = p.unwrap();
        let gp = self.get_grandparent(node.clone());
        if gp.is_none() {
            self.set_color(parent.clone(), NodeColor::Black);
            return self.climb_to_root(node.clone())
            // return
        }

        let grandparent = gp.unwrap();
        let uncle = self.get_uncle(node.clone());
        if uncle.is_some() && self.is_red(uncle.clone()) {
            self.recolor_ins(node.clone());
            self.insert_fix(grandparent.clone());
        }

        else {
            self.rotate_ins(node.clone());
        }

        self.climb_to_root(node.clone())
    }

}

impl<T> RedBlackTree<T> 
where 
    T: Ord + Copy + std::fmt::Debug + std::fmt::Display
{
    
    fn get_sibling(&self, node: Rc<RefCell<RedBlackTreeNode<T>>>) -> MaybeRedBlackTree<T> {
        node.as_ref().borrow().get_sibling()
    }

    fn get_uncle(&self, node: Rc<RefCell<RedBlackTreeNode<T>>>) -> MaybeRedBlackTree<T> {
        node.as_ref().borrow().get_uncle()
    }

    fn get_grandparent(&self, node: Rc<RefCell<RedBlackTreeNode<T>>>) -> MaybeRedBlackTree<T> {
        node.as_ref().borrow().get_grandparent()
    }

    fn get_color(&self, node: Rc<RefCell<RedBlackTreeNode<T>>>) -> NodeColor {
        node.as_ref().borrow().get_color()
    }

    fn set_color(&self, node: Rc<RefCell<RedBlackTreeNode<T>>>, color: NodeColor) {
        node.as_ref().borrow_mut().set_color(color);
    } 

    fn equal(&self, a: MaybeRedBlackTree<T>, b: MaybeRedBlackTree<T>) -> bool {
        if a.is_none() && b.is_none() {
            true
        } else if a.is_some() && b.is_some() {
            let a_ptr = a.unwrap();
            let b_ptr = b.unwrap();
            if self.is_nil(a_ptr.clone()) && self.is_nil(b_ptr.clone()) {
                true
            } else if !self.is_nil(a_ptr.clone()) && !self.is_nil(b_ptr.clone()) {
                a_ptr.as_ref().borrow().equal(b_ptr.as_ref().borrow().get_key().clone())
            } else {
                false
            }
        } else {
            false
        }
    }

    fn is_red(&self, node: MaybeRedBlackTree<T>) -> bool {
        match node {
            None => false,
            Some(n) => n.as_ref().borrow().is_red(),
        }
    }

    fn is_nil(&self, node: Rc<RefCell<RedBlackTreeNode<T>>>) -> bool {
        node.as_ref().borrow().is_nil()
    }

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

    fn replace(&mut self, node: Rc<RefCell<RedBlackTreeNode<T>>>) -> Option<Rc<RefCell<RedBlackTreeNode<T>>>> {
        let parent = self.get_parent(node.clone());

        if let Some(left) = self.left(node.clone())  {
            self.replace_parent_child(parent.clone(), node.clone(), Some(left.clone()));
            Some(left.clone())
            
        } else if let Some(right) = self.right(node.clone())  {
            self.replace_parent_child(parent.clone(), node.clone(), Some(right.clone()));
            Some(right.clone())

        } else {
            let new_child = match self.is_red(Some(node.clone())) {
                false => {
                    let nil_node: RedBlackTreeNode<T> = RedBlackTreeNode::nil();
                    Some(Rc::new(RefCell::new(nil_node)))
                },
                true => {
                    None
                }
            };
            self.replace_parent_child(parent.clone(), node.clone(), new_child.clone());
            new_child.clone()
        }
    }

    fn delete_fix(&mut self, node: Rc<RefCell<RedBlackTreeNode<T>>>) {
        
        if self.get_parent(node.clone()).is_none() {
            node.as_ref().borrow_mut().set_color(NodeColor::Black);
            return;
        } 

        let mut sibling_ptr = self.get_sibling(node.clone());

        // Case 1
        if self.is_red(sibling_ptr.clone()) {
            self.delete_fix_case_1(node.clone(), sibling_ptr.clone().unwrap());
            sibling_ptr = sibling_ptr.unwrap().as_ref().borrow().get_sibling();
        }


        // Case 2+3
        let sibling = sibling_ptr.unwrap();
        let sibling_left = self.left(sibling.clone());
        let sibling_right = self.right(sibling.clone());
        
        if !self.is_red(sibling_left.clone()) && !self.is_red(sibling_right.clone()) {
            sibling.as_ref().borrow_mut().set_color(NodeColor::Red);

            let node_parent = self.get_parent(node.clone());
            let p = node_parent.clone().unwrap();

            if self.is_red(node_parent.clone()) {
                p.as_ref().borrow_mut().set_color(NodeColor::Black);
            } else {
                // Case 4
                self.delete_fix(node_parent.clone().unwrap());
            }
        } else {
            self.delete_fix_case_5(node.clone(), sibling.clone());
        }

    }

    pub fn delete(&mut self, k: T) {
        let search = self.bst_find(self.get_root().clone(), k);
        if search.is_none() {
            return;
        }

        let node = search.unwrap();
        let deleted_color;
        let moved_up_node;

        if self.left(node.clone()).is_none() || self.right(node.clone()).is_none() {
            moved_up_node = self.replace(node.clone());
            deleted_color = node.as_ref().borrow().get_color();
        } else {
            let right_child = self.right(node.clone()).unwrap();
            let successor = self.find_min(right_child.clone());
            let new_key = self.get_key(successor.clone());
            self.set_key(node.clone(), new_key);
            deleted_color = successor.as_ref().borrow().get_color();
            moved_up_node = self.replace(successor.clone());
        }

        if deleted_color == NodeColor::Black {
            self.delete_fix(moved_up_node.clone().unwrap());
            if self.is_nil(moved_up_node.clone().unwrap()) {
                let nil_parent = self.get_parent(moved_up_node.unwrap().clone());
                self.replace_parent_child(nil_parent, node.clone(), None);  
            }
        }   
    }

    fn delete_fix_case_1(&mut self, node: Rc<RefCell<RedBlackTreeNode<T>>>, sibling: Rc<RefCell<RedBlackTreeNode<T>>>) {
        let p = self.get_parent(node.clone());
        
        let sibling_ptr = sibling.clone();
        let parent_ptr = p.unwrap();

        sibling_ptr.as_ref().borrow_mut().set_color(NodeColor::Black);
        parent_ptr.as_ref().borrow_mut().set_color(NodeColor::Red);

        let left = self.left(parent_ptr.clone());
        let right = self.right(parent_ptr.clone());

        if self.equal(Some(node.clone()), left.clone()) {
            self.rotate(Side::Left, parent_ptr.clone());
        } else {
            self.rotate(Side::Right, parent_ptr.clone());
        }
    }

    fn delete_fix_case_5(&mut self, node: Rc<RefCell<RedBlackTreeNode<T>>>, sibling: Rc<RefCell<RedBlackTreeNode<T>>>) {        
        let mut sibling_ptr = sibling.clone();
        let parent_ptr = self.get_parent(node.clone()).unwrap();
        let is_left_child = self.equal(Some(node.clone()), self.left(parent_ptr.clone()));

        if is_left_child && !self.is_red(self.right(sibling_ptr.clone())) {

            let sibling_left = self.left(sibling_ptr.clone()).unwrap();
            self.set_color(sibling_left.clone(), NodeColor::Black);
            self.set_color(sibling_ptr.clone(), NodeColor::Red);
            self.rotate(Side::Right, sibling_ptr.clone());
            sibling_ptr = self.right(parent_ptr.clone()).unwrap();

        } else if !is_left_child && !self.is_red(self.left(sibling_ptr.clone())) {

            let sibling_right = self.right(sibling_ptr.clone()).unwrap();
            self.set_color(sibling_right.clone(), NodeColor::Black);
            self.set_color(sibling_ptr.clone(), NodeColor::Red);
            self.rotate(Side::Left, sibling_ptr.clone());
            sibling_ptr = self.left(parent_ptr.clone()).unwrap();
        }

        self.set_color(sibling_ptr.clone(), self.get_color(parent_ptr.clone()));
        self.set_color(parent_ptr.clone(), NodeColor::Black);

        if is_left_child {

            let sibling_right = self.right(sibling_ptr.clone()).unwrap();
            self.set_color(sibling_right.clone(), NodeColor::Black);
            self.rotate(Side::Left, parent_ptr.clone());

        } else {

            let sibling_left = self.right(sibling_ptr.clone()).unwrap();
            self.set_color(sibling_left.clone(), NodeColor::Black);
            self.rotate(Side::Right, parent_ptr.clone());
        }
    }

    fn replace_parent_child(&mut self, parent: MaybeRedBlackTree<T>, node: Rc<RefCell<RedBlackTreeNode<T>>>, child: MaybeRedBlackTree<T>) {
        match parent.clone() {
            None => {
                self.set_root(child.clone());
            },
            Some(p) => {
                let node_side = self.get_is_child(node.clone()).unwrap();
                self.set_child(p.clone(), node_side, child.clone());
                if child.is_some() {
                    self.set_parent(child.unwrap().clone(), Some(node_side), parent.clone());
                }
            }
        }
    }

    fn recolor_ins(&self, node: Rc<RefCell<RedBlackTreeNode<T>>>) {
        let n = node.as_ref().borrow();
        if n.get_grandparent().is_some() && n.get_uncle().is_some() {
            n.get_grandparent().unwrap().as_ref().borrow_mut().set_color(NodeColor::Red);
            n.get_parent().unwrap().as_ref().borrow_mut().set_color(NodeColor::Black);
            n.get_uncle().unwrap().as_ref().borrow_mut().set_color(NodeColor::Black);
        }
    }

    fn rotate_ins(&mut self, node: Rc<RefCell<RedBlackTreeNode<T>>>) {
        let mut parent = self.get_parent(node.clone()).unwrap();
        let grandparent = self.get_grandparent(node.clone()).unwrap();

        if self.get_is_child(parent.clone()).unwrap() == Side::Left {
            if self.get_is_child(node.clone()).unwrap() == Side::Right {
                self.rotate(Side::Left, parent.clone());
                parent = node.clone();
            } 

            self.rotate(Side::Right, grandparent.clone());

            self.set_color(parent.clone(), NodeColor::Black);
            self.set_color(grandparent.clone(), NodeColor::Red);

        } else {
            if self.get_is_child(node.clone()).unwrap() == Side::Left {
                self.rotate(Side::Right, parent.clone());
                parent = node.clone();
            } 

            self.rotate(Side::Left, grandparent.clone());

            self.set_color(parent.clone(), NodeColor::Black);
            self.set_color(grandparent.clone(), NodeColor::Red);
        }
    }

}