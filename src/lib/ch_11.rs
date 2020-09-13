// not public... but still testable!
fn adder(i: i32, n: i32) -> i32 {
    i + n
}

pub fn greeter(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod adder {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn adder_adds() {
        assert_eq!(5, adder(2, 3));
    }

    #[test]
    #[ignore]
    fn two_plus_two() {
        assert_eq!(4, adder(2, 2));
    }

    #[test]
    fn test_returns_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Two plus two does not equal four"))
        }
    }

    #[test]
    fn greeter_greets() {
        let greeting = greeter("Zakk");

        assert!(greeting.contains("Zakk"));
    }

    #[test]
    fn flakey_test() -> Result<(), String> {
        let mut rng = rand::thread_rng();
        let rand: f64 = rng.gen();

        if rand > 0.5 {
            Ok(())
        } else {
            Err("Tails!".to_string())
        }
    }
}
