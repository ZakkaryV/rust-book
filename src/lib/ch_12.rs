pub mod minigrep {
    use std::fs;

    pub struct Config {
        pub query: String,
        pub filename: String,
        pub case_sensitive: bool,
    }

    impl Config {
        pub fn new(args: &[String]) -> Result<Config, &str> {
            let flag_abbreviations: std::collections::HashMap<String, String> =
                [("-c".to_string(), "--case-insensitive".to_string())]
                    .iter()
                    .cloned()
                    .collect();
            let (filtered_args, flags) = split_args(args, flag_abbreviations);

            if !(filtered_args.len() < 3) {
                let case_sensitive: bool = match flags.get("--case-insensitive") {
                    Some(_) => false,
                    None => std::env::var("CASE_INSENSITIVE").is_err(),
                };

                Ok(Config {
                    query: filtered_args[1].to_owned(),
                    filename: filtered_args[2].to_owned(),
                    case_sensitive,
                })
            } else {
                std::result::Result::Err("Wrong number of arguments specified!")
            }
        }
    }

    fn split_args<'a>(
        args: &[String],
        flag_abbreviations: std::collections::HashMap<String, String>,
    ) -> (Vec<&str>, std::collections::HashMap<String, bool>) {
        let mut i = 0;
        let mut filtered_args: Vec<&str> = Vec::new();
        let mut flags: std::collections::HashMap<String, bool> = std::collections::HashMap::new();

        while i < args.len() {
            if args[i].starts_with("-") {
                match flag_abbreviations.get(&args[i]) {
                    Some(full) => {
                        flags.insert(full.to_owned(), true);
                    }
                    None => {
                        flags.insert(args[i].to_owned(), true);
                    }
                };
            } else {
                filtered_args.push(&args[i]);
            }

            i = i + 1;
        }

        (filtered_args, flags)
    }

    // because both args to search are references we must explicitly declare that the return types
    // reference belongs to one or the other, Rust cannot infer this
    pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        // parse contents line by line, push any line containing the substring to the result vec
        let mut result: Vec<&'a str> = Vec::new();
        for line in contents.lines() {
            if line.contains(query) {
                result.push(line);
            }
        }

        result
    }

    pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut result: Vec<&'a str> = Vec::new();

        for line in contents.lines() {
            if line.to_lowercase().contains(&query.to_lowercase()) {
                result.push(line);
            }
        }

        result
    }

    pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(config.filename)?;

        let results = if config.case_sensitive {
            search_case_sensitive(&config.query, &contents)
        } else {
            search_case_insensitive(&config.query, &contents)
        };

        for line in results.iter() {
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
            minigrep::search_case_sensitive(query, contents)
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
            minigrep::search_case_sensitive(&query, &contents)
        )
    }

    #[test]
    fn searches_case_insensitive() {
        let query = "LazY dOg";
        let contents = "tHe QuIcK bRoWn\
FoX jUmPs OvEr
ThE lAzY dOg";

        assert_eq!(
            vec!["ThE lAzY dOg"],
            minigrep::search_case_insensitive(&query, &contents)
        );
    }
}
