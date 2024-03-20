use std::ops::Not;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::max;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Side {
    Left,
    Right,
}

impl Not for Side {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

pub trait Traversible<T> {
    // fn left_mut(&mut self) -> &mut Option<Rc<RefCell<Self>>>;
    // fn right_mut(&mut self) -> &mut Option<Rc<RefCell<Self>>>;
    fn left(&self) -> &Option<Rc<RefCell<Self>>>;
    fn right(&self) -> &Option<Rc<RefCell<Self>>>;
}
pub trait Node<T>: Traversible<T> {
    fn new(key: T) -> Self;
    fn get_key(&self) -> &T; 
    fn set_key(&mut self, val: T);

    fn greater(&self, val: T) -> bool;
    fn equal(&self, val: T) -> bool;
    fn less(&self, val: T) -> bool;

    fn get_child(&self, side: Side) -> Option<Rc<RefCell<Self>>>;
    fn get_is_child(&self) -> &Option<Side>;
    fn is_child(&self, side: Side) -> bool;
    fn take_child(&mut self, side: Side) -> Option<Rc<RefCell<Self>>>;
    fn set_child(&mut self, side: Side, child: Option<Rc<RefCell<Self>>>);

    fn set_parent(&mut self, is_child: Option<Side>, parent: Option<Rc<RefCell<Self>>>);
    fn get_parent(&self) -> Option<Rc<RefCell<Self>>>;
    fn get_parent_mut(&mut self) -> &mut Option<Rc<RefCell<Self>>>;

    fn get_sibling(&self) -> Option<Rc<RefCell<Self>>>;
    fn get_uncle(&self) -> Option<Rc<RefCell<Self>>>;
    fn get_grandparent(&self) -> Option<Rc<RefCell<Self>>>;
    
    fn is_leaf(&self) -> bool;

    fn set_height(&mut self, height: usize);
    fn get_height(&self) -> usize;
}