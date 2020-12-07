use lib::List::{Cons, Nil};
use lib::MyBox;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // the BOX is stored on the stack, but the DATA is stored on the heap
    // as expected, the Box will be deallocated from the heap AND stack at the end of the scope
    let _b = Box::new(5);

    let one = Rc::new(RefCell::new(1));

    *one.borrow_mut() += 1;
    println!("ONE: {:#?}", one);

    // storing an i32 on the heap is not useful: it's already stored there anyways
    let list_1 = Rc::new(Cons(
        one,
        Rc::new(Cons(
            Rc::new(RefCell::new(2)),
            Rc::new(Cons(Rc::new(RefCell::new(3)), Rc::new(Nil))),
        )),
    ));

    {
        // custom Drop behavior is implemented here
        let _custom_box1 = MyBox::new("abc");
        let _custom_box2 = MyBox::new(123);

        // we can opt to drop a value early
        drop(_custom_box2);
    }

    // Cons variants own their data, which could be problematic in a scneario where
    // we want multiple pointers to the same list value, like in a graph
    //

    println!("Reference count of List 1: {}", Rc::strong_count(&list_1)); // 1

    let _list_2 = Cons(Rc::new(RefCell::new(5)), Rc::clone(&list_1));

    {
        // when this reference counter is dropped it will decrement the total count
        let _list_3 = Rc::clone(&list_1);
        println!("Reference count of List 1: {}", Rc::strong_count(&list_1));
    }

    println!("Reference count of List 1: {}", Rc::strong_count(&list_1));
}
