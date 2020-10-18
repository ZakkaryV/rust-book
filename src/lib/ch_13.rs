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

    #[test]
    fn iterators_iterate() {
        // Vec implements std::iter::Iter
        let list = vec!['a', 'b', 'c'];

        // iterators must be mutable because their internal state is updated on each next call,
        // therefor iterators are consumed upon their call
        let mut iterable = list.iter();

        assert_eq!(&list[0], iterable.next().unwrap());
        assert_eq!(&list[1], iterable.next().unwrap());
        assert_eq!(&list[2], iterable.next().unwrap());
        assert_eq!(None, iterable.next());
    }

    #[test]
    fn iterators_sum() {
        let nums = vec![5, 10, 20];
        let nums_iter = nums.iter();
        // sum() takes ownership of nums_iter: we would not be able to call this a second time
        let sum: i32 = nums_iter.sum();

        assert_eq!(35, sum);
    }

    #[test]
    fn iterator_adapters() {
        let v_1: Vec<i32> = vec![1, 2, 3];
        // unlike sum, who consumes iterators, map returns a new iterator, also LAZY
        // (closure evocation is delayed until the call to .collect)
        let v_2 = v_1.iter().map(|x| x + 1);

        assert_eq!(vec![2, 3, 4], v_2.collect::<Vec<i32>>());
    }
}

pub mod custom_iterator {
    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    #[test]
    fn counter_counts() {
        // only counts to five
        let mut c = Counter::new();

        assert_eq!(Some(1), c.next());
        assert_eq!(Some(2), c.next());
        assert_eq!(Some(3), c.next());
        assert_eq!(Some(4), c.next());
        assert_eq!(Some(5), c.next());
        assert_eq!(None, c.next());
    }

    #[test]
    fn chaining_myriad_iterator_methods() {
        // the type annotation is important here, otherwise Iterator can't
        // know which trait implementation to use
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();

        assert_eq!(18, sum);
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
