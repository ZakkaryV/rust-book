use std::cmp::Ordering;
use std::io;

#[derive(Debug)]
pub struct Rectangle {
    pub height: u32,
    pub width: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.height * self.width
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    pub fn is_valid_rectangle(&self) -> bool {
        match self.height.cmp(&self.width) {
            Ordering::Less => true,
            Ordering::Equal => {
                println!("\nEnter the dimensions for a rectangle, not a square!");
                std::process::exit(1)
            }
            Ordering::Greater => true,
        }
    }
}

fn main() {
    let dimension_specifiers = (String::from("height"), String::from("width"));

    println!("\nEnter the dimensions for rectangle 1: ");
    let rectangle1: Rectangle = Rectangle {
        height: get_input_number(&dimension_specifiers.0),
        width: get_input_number(&dimension_specifiers.1),
    };
    rectangle1.is_valid_rectangle();
    println!("\n{:?}\nArea: {}", rectangle1, rectangle1.area());

    println!("\nEnter the dimensions for rectangle 2: ");
    let rectangle2: Rectangle = Rectangle {
        height: get_input_number(&dimension_specifiers.0),
        width: get_input_number(&dimension_specifiers.1),
    };
    rectangle2.is_valid_rectangle();
    println!("\n{:?}\nArea: {}", rectangle2, rectangle2.area());

    let will_or_wont = if rectangle1.can_hold(&rectangle2) {
        String::from("will")
    } else {
        String::from("won't")
    };

    println!(
        "\nThe second one {} fit inside of the first one.",
        will_or_wont
    );
}

fn get_input_number(label: &String) -> u32 {
    println!("{}: ", label);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("You were supposed to pass in a number!");

    match input.trim().parse() {
        Ok(i) => i,
        Err(..) => 0,
    }
}
