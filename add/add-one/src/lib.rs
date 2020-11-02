pub fn add_one(num: i32) -> i32 {
    num + 1
}

#[cfg(test)]
mod tests {
    use super::add_one;

    #[test]
    fn it_adds_one() {
        assert_eq!(2, add_one(1));
    }
}
