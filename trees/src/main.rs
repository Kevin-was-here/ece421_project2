mod rb_tree;
mod avl_tree;
mod tree;
use crate::rb_tree::*;
use crate::avl_tree::*;

fn main() {
    let mut tree = RedBlackTree::new();
    tree.insert(3);
    println!("Done");
    tree.insert(1);
    println!("Done");
    tree.insert(2);
    println!("Done");
    tree.insert(5);
    println!("Done");
    tree.insert(4);
    println!("Done");
    tree.insert(6);
    println!("Done");

//     3
//    /  \
//   1    5
//    \   / \
//     2 4   6


    //let mut tree2 = AvlTree::new();
}
