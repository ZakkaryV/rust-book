use std::cmp::Ordering;

pub fn selection_sort_int(v: &mut [i32]) -> &[i32] {
    let len = v.len();

    for i in 0..len {
        let mut lowest = i;

        for j in (i + 1)..len {
            if let Ordering::Less = v[j].cmp(&v[lowest]) {
                lowest = j;
            }
        }

        v.swap(i, lowest);
    }

    v
}

#[cfg(test)]
mod sorting {
    use super::selection_sort_int;

    #[test]
    fn selection() {
        let mut unsorted_arr = vec![7, 6, 5, 4, 3, 2, 1];
        let result = [1, 2, 3, 4, 5, 6, 7];

        assert_eq!(selection_sort_int(&mut unsorted_arr), &result);
    }
}
