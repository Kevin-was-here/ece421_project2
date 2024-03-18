mod rb_tree;
mod avl_tree;
mod tree;
use crate::rb_tree::*;
use crate::avl_tree::*;

fn main() {
    let mut tree = RedBlackTree::new();
    tree.insert(3);
    tree.insert(1);
    tree.insert(1);
    tree.insert(2);
    // tree.insert(5);
    // tree.insert(4);
    // tree.insert(6);
    println!("{}", tree.size);
//     3
//    /  \
//   1    5
//    \   / \
//     2 4   6


    let mut tree2 = AvlTree::new();
}
