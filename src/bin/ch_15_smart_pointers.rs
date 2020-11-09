use lib::List::{Cons, Nil};
use lib::MyBox;

fn main() {
    // the BOX is stored on the stack, but the DATA is stored on the heap
    // as expected, the Box will be deallocated from the heap AND stack at the end of the scope
    let _b = Box::new(5);
    // storing an i32 on the heap is not useful: it's already stored there anyways
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    {
        // custom Drop behavior is implemented here
        let _custom_box1 = MyBox::new("abc");
        let _custom_box2 = MyBox::new(123);

        // we can opt to drop a value early
        drop(_custom_box2);
    }

    println!("This program does nothing");
}
