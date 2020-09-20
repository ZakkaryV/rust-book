# Rust Book exercises
Learning how 2 Rust with [The Book™](https://doc.rust-lang.org/stable/book/)!   

## Binaries
### Ch. 05: Using Structs to Structure Related Data
Create `Rectangle` structs with methods that operate on their own instance. Enter the dimensions for two rectangles and learn if the second `Rectangle` fits into the first.
```
cargo run --bin ch_05_rectangles
```

### Ch. 08: Common Collections
Interactive company directory program from the end of chapter 8. Demonstrates collections such as `Vec`, `HashMap` to store user input. Prompts you to create a new `Company` where you can create departments 
    which employees can be added to. You can retreive an alphabetically sorted list of employee names per department.
```
cargo run --bin ch_08_company
```

### Ch. 12: An I/O Project: Building a Command Line Program
Run the grep clone binary. First argument is the search query, second is the file to search.  
```
cargo run --bin ch_12_minigrep "!" ~/Projects/rust-book/poem.txt
```

Minigrep is case-sensitive by default. Pass the `--case-insensitive` flag (or `-c` for short) to perform case-insensitive search.
```
cargo run --bin ch_12_minigrep "somEbOdY" --case-insensitive  ~/Projects/rust-book/poem.txt
                                          ^^^^^^^^^^^^^^^^^^ 
```

Alernatively the `CASE_INSENSITIVE` env var will be checked to determine search case sensetivity setting. Flags take precendence.  
```
CASE_INSENSITIVE=1 cargo run --bin ch_12_minigrep "to" ~/Projects/rust-book/poem.txt
^^^^^^^^^^^^^^^^^^
```

## Libraries
Run the unit and integration tests.
```
cargo test

running 19 tests
test ch_05::tests::larger_can_hold_smaller ... ok  
test ch_05::tests::smaller_can_hold_larger ... ok  
test ch_08::math::calculate_mean ... ok  
test ch_08::math::calculate_median ... ok  
test ch_08::math::calculate_mode ... ok  
test ch_08::pig_latin_converter::converts_to_pig_latin ... ok  
test ch_09::tests::guess_panics_if_outside_range ... ok  
test ch_10::traits::point_mixes ... ok  
test ch_10::traits::return_generic_largest ... ok  
test ch_10::traits::summarize_trait ... ok  
test ch_10::traits::summarize_trait_default_impl ... ok  
test ch_10::traits::trait_bounds_and_parameters ... ok  
test ch_11::adder::adder_adds ... ok  
test ch_11::adder::two_plus_two ... ignored  
test ch_11::adder::flakey_test ... FAILED <------------------ might have to try this one a few times
test ch_11::adder::greeter_greets ... ok  
test ch_11::adder::test_returns_result ... ok  
test ch_12::minigrep_tests::one_result ... ok
test ch_12::minigrep_tests::multiple_results ... ok
test ch_12::minigrep_tests::searches_case_insensitive ... ok
test util::sorting::selection ... ok  

Running target/debug/deps/integration_test-3d1ca2eca157be7d

running 1 test
test sort_vec_ascending ... ok
```
