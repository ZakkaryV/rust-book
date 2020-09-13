pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Too low! Expected a value of at least 1, got {}", value);
        } else if value > 100 {
            panic!(
                "Too high! Expected a value no higher than 100, got {}",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Too high! Expected a value no higher than 100, got 666")]
    fn guess_panics_if_outside_range() {
        Guess::new(666);
    }
}
