use add_one::add_one;

pub fn add_two(num: i32) -> i32 {
    add_one(add_one(num))
}

#[cfg(test)]
mod tests {
    use add_one::add_one;

    #[test]
    fn adds_two() {
        assert_eq!(3, add_one(add_one(1)));
    }
}
