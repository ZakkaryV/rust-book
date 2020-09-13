mod common;

use lib::selection_sort_int;

#[test]
fn sort_vec_ascending() {
    let mut v = common::get_random_vec(10);
    let mut sorted = v.clone();
    sorted.sort();

    assert_eq!(sorted, selection_sort_int(&mut v));
}
