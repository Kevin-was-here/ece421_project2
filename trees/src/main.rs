// mod avl_tree;
mod rb_tree;
mod tree;
use crate::rb_tree::*;

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

    tree.print_inorder();
    tree.print_structure();

    let mut tree2: RedBlackTree<char> = RedBlackTree::new();
    tree2.insert('a');
    tree2.insert('d');
    tree2.insert('c');
    tree2.insert('v');
    tree2.print_inorder();
    tree2.print_structure();
//     3
//    /  \
//   1    5
//    \   / \
//     2 4   6
}
