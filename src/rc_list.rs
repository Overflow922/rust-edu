use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub enum RcList {
    Cons(Rc<RefCell<i32>>, Rc<RcList>),
    Nil,
}