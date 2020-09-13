use rand::prelude::*;

pub fn get_random_vec(len: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut result: Vec<i32> = (1..len).collect();
    result.shuffle(&mut rng);

    result
}
