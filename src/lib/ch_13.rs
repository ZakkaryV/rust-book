pub mod custom_workout_builder {
    use std::thread;
    use std::time::Duration;

    // fn simulated_expensive_function(intensity: u32) -> u32 {
    //     println!("Calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     intensity
    // }

    pub struct Cacher<T: Fn(u32) -> u32> {
        calculation: T,
        value: std::collections::HashMap<u32, u32>,
    }

    impl<T: Fn(u32) -> u32> Cacher<T> {
        pub fn new(closure: T) -> Cacher<T> {
            Cacher {
                calculation: closure,
                value: std::collections::HashMap::new(),
            }
        }

        pub fn value(&mut self, arg: u32) -> u32 {
            match self.value.get(&arg) {
                Some(v) => *v,
                None => {
                    let result = (self.calculation)(arg);
                    &self.value.insert(arg, result);
                    result
                }
            }
        }
    }

    pub fn generate_workout(intensity: u32, random_number: u32) -> String {
        let mut expensive_result = Cacher::new(|intensity| {
            println!("Calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            intensity
        });

        if intensity > 25 {
            format!("Next do {} crunches!", expensive_result.value(intensity))
        } else {
            if random_number == 3 {
                format!("Take a break today, you don't need to work out.")
            } else {
                format!(
                    "Today will be {} minutes of cardio!",
                    expensive_result.value(intensity)
                )
            }
        }
    }

    #[test]
    fn cacher_caches() {
        let mut cacher = Cacher::new(|val| val + 1234 as u32);
        cacher.value(1111);
        cacher.value(2222);

        assert_eq!(3456, cacher.value(2222));
        assert_eq!(2345, cacher.value(1111));
    }

    // #[test]
    // XXX SOS I CAN'T MAKE THIS PASS
    // fn cacher_ops_on_T() {
    //     let some_value = "This is a str!";
    //     let mut cacher = Cacher::new(|val| val);
    //     cacher.value(some_value);

    //     assert_eq!(some_value, cacher.value(some_value))
    // }

    #[test]
    fn closure_owns_its_env() {
        let x = 5;
        let anon = |y| y + x;

        fn do_stuff(clos: impl Fn(i32) -> i32, num: i32) -> i32 {
            clos(num)
        }

        assert_eq!(6, do_stuff(anon, 1));
    }
}

#[cfg(test)]
mod closures {
    use super::custom_workout_builder;

    #[test]
    fn expensive_function() {
        // > 25 equals muscle workout
        let user_input_intensity = 20;
        // if 3, you're off the hook
        let sim_rand_num = 10;

        assert_eq!(
            "Today will be 20 minutes of cardio!",
            custom_workout_builder::generate_workout(user_input_intensity, sim_rand_num)
        );
    }
}
