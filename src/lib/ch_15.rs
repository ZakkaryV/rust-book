use std::fmt::Display;

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

pub enum List {
    // A recursive type can't work like this because `List`s size
    // cannot be known at compiletime. `Box` however, is merely a pointer
    // to a value on the heap.
    // Cons(i32, List),
    Cons(i32, Box<List>),
    Nil,
}

#[cfg(test)]
mod smart_pointers {

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
}
