#![allow(dead_code)]

//! # Rust Book Exercises
//!
//! A barrel of modules demonstrating basic Rust lang functionality. Some of them are used in this
//! Crate's binary programs.

mod ch_05;
pub use ch_05::Rectangle;

mod ch_08;
pub use ch_08::{calculate, pig_latin};

mod ch_09;
pub use ch_09::Guess;

mod ch_10;
pub use ch_10::content;

mod ch_11;
pub use ch_11::greeter;

mod ch_12;
pub use ch_12::minigrep;

mod ch_13;
pub use ch_13::custom_workout_builder;

mod ch_14;
pub use ch_14::*;

mod ch_15;
pub use ch_15::*;

mod util;
pub use util::*;
