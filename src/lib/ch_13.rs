pub mod custom_workout_builder {
    use std::thread;
    use std::time::Duration;

    // fn simulated_expensive_function(intensity: u32) -> u32 {
    //     println!("Calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     intensity
    // }

    pub fn generate_workout(intensity: u32, random_number: u32) -> String {
        let _prescription = |intensity| {
            println!("Calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            intensity
        };

        let prescription = _prescription(intensity);

        if intensity > 25 {
            format!("Next do {} crunches!", prescription)
        } else {
            if random_number == 3 {
                format!("Take a break today, you don't need to work out.")
            } else {
                format!("Today will be {} minutes of cardio!", prescription)
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
