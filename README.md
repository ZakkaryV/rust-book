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
cargo run --bin ch_12_minigrep "to" ./poem.txt
```

Minigrep is case-sensitive by default. Pass the `--case-insensitive` flag (or `-c` for short) to perform case-insensitive search.
```
cargo run --bin ch_12_minigrep "somEbOdY" --case-insensitive  ./poem.txt
                                          ^^^^^^^^^^^^^^^^^^ 
```

Alernatively the `CASE_INSENSITIVE` env var will be checked to determine search case sensetivity setting. Flags take precendence.  
```
CASE_INSENSITIVE=1 cargo run --bin ch_12_minigrep "to" ./poem.txt
^^^^^^^^^^^^^^^^^^
```

## Libraries
Run the unit and integration tests.
```
cargo test

running 29 tests
test util::sorting::selection ... ok  
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
test ch_11::adder::flakey_test ... FAILED <-------------- might have to try this one a few times
test ch_11::adder::greeter_greets ... ok  
test ch_11::adder::test_returns_result ... ok  
test ch_12::minigrep_tests::one_result ... ok
test ch_12::minigrep_tests::multiple_results ... ok
test ch_12::minigrep_tests::searches_case_insensitive ... ok
test ch_13::closures::expensive_function ... ok
test ch_13::custom_workout_builder::closure_owns_its_env ... ok
test ch_13::custom_workout_builder::cacher_caches ... ok
test ch_13::custom_workout_builder::cacher_ops_on_generic_closures ... FAILED <-- bypassed bc I can't figure out how to make explicity typed closures use generics :( 
test ch_13::custom_workout_builder::iterator_adapters ... ok
test ch_13::custom_workout_builder::iterators_iterate ... ok
test ch_13::custom_workout_builder::iterators_sum ... ok
test ch_13::custom_iterator::counter_counts ... ok
test ch_13::custom_iterator::chaining_myriad_iterator_methods ... ok

Running target/debug/deps/integration_test-3d1ca2eca157be7d

running 1 test
test sort_vec_ascending ... ok

// documentation tests
running 3 tests
test src/lib/ch_12.rs - ch_12::minigrep::search_case_insensitive (line 111) ... ok
test src/lib/ch_12.rs - ch_12::minigrep::search_case_sensitive (line 78) ... ok
test src/lib/ch_14.rs - ch_14::art::utils::mix (line 27) ... ok

```
