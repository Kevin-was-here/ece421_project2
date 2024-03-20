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
    // tree.delete(4);

    // tree.print_inorder();
    // tree.print_structure();

    // let mut tree2: RedBlackTree<char> = RedBlackTree::new();
    // tree2.insert('a');
    // tree2.insert('d');
    // tree2.insert('c');
    // tree2.insert('v');
    // tree2.print_inorder();
    // tree2.print_structure();

//    let mut avl:AvlTree<i32> = AvlTree::new();
//     avl.insert(5);
//     println!("Done 5");
//     avl.insert(3);
//     println!("Done 3");
//     avl.insert(7);
//     println!("Done 7");
//     avl.insert(8);
//     println!("Done 8");
//     avl.insert(9);
//     println!("Done 9");

//     let mut avl2:AvlTree<i32> = AvlTree::new();
//     avl2.insert(6);
//     println!("Done 6");
//     avl2.insert(7);
//     println!("Done 7");
//     avl2.insert(1);
//     println!("Done 1");
//     avl2.insert(2);
//     println!("Done 2");
//     avl2.insert(3);
//     println!("Done 3");

    // let mut avl3:AvlTree<i32> = AvlTree::new();
    // avl3.insert(10);
    // println!("Done 10");
    // avl3.insert(1);
    // println!("Done 1");
    // avl3.insert(15);
    // println!("Done 15");
    // avl3.insert(14);
    // println!("Done 14");
    // avl3.insert(13);
    // println!("Done 13");

//     let mut avl4:AvlTree<i32> = AvlTree::new();
//     avl4.insert(10);
//     println!("Done 10");
//     avl4.insert(15);
//     println!("Done 15");
//     avl4.insert(5);
//     println!("Done 5");
//     avl4.insert(4);
//     println!("Done 4");
//     avl4.insert(3);
//     println!("Done 3");

    let mut avl5:AvlTree<i32> = AvlTree::new();
    avl5.insert(1);
    println!("Done 1");
    avl5.insert(2);
    println!("Done 2");
    avl5.insert(3);
    println!("Done 3");
    avl5.insert(4);
    println!("Done 4");
    avl5.insert(5);
    println!("Done 5");
    avl5.insert(6);
    println!("Done 6");
    avl5.insert(7);
    println!("Done 7");
    avl5.insert(8);
    println!("Done 8");
    avl5.insert(9);
    println!("Done 9");
    avl5.insert(10);
    println!("Done 10");
}
