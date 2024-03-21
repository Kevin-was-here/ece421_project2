mod rb_tree;
mod avl_tree;
mod tree;
mod node;
mod cli;
use std::io::stdin;

use crate::rb_tree::*;
use crate::tree::*;
use crate::avl_tree::*;
use crate::cli::*;

fn test_rb() {
    let mut tree = RedBlackTree::new();
    tree.insert(2);
    tree.insert(4);
    tree.insert(1);
    tree.insert(5);
    tree.insert(6);
    tree.insert(10);
    tree.insert(8);
    tree.insert(3);
    tree.insert(0);
    tree.insert(9);
    tree.insert(12);
    tree.insert(7);

    // tree.delete(6);
    tree.delete(8);
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
   
}

fn test_avl(){

    let mut avl:AvlTree<i32> = AvlTree::new();
    avl.insert(2);
    avl.insert(4);
    avl.insert(1);
    avl.insert(5);
    avl.insert(6);
    //println!("AVL");

    let mut avl:AvlTree<i32> = AvlTree::new();
    avl.insert(1);
    avl.insert(2);
    avl.insert(3);
    avl.insert(4);
    avl.insert(5); 
    avl.insert(6);
    avl.insert(7);
    avl.insert(8);
    avl.insert(9);
    avl.insert(10);
    avl.insert(11);
    avl.insert(12);
    //println!("AVL");
}

fn test_search(){

    let mut avl:AvlTree<i32> = AvlTree::new();
    avl.insert(1);
    avl.insert(2);
    avl.insert(3);
    avl.insert(4);
    avl.insert(5);
    avl.insert(6);
    avl.insert(7);
    avl.insert(8);
    avl.insert(9);
    avl.insert(10);
    avl.insert(11);
    avl.insert(12);

    println!( "search is {}", avl.bst_search(12));
    println!( "search is {}", avl.bst_search(13));
}

fn main() {
    //test_avl();
    //test_rb();
    //test_search();
    cli::run_cli();
}
