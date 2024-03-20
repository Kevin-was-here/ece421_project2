use std::io::stdin;
use crate::rb_tree::*;
use crate::tree::*;
//use crate::avl_tree::*;

fn get_menu_choice(menu: &str, n: u32, ret: &mut u32) {
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

fn get_i32(ret: &mut i32) {
    loop {
        println!("Please enter a 32-bit integer (i32) value.");
        let mut s: String = "".to_string();

        match stdin().read_line(&mut s) {
            Err(_) => println!("Something went wrong reading input, please try again."),
            Ok(_) => {}
        };
    
        match s.trim().parse::<i32>() {
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

fn rb_loop() {
    let mut tree: RedBlackTree<i32> = RedBlackTree::new();

    println!("> A new Red-Black Tree has been created.");

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
                let mut node_val = 0;
                get_i32(&mut node_val);
                tree.insert(node_val);
                println!("> The node {node_val} was inserted.\n");
            }   
            2u32 => {
                // delete
                let mut node_val = 0;
                get_i32(&mut node_val);
                // tree.delete(node_val);
                println!("> This feature is not yet implemented.\n");
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

pub fn run_cli() {
    // choose a type of tree

    loop {
        let mut c = 0;
        get_menu_choice("Please select a type of tree by entering the corresponding integer:
1. Red-Black Tree
2. AVL Tree", 2, &mut c);

        if c == 1u32 {
            rb_loop();
        }
        else if c == 2u32 {
            println!("> This feature is not yet implemented.\n");
            // avl_loop();
        }
        else {
            println!("Something went wrong, please try again.\n")
        }
    }
}