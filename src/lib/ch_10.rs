pub mod content {
    pub struct Tweet<T> {
        pub username: String,
        pub content: T,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet<String> {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }

        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    impl Summary for Tweet<bool> {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            format!("Written by {} from {}", self.author, self.location)
        }

        fn summarize(&self) -> String {
            format!("{}, by {}", self.headline, self.author)
        }
    }

    pub trait Summary {
        fn summarize_author(&self) -> String;

        fn summarize(&self) -> String {
            String::from("Tweet is deleted due to violating Twitter's hate speech policies.")
        }
    }
}

#[cfg(test)]
mod traits {
    use super::content::{NewsArticle, Summary, Tweet};

    #[test]
    fn summarize_trait() {
        let tweet = Tweet {
            username: "@yourbrainonporn".to_string(),
            content:
                "Check out the hundreds of studies proving physiological pornography addiction!"
                    .to_string(),
            reply: false,
            retweet: false,
        };

        let article = NewsArticle {
            headline: "New study shows average age of first exposure to pornogaphy is 6".to_string(),
            location: "Eurasia".to_string(),
            author: "Goldstein".to_string(),
            content: "There is a scientific concensus, the industry just can't afford for you to find out.".to_string(),
        };

        assert_eq!(
            format!("{}: {}", tweet.username, tweet.content),
            tweet.summarize()
        );

        assert_eq!(
            format!("{}, by {}", article.headline, article.author),
            article.summarize()
        );
    }

    #[test]
    fn summarize_trait_default_impl() {
        let tweet = Tweet {
            username: "@yourbrainonporn".to_string(),
            content: false,
            reply: false,
            retweet: false,
        };

        assert_eq!(
            "Tweet is deleted due to violating Twitter's hate speech policies.",
            tweet.summarize()
        )
    }
}
