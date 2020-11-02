use add_one::add_one;
use add_two::add_two;
use rand::prelude::*;

fn main() {
    let x = rand::thread_rng().gen::<i32>();

    println!("{} + 1 is {}", x, add_one(x));

    println!("{} + 2 is {}", x, add_two(x));
}
