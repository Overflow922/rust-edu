mod cons_list;
mod my_box;
mod ref_cell;

use std::rc::Rc;
use crate::cons_list::List;
use crate::my_box::MyBox;

fn main() {
    let _list = List::Cons(1, Rc::new(List::Cons(2, Rc::new(List::Nil))));

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let name = MyBox::new(String::from("Rust"));
    hello(&name);
    ref_counter();
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
