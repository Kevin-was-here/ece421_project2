use std::cell::RefCell;
use std::rc::Rc;
mod tree;

type AvlTree<T> = Option<Rc<RefCell<AvlTreeNode<T>>>>;

#[derive(Debug, PartialEq, Clone)]
struct AvlTreeNode<T: Ord> {
    key: T,
    left: AvlTreeNode<T>,
    right: AvlTreeNode<T>,
    height: u32,
}

impl<T: Ord> AvlTreeNode<T> {
    fn new() -> Self {
        Self { root: None }
    }

    fn insert(&self, key: T) {

        //insertion happens

        //get balance factor
        //if bf > 1 and key < root.left.key

        //return self.right_rotate();

        //if bf < -1 and key > root.right.key

        //return self.left_rotate();

        //if bf > 1 and key > root.left.key

        //root.left = root.left.left_rotate();

        //return self.right_rotate();

        //if bf < -1 and key < root.right.key

        //root.right = root.right.right_rotate();

        //return self.left_rotate();

    }

    fn left_rotate(&self) {

        //let &mut cur_right = self.right;

        //let &mut cur_right_left_child = cur_right.left;

        //cur_right.left = self;  
        //self.right = cur_right_left_child;

        //self.height = 1 + max(self.left.height, self.right.height);
        //cur_right.height = 1 + max(cur_right.left.height, cur_right.right.height);

        //return cur_right;
    }

    fn right_rotate(&self) {

        //let &mut cur_left = self.left;

        //let &mut cur_left_right_child = cur_left.right;

        //cur_left.right = self;  
        //self.left = cur_left_right_child;

        //self.height = 1 + max(self.left.height, self.right.height);
        //cur_left.height = 1 + max(cur_left.left.height, cur_left.right.height);

        //return cur_left;
    }

    fn search(&self, key: T) -> AvlTreeNode<T> {
        
        //let mut current = self.root;

        //while current is not none and key != current.key{

        //  if key < current.key{

        //      current = current.left;
        
        //  } else {

        //      current = current.right;

        //  }

        //return current;
    }
}

//for any node, the height of its 2 subtrees should not differ by more than 1
//balance factor = height of left subtree - height of right subtree