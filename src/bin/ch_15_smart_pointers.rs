use lib::List::{Cons, Nil};

fn main() {
    // the BOX is stored on the stack, but the DATA is stored on the heap
    // as expected, the Box will be deallocated from the heap AND stack at the end of the scope
    let b = Box::new(5);
    // storing an i32 on the heap is not useful: it's already stored there anyways
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("This program does nothing");
}
