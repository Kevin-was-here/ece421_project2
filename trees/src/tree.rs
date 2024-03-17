use std::cell::RefCell;
use std::rc::Rc;

pub trait Traversible {
    fn left_mut(&mut self) -> &mut Option<Rc<RefCell<Self>>>;
    fn right_mut(&mut self) -> &mut Option<Rc<RefCell<Self>>>;
    fn left(&self) -> &Option<Rc<RefCell<Self>>>;
    fn right(&self) -> &Option<Rc<RefCell<Self>>>;
}

fn bst_insert<R: Traversible, T>(tree: R, k: T) {
    // ...
}