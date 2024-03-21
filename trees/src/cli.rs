use std::io::stdin;
use crate::rb_tree::*;
use crate::tree::*;
use crate::avl_tree::*;

// type names for some basic node types
pub trait CLIPrintable {
    fn pretty_name() -> &'static str;
}

impl CLIPrintable for i8 {
    fn pretty_name() -> &'static str {
        "8-bit integer"
    }
}

impl CLIPrintable for i32 {
    fn pretty_name() -> &'static str {
        "32-bit integer"
    }
}

impl CLIPrintable for char {
    fn pretty_name() -> &'static str {
        "character"
    }
}


fn get_menu_choice(menu: &str, n: u32, ret: &mut u32) {
    // obtaining an integer selection from CLI menu
    loop {
        println!("{}\n", menu);
        let mut s: String = "".to_string();

        match stdin().read_line(&mut s) {
            Err(_) => println!("Something went wrong reading input, please try again."),
            Ok(_) => {}
        };
    
        match s.trim().parse::<u32>() {
            Err(_) => println!("Please enter a valid integer."),
            Ok(p) => {
                if 1 <= p && p <= n {
                    *ret = p;
                    println!("");
                    break;
                }
            }
        }    
    }
}

fn get_gen_type<T: std::str::FromStr + CLIPrintable>(ret: &mut T) {
    // read and format a generic type from the console
    loop {
        let name = T::pretty_name();
        println!("Please enter a node value of type {name}.");
        let mut s: String = "".to_string();

        match stdin().read_line(&mut s) {
            Err(_) => println!("Something went wrong reading input, please try again."),
            Ok(_) => {}
        };
    
        match s.trim().parse::<T>() {
            Err(_) => println!("That input isn't valid, please try again."),
            Ok(p) => {
                *ret = p;
                println!("");
                break;
            }
        }
    }    
}

fn get_continue() {
    // force user to press enter so menu isn't printed immediately after output
    loop {
        println!("Please press 'enter' to continue...");
        let mut s: String = "".to_string();

        match stdin().read_line(&mut s) {
            Err(_) => println!("Something went wrong reading input, please try again."),
            Ok(_) => {
                println!("");
                break;
            }
        };
    }
}

fn tree_loop<T, R>() where
    T: std::cmp::Ord + Copy + std::fmt::Debug + std::fmt::Display
        + std::default::Default + std::str::FromStr + CLIPrintable,
    R: Tree<T> + CLIPrintable
{
    let mut tree: R = R::new();

    println!("> A new {} with {} keys has been created.", R::pretty_name(), T::pretty_name());

    loop {
        let mut c = 0;
        get_continue();
        get_menu_choice("Please select an operation by entering the corresponding integer:
1. Insert Node
2. Delete Node
3. Count Leaves
4. Get Height
5. Print In-Order Traversal
6. Check if Empty
7. Print Tree Structure
8. Go Back (deletes tree)", 8, &mut c);

        match c {
            1u32 => {
                // insert
                let mut node_val: T = T::default();
                get_gen_type(&mut node_val);
                tree.insert(node_val);
                println!("> The node {node_val} was inserted, if it did not already exist in the tree.\n");
            }   
            2u32 => {
                // delete
                let mut node_val: T = T::default();
                get_gen_type(&mut node_val);
                tree.delete(node_val);
                println!("> The node {node_val} was deleted, if it existed in the tree.\n");
            }
            3u32 => {
                // count leaves
                let count = tree.count_leaves();
                println!("> Leaf count: {count}\n");
            }
            4u32 => {
                // hright
                let height = tree.get_height();
                println!("> Height: {height}\n\n");
            }
            5u32 => {
                tree.print_inorder();
            }
            6u32 => {
                if tree.is_empty() {
                    println!("> Tree is empty.\n");
                } else {
                    println!("> Tree is not empty.\n");
                }
            }
            7u32 => {
                tree.print_structure();
            }
            8u32 => {
                println!("> Returning to main menu (and deleting this tree)...\n");
                return;
            }
            _ => {
                println!("Something went wrong, please try again.\n");
            }
        };
    }
}

#[allow(dead_code)]
pub fn run_cli() {
    // choose a type of tree

    loop {
        let mut c = 0;
        get_menu_choice("Please select a type of tree by entering the corresponding integer:
1. Red-Black Tree
2. AVL Tree
3. (Exit Program)", 3, &mut c);

        if c == 3u32 {
            break;
        }

        let mut t = 0;
        get_menu_choice("Please select a type of node key to use with the tree:
1. i8 (8-bit integer)
2. i32 (32-bit integer)
3. char (character)", 3, &mut t);

        // enter loop with a tree of user's choice
        match c {
            1u32 => {
                match t {
                    1u32 => tree_loop::<i8, RedBlackTree<i8>>(),
                    2u32 => tree_loop::<i32, RedBlackTree<i32>>(),
                    3u32 => tree_loop::<char, RedBlackTree<char>>(),
                    _ => println!("Something went wrong, please try again.\n")
                }
            },
            2u32 => {
                match t {
                    1u32 => tree_loop::<i8, AvlTree<i8>>(),
                    2u32 => tree_loop::<i32, AvlTree<i32>>(),
                    3u32 => tree_loop::<char, AvlTree<char>>(),
                    _ => println!("Something went wrong, please try again.\n")
                }
            },
            _ => println!("Something went wrong, please try again.\n")
        }
    }
}