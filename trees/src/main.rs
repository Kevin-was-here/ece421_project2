mod rb_tree;
mod avl_tree;
mod tree;
use crate::rb_tree::*;
use crate::tree::*;
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

    tree.print_inorder();
    tree.print_structure();

    let mut tree2: RedBlackTree<char> = RedBlackTree::new();
    tree2.insert('a');
    tree2.insert('d');
    tree2.insert('c');
    tree2.insert('v');
    tree2.print_inorder();
    tree2.print_structure();

    let mut avl:AvlTree<i32> = AvlTree::new();
}
