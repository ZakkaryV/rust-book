use std::cmp::Ordering;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let rect1 = Rectangle {
            height: 8,
            width: 7,
        };
        let rect2 = Rectangle {
            height: 5,
            width: 1,
        };

        assert!(rect1.can_hold(&rect2));
    }

    #[test]
    fn smaller_can_hold_larger() {
        let rect1 = Rectangle {
            height: 5,
            width: 1,
        };
        let rect2 = Rectangle {
            height: 8,
            width: 7,
        };

        assert!(!rect1.can_hold(&rect2));
    }
}
