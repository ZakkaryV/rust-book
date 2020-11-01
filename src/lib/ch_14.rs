//! # Art
//!
//! Color types and color mixing capabilities.

pub mod art {
    pub use kinds::{PrimaryColors, SecondaryColors};
    pub use utils::mix;

    mod kinds {
        pub enum PrimaryColors {
            Red,
            Blue,
            Yellow,
        }

        pub enum SecondaryColors {
            Purple,
            Orange,
            Green,
        }
    }

    mod utils {
        use super::kinds::{PrimaryColors, SecondaryColors};

        /// # Example
        /// ```
        /// use crate::lib::art::{PrimaryColors, SecondaryColors, mix};
        ///
        /// let red = PrimaryColors::Red;
        /// let blue = PrimaryColors::Blue;
        ///
        /// assert!(matches!(mix(red, blue), SecondaryColors::Purple));
        /// ```
        pub fn mix(color_1: PrimaryColors, color_2: PrimaryColors) -> SecondaryColors {
            match color_1 {
                PrimaryColors::Red => match color_2 {
                    PrimaryColors::Blue => SecondaryColors::Purple,
                    PrimaryColors::Yellow => SecondaryColors::Orange,
                    _ => panic!("Expected two different primary colors"),
                },
                PrimaryColors::Blue => match color_2 {
                    PrimaryColors::Red => SecondaryColors::Purple,
                    PrimaryColors::Yellow => SecondaryColors::Green,
                    _ => panic!("Expected two different primary colors"),
                },
                PrimaryColors::Yellow => match color_2 {
                    PrimaryColors::Red => SecondaryColors::Orange,
                    PrimaryColors::Blue => SecondaryColors::Green,
                    _ => panic!("Expected two different primary colors"),
                },
            }
        }
    }
}
