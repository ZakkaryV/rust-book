use lib::tree::Node;
use lib::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

//fn _recursive_types() {
//    // the BOX is stored on the stack, but the DATA is stored on the heap
//    // as expected, the Box will be deallocated from the heap AND stack at the end of the scope
//    let _b = Box::new(5);

//    let one = Rc::new(RefCell::new(1));

//    *one.borrow_mut() += 1;
//    println!("ONE: {:#?}", one);

//    // storing an i32 on the heap is not useful: it's already stored there anyways
//    let list_1 = Rc::new(Cons(
//            one,
//            Rc::new(Cons(
//                    Rc::new(RefCell::new(2)),
//                    Rc::new(Cons(Rc::new(RefCell::new(3)), Rc::new(Nil))),
//            )),
//    ));

//    {
//        // custom Drop behavior is implemented here
//        let _custom_box1 = MyBox::new("abc");
//        let _custom_box2 = MyBox::new(123);

//        // we can opt to drop a value early
//        drop(_custom_box2);
//    }

//    // Cons variants own their data, which could be problematic in a scneario where
//    // we want multiple pointers to the same list value, like in a graph
//    //

//    println!("Reference count of List 1: {}", Rc::strong_count(&list_1)); // 1

//    let _list_2 = Cons(Rc::new(RefCell::new(5)), Rc::clone(&list_1));

//    {
//        // when this reference counter is dropped it will decrement the total count
//        let _list_3 = Rc::clone(&list_1);
//        println!("Reference count of List 1: {}", Rc::strong_count(&list_1));
//    }

//    println!("Reference count of List 1: {}", Rc::strong_count(&list_1));
//}

fn _reference_cycle() {
    // Reference Cycles

    // list #1
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial count: {}", Rc::strong_count(&a));
    println!("a next item {:#?}", a.tail());

    // list #2: pointing to list #1
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a count after b creation: {}", Rc::strong_count(&a));
    println!("b initial count: {}", Rc::strong_count(&b));
    println!("b next item {:#?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b ref count after changing: {}", Rc::strong_count(&b));
    println!("a ref count after changing: {}", Rc::strong_count(&a));

    println!("a next item = {:?}", a.tail());
}

fn tree_data_structure() {
    let leaf = Rc::new(Node {
        val: 5,
        children: RefCell::new(vec![]),
        parents: RefCell::new(Weak::new()),
    });

    println!("leaf parent: {:#?}", leaf.parents.borrow().upgrade());

    println!(
        "leaf strong_count: {}, weak_count: {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node {
            val: 22,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
            parents: RefCell::new(Weak::new()),
        });

        *leaf.parents.borrow_mut() = Rc::downgrade(&branch);

        println!("branch parent: {:#?}", branch.parents.borrow().upgrade());

        println!(
            "branch strong_count: {}, weak_count: {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );

        println!("leaf parent: {:#?}", leaf.parents.borrow().upgrade());
    }

    // branch has gone out of scope and been dropped; Weak refs return an Option
    // to prevent the type of panic that would occur if trying to access
    // a strong reference to a Rc that has been dropped
    println!("leaf parent: {:#?}", leaf.parents.borrow().upgrade());
    // leaf.parents.borrow_mut().push(Rc::downgrade(&branch));

    // println!("{:#?}", branch);
}

fn main() {
    // _recursive_types();

    // _reference_cycle();

    tree_data_structure();
}
