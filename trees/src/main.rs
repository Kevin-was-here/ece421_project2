mod rb_tree;
//mod avl_tree;
mod tree;
mod node;
mod cli;
use std::io::stdin;

use crate::rb_tree::*;
use crate::tree::*;
//use crate::avl_tree::*;
use crate::cli::*;

fn test() {
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
    println!("num leaves: {}", tree.count_leaves());

    let mut tree2: RedBlackTree<char> = RedBlackTree::new();
    tree2.insert('a');
    tree2.insert('d');
    tree2.insert('c');
    tree2.insert('v');
    tree2.print_inorder();
    tree2.print_structure();
    println!("num leaves: {}", tree2.count_leaves());    
   // let mut avl:AvlTree<i32> = AvlTree::new();
}

fn main() {
    cli::run_cli();
}