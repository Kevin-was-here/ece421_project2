// for tree_size in (10,000, 40,000, 70,000, 100,000, 130,000) do:
// Start by creating an empty tree.
// Values with tree_size are inserted into the tree.
// A search is conducted for the (tree_size/10) lowest values.
// end

use lib::{rb_tree::RedBlackTree, tree::Tree, avl_tree::AvlTree, node::Node};

use std::iter;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{thread_rng, Rng};

//---------- RB Tree ---------------
fn bench_rb_tree_insert(c: &mut Criterion){

    let mut group = c.benchmark_group("bench_rb_tree");

    let mut rng = rand::thread_rng();
    //create an empty tree
    let mut rb_tree = RedBlackTree::new();

    for tree_size in [10000, 40000, 70000 , 100000, 130000].iter() {
        group.bench_with_input(
            criterion::BenchmarkId::new("RB_tree_insert", tree_size),
            tree_size,
            |b, &tree_size| {
                b.iter(|| {
                    //insert values into the tree
                    rb_tree.insert(tree_size);
                })
            },
        );

    }
    group.finish();
}

fn bench_rb_tree_search(c: &mut Criterion){

    let mut group = c.benchmark_group("bench_rb_tree");

    //create an empty tree
    let mut rb_tree = RedBlackTree::new();

    //for 10k elements
    for tree_size in [10000, 40000, 70000 , 100000, 130000].iter() {
        //insert the values into the tree without benching
        for i in 0..=*tree_size {
            rb_tree.insert(i);
        }

        //bench the search to the tree_size/10 lowest values
        group.bench_with_input(
            criterion::BenchmarkId::new("RB_tree_search", tree_size),
            tree_size,
            |b, &tree_size| {
                b.iter(|| {
                    //search for the lowest values
                    for i in 0..tree_size/10 {
                        rb_tree.bst_search(i);
                    }
                })
            },
        );

    }
    group.finish();
}

//---------- AVL Tree ---------------
fn bench_avl_tree_insert(c: &mut Criterion){

    let mut group = c.benchmark_group("bench_avl_tree");

    let mut rng = rand::thread_rng();
    //create an empty tree
    let mut avl_tree = AvlTree::new();

    for tree_size in [10000, 40000, 70000 , 100000, 130000].iter() {
        group.bench_with_input(
            criterion::BenchmarkId::new("AVL_tree_insert", tree_size),
            tree_size,
            |b, &tree_size| {
                b.iter(|| {
                    //insert values into the tree
                    avl_tree.insert(tree_size);

                })
            },
        );

    }
    group.finish();
}

fn bench_avl_tree_search(c: &mut Criterion){
    
        let mut group = c.benchmark_group("bench_avl_tree");
    
        //create an empty tree
        let mut avl_tree = AvlTree::new();

        for tree_size in [10000, 40000, 70000 , 100000, 130000].iter() {
            //insert the values into the tree without benching
            for i in 0..*tree_size {
                avl_tree.insert(i);
            }

            //bench the search to the tree_size/10 lowest values
            group.bench_with_input(
                criterion::BenchmarkId::new("AVL_tree_search", tree_size),
                tree_size,
                |b, &tree_size| {
                    b.iter(|| {
                        //search for the lowest values
                        for i in 0..tree_size/10 {
                            avl_tree.bst_search(i);
                        }
                    })
                },
            );
    
        }
        group.finish();
}

criterion_group!(benches, bench_rb_tree_insert, bench_rb_tree_search, bench_avl_tree_insert, bench_avl_tree_search);
criterion_main!(benches);