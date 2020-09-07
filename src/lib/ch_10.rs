pub mod content {
    use std::fmt;

    pub struct Tweet<T> {
        pub username: String,
        pub content: T,
        pub reply: bool,
        pub retweet: bool,
    }

    // pub fn notify(item: &impl Summary) -> String {
    //     format!("Breaking news!! {}", item.summarize())
    // }
    //
    // above ex. using impl syntax, here with trait bounds and where syntax
    pub fn notify<T>(item: &T) -> String
    where
        T: Summary + fmt::Display,
    {
        format!("Breaking news!! {}", item.summarize())
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

    impl fmt::Display for Tweet<String> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "@{}: {}", self.username, self.content)
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
            format!(
                "{}'s Tweet has been deleted for violating Twitter's hate speech policies.",
                self.summarize_author(),
            )
        }
    }
}

mod point {
    #[derive(Debug)]
    pub struct Point<T, U> {
        pub x: T,
        pub y: U,
    }

    impl<T, U> Point<T, U> {
        pub fn mixup<V, W>(self, pt: Point<V, W>) -> Point<T, W> {
            Point { x: self.x, y: pt.y }
        }
    }

    pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }
}

#[cfg(test)]
mod traits {
    use super::content::{notify, NewsArticle, Summary, Tweet};
    use super::point::{largest, Point};

    #[test]
    fn summarize_trait() {
        let tweet = Tweet {
            username: "yourbrainonporn".to_string(),
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
            username: "yourbrainonporn".to_string(),
            content: false,
            reply: false,
            retweet: false,
        };

        assert_eq!(
            "@yourbrainonporn's Tweet has been deleted for violating Twitter's hate speech policies.",
            tweet.summarize()
        )
    }

    #[test]
    fn trait_bounds_and_parameters() {
        let tweet = Tweet {
            username: "yourbrainonporn".to_string(),
            content: "Buy my book!".to_string(),
            reply: false,
            retweet: false,
        };

        assert_eq!(
            "Breaking news!! yourbrainonporn: Buy my book!",
            notify(&tweet)
        )
    }

    #[test]
    fn return_generic_largest() {
        let list_i32 = vec![100, 242, 435, 852, 120, 529, 129];
        assert_eq!(852, largest(&list_i32));

        let list_char = vec!['a', 'z', 'b', 'y', 'c'];
        assert_eq!('z', largest(&list_char));
    }

    #[test]
    fn point_mixes() {
        let pt1 = Point { x: 10, y: 'c' };
        let pt2 = Point {
            x: "Hi!!!!",
            y: 6.66,
        };
        let pt3 = pt1.mixup(pt2);

        assert_eq!(10, pt3.x);
        assert_eq!(6.66, pt3.y);
    }
}
