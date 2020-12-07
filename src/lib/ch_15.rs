use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug)]
pub struct MyBox<T: Display>(T);

impl<T: Display> MyBox<T> {
    pub fn new(enclosed_val: T) -> MyBox<T> {
        MyBox(enclosed_val)
    }
}

impl<T: Display> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: Display> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("{} is about to drop out of scope", self.0);
    }
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

// a MOCK OBJECT: for testing, when you'd like to keep track of your objects usage
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max_value: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, value: usize, max_value: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value,
            max_value,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let perc_of_max = self.value as f64 / self.max_value as f64;

        if perc_of_max >= 1.0 {
            self.messenger.send("You are now over your quota!")
        } else if perc_of_max >= 0.9 {
            self.messenger.send("You are at 90% of your quota limit.")
        } else if perc_of_max >= 0.75 {
            self.messenger.send("You are at 75% of your quota limit.")
        }

        // I guess f64 doesn't implement Cmp :(
        // match perc_of_max.cmp(1.0) {
        //     Ordering::Greater => println!("You are now over your quota!"),
        //     Ordering::Equal => println!("You've reached your quota"),
        //     Ordering::Less => println!("You're under your quota"),
        // }
    }

    pub fn send(&self, msg: &str) {
        println!("MESSAGE RECEIVED: {}", msg);
    }
}

#[derive(Debug)]
pub enum List {
    // A recursive type can't work like this because `List`s size
    // cannot be known at compiletime. `Box` however, is merely a pointer
    // to a value on the heap.
    // Cons(i32, List),
    //
    // We'll opt to use Rc over Box because it's implementation as a Smart
    // Pointer provides the ability to have multiple references at the same
    // time, where Box behaves like a "regular" pointer
    // Cons(i32, Box<List>),
    //
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    pub fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            List::Cons(_, val) => Some(val),
            List::Nil => None,
        }
    }
}

// impl std::fmt::Display for List {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{}", &self.)
//     }
// }

pub mod tree {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    #[derive(Debug)]
    pub struct Node {
        pub val: i32,
        pub children: RefCell<Vec<Rc<Node>>>,
        pub parents: RefCell<Weak<Node>>,
    }
}

#[cfg(test)]
mod smart_pointers {
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn deref_is_sometimes_necessary() {
        let x = 5;
        // y cannot be compared to x without DEREFing it as they are DIFFERENT TYPES:
        // i32 vs &i32
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn deref_works_with_box() {
        let x = 5;
        // y is still a pointer, but not to x (on the stack) rather to a copied version of x on the
        // HEAP. The magic of Box allows us to still use the deref operator as expected
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn deref_works_with_custom_smart_pointer() {
        let x = 5;
        // y is still a pointer, but not to x (on the stack) rather to a copied version of x on the
        // HEAP. The magic of Box allows us to still use the deref operator as expected
        let y = super::MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(msg.to_owned())
        }
    }

    #[test]
    fn mock_obj_tracker_reports_a_value_over_threshold() {
        let mock_messenger = MockMessenger::new();

        let mut tracker = LimitTracker::new(&mock_messenger, 0, 100);

        tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow_mut().len(), 1);
    }
}
