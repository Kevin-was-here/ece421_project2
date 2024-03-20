mod rb_tree;
mod avl_tree;
mod tree;
mod node;
use crate::rb_tree::*;
use crate::tree::*;
use crate::avl_tree::*;

fn main() {
    // let mut tree = RedBlackTree::new();
    // tree.insert(3);
    // println!("Done");
    // tree.insert(1);
    // println!("Done");
    // tree.insert(2);
    // println!("Done");
    // tree.insert(5);
    // println!("Done");
    // tree.insert(4);
    // println!("Done");
    // tree.insert(6);
    // println!("Done");

    // tree.print_inorder();
    // tree.print_structure();

    // let mut tree2: RedBlackTree<char> = RedBlackTree::new();
    // tree2.insert('a');
    // tree2.insert('d');
    // tree2.insert('c');
    // tree2.insert('v');
    // tree2.print_inorder();
    // tree2.print_structure();

    let mut avl:AvlTree<i32> = AvlTree::new();
    avl.insert(3);
    println!("3 Done");
    avl.insert(1);
    println!("1 Done");
    avl.insert(2);
    println!("2 Done");
    avl.insert(5);
    println!("5 Done");
    avl.insert(4);
    println!("4 Done");
    avl.insert(6);
    println!("6 Done");
}
