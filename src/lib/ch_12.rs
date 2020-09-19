pub mod minigrep {
    use std::fs;

    pub struct Config {
        pub query: String,
        pub filename: String,
    }

    impl Config {
        pub fn new(args: &[String]) -> Result<Config, &'static str> {
            if !(args.len() < 3) {
                Ok(Config {
                    query: args[1].clone(),
                    filename: args[2].clone(),
                })
            } else {
                std::result::Result::Err("Wrong number of arguments specified!")
            }
        }
    }

    // because both args to search are references we must explicitly declare that the return types
    // reference belongs to one or the other, Rust cannot infer this
    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        // parse contents line by line, push any line containing the substring to the result vec
        let mut result: Vec<&'a str> = Vec::new();
        for line in contents.lines() {
            if line.contains(query) {
                result.push(line);
            }
        }

        result
    }

    pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(config.filename)?;

        for line in search(&config.query, &contents).iter() {
            println!("{}", line);
        }

        Ok(())
    }
}

#[cfg(test)]
mod minigrep_tests {
    use super::minigrep;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            minigrep::search(query, contents)
        );
    }

    #[test]
    fn multiple_results() {
        let query = "the";
        let contents = std::fs::read_to_string("poem.txt").expect("File to exist");

        assert_eq!(
            vec![
                "Then there’s a pair of us - don’t tell!",
                "To tell your name the livelong day",
            ],
            minigrep::search(&query, &contents)
        )
    }
}
