# Rust Book exercises
Just learning 2 Rust with [The Book™](https://doc.rust-lang.org/stable/book/). So far: way more fun than TypeScript. 

`cargo run --bin ch_5`  
Create `Rectangle` structs with methods that operate on their own instance. Enter the dimensions for two rectangles and learn if the second `Rectangle` fits into the first.

`cargo run --bin ch_8`  
Interactive company directory program from the end of chapter 8. Demonstrates collections such as `Vec`, `HashMap` to store user input. Prompts you to create a new `Company` where you can create departments 
which employees can be added to. You can retreive an alphabetically sorted list of employee names per department.

`cargo run --bin ch_10`  

`cargo test --quiet`  
Quiet to bypass unused code warnings in `lib`.

test ch_8::math::calculate_mean ... ok  
test ch_8::math::calculate_mode ... ok  
test ch_8::math::calculate_median ... ok  
test ch_8::pig_latin_converter::converts_to_pig_latin ... ok  
test ch_10::traits::summarize_trait ... ok  
test ch_10::traits::summarize_trait_default_impl ... ok  
test util::sorting::selection ... ok  
