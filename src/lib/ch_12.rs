pub mod minigrep {
    use std::fs;

    pub struct Config {
        pub query: String,
        pub filename: String,
        pub case_sensitive: bool,
    }

    impl Config {
        pub fn new(args: std::env::Args) -> Result<Config, &'static str> {
            let flag_abbreviations: std::collections::HashMap<String, String> =
                [("-c".to_string(), "--case-insensitive".to_string())]
                    .iter()
                    .cloned()
                    .collect();
            let (mut filtered_args, flags) = split_args(args, flag_abbreviations);

            let query = match filtered_args.next() {
                Some(q) => q,
                None => return std::result::Result::Err("You didn't enter a search query!"),
            };

            let filename = match filtered_args.next() {
                Some(f) => f,
                None => {
                    return std::result::Result::Err("You didn't enter a filename to search in!")
                }
            };

            let case_sensitive: bool = match flags.get("--case-insensitive") {
                Some(_) => false,
                None => std::env::var("CASE_INSENSITIVE").is_err(),
            };

            Ok(Config {
                query,
                filename,
                case_sensitive,
            })
        }
    }

    fn split_args<'a>(
        args: std::env::Args,
        flag_abbreviations: std::collections::HashMap<String, String>,
    ) -> (
        std::vec::IntoIter<String>,
        std::collections::HashMap<String, bool>,
    ) {
        let mut filtered_args: Vec<String> = Vec::new();
        let mut flags: std::collections::HashMap<String, bool> = std::collections::HashMap::new();

        args.enumerate().for_each(|(index, arg)| {
            if index != 0 {
                if arg.starts_with("-") {
                    match flag_abbreviations.get(&arg) {
                        Some(full) => {
                            flags.insert(full.clone(), true);
                        }
                        None => {
                            flags.insert(arg, true);
                        }
                    };
                } else {
                    filtered_args.push(arg);
                }
            }
        });

        (filtered_args.into_iter(), flags)
    }

    /// Return a filtered list of `contents` substrings  containing `query`.
    /// Case is considered.
    ///
    /// # Example
    /// ```
    /// let contents = "Hello\nworld!";
    /// let query = "!";
    ///
    /// assert_eq!(vec!["world!"], crate::lib::minigrep::search_case_insensitive("!", &contents));
    /// ```
    pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        // parse contents line by line, push any line containing the substring to the result vec
        // let mut result: Vec<&'a str> = Vec::new();
        // for line in contents.lines() {
        //     if line.contains(query) {
        //         result.push(line);
        //     }
        // }

        // result
        //
        // On iterator vs imperative patterns:
        // "It’s a bit tougher to get the hang of at first, but once you get a feel for the various
        // iterator adaptors and what they do, iterators can be easier to understand."
        //
        // lol

        contents
            .lines()
            .filter(|line| line.contains(&query))
            .collect()
    }

    /// Return a filtered list of `contents` substrings  containing `query`.
    /// Case is ignored.
    ///
    /// # Example
    /// ```
    /// let contents = "Hello\nworld!";
    /// let query = "WoRlD";
    ///
    /// assert_eq!(vec!["world!"], crate::lib::minigrep::search_case_insensitive("!", &contents));
    /// ```
    pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        // let mut result: Vec<&'a str> = Vec::new();

        // for line in contents.lines() {
        //     if line.to_lowercase().contains(&query.to_lowercase()) {
        //         result.push(line);
        //     }
        // }

        // result

        contents
            .lines()
            .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
            .collect()
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
