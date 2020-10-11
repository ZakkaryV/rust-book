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
        value: Option<u32>,
    }

    impl<T: Fn(u32) -> u32> Cacher<T> {
        fn new(closure: T) -> Cacher<T> {
            Cacher {
                calculation: closure,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let result = (self.calculation)(arg);
                    self.value = Some(result);
                    result
                }
            }
        }
    }
    // Cacher::new(|| )

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
