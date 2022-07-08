mod cons_list;

use crate::cons_list::List;

fn main() {
    let _list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
}
