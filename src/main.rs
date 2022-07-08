mod cons_list;
mod my_box;
mod ref_cell;
mod rc_list;

use std::cell::RefCell;
use std::rc::Rc;
use crate::cons_list::List;
use crate::my_box::MyBox;
use crate::rc_list::RcList;
use crate::RcList::Nil;

fn main() {
    let _list = List::Cons(1, Rc::new(List::Cons(2, Rc::new(List::Nil))));

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let name = MyBox::new(String::from("Rust"));
    hello(&name);
    ref_counter();
    rc_ref_cell();
}

fn rc_ref_cell() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(RcList::Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = RcList::Cons(Rc::new(RefCell::new(2)), Rc::clone(&a));
    let c = RcList::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    *value.borrow_mut() += 10;

    println!("a after add {:?}", a);
    println!("b after add {:?}", b);
    println!("c after add {:?}", c);
}

fn ref_counter() {
    let a = Rc::new(List::Cons(3, Rc::new(List::Cons(4, Rc::new(List::Nil)))));
    println!("ref count for &a {}", Rc::strong_count(&a));
    let _b = List::Cons(2, Rc::clone(&a));
    println!("ref count for &a {}", Rc::strong_count(&a));
    {
        let _c = List::Cons(1, Rc::clone(&a));
        println!("ref count for &a {}", Rc::strong_count(&a));
    }
    println!("ref count for &a {}", Rc::strong_count(&a));
}

fn hello(name: &str) {
    println!("hello, {}", name);
}
