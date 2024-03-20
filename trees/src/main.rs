mod rb_tree;
mod avl_tree;
mod tree;
mod node;
use crate::rb_tree::*;
use crate::tree::*;
use crate::avl_tree::*;

fn main() {
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
    // tree.delete(8);
    tree.print_inorder();
    tree.print_structure();

    // let mut tree2: RedBlackTree<char> = RedBlackTree::new();
    // tree2.insert('a');
    // tree2.insert('d');
    // tree2.insert('c');
    // tree2.insert('v');
    // tree2.delete('c');
    // tree2.print_inorder();
    // tree2.print_structure();

//    let mut avl:AvlTree<i32> = AvlTree::new();
//     avl.insert(3);
//     println!("Done 3");
//     //avl.insert(1);
//     //println!("Done 1");
//     //avl.insert(2);
//     //println!("Done 2");
//     avl.insert(5);
//     println!("Done 5");
//     //avl.insert(4);
//     //println!("Done 4");
//     avl.insert(6);
//     println!("Done 6");
}
