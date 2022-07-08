mod cons_list;
mod my_box;

use crate::cons_list::List;
use crate::my_box::MyBox;

fn main() {
    let _list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let name = MyBox::new(String::from("Rust"));
    hello(&name);
}

fn hello(name: &str) {
    println!("hello, {}", name);
}
