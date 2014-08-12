//Implements http://rosettacode.org/wiki/Sorting_algorithms/Quicksort

// Used by the tests
#[cfg(test)]
use std::rand::{task_rng, Rng};

// We use in place quick sort
// For details see http://en.wikipedia.org/wiki/Quicksort#In-place_version
fn quick_sort<T: Ord>(v: &mut [T]) {
    let len = v.len();
    if len < 2 { return }

    let pivot_index = partition(v);

    // Sort the left side
    quick_sort(v.mut_slice(0, pivot_index));

    // Sort the right side
    quick_sort(v.mut_slice(pivot_index + 1, len));
}

// Reorders the slice with values lower than the pivot at the left side,
// and values bigger than it at the right side.
// Also returns the store index.
fn partition<T: Ord>(v: &mut [T]) -> uint {
    let len = v.len();
    let pivot_index = len / 2;

    v.swap(pivot_index, len - 1);

    let mut store_index = 0;
    for i in range(0, len - 1) {
        if v[i] <= v[len - 1] {
            v.swap(i, store_index);
            store_index += 1;
        }
    }

    v.swap(store_index, len - 1);
    store_index
}

#[cfg(not(test))]
fn main() {
    // Sort numbers
    let numbers = &mut [4i, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {}", numbers);

    quick_sort(numbers);
    println!("After: {}", numbers);

    // Sort strings
    let strings = &mut ["beach", "hotel", "airplane", "car", "house", "art"];
    println!("Before: {}", strings);

    quick_sort(strings);
    println!("After: {}", strings);
}



#[cfg(test)]
fn check_sort<T: Ord>(v: &mut [T]) {
    quick_sort(v);

    for i in range(1, v.len()) {
        assert!(v[i - 1] <= v[i]);
    }
}

#[test]
fn test_rosetta_vector() {
    let mut numbers = [4i, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    check_sort(numbers);
}

#[test]
fn test_empty_vector() {
    let mut numbers: Vec<int> = Vec::new();
    check_sort(numbers.as_mut_slice());
}

#[test]
fn test_one_element_vector() {
    let mut numbers = [0i];
    check_sort(numbers);
}

#[test]
fn test_repeat_vector() {
    let mut numbers = [1i, 1, 1, 1, 1];
    check_sort(numbers);
}

#[test]
fn test_worst_case_vector() {
    let mut numbers = [20i, 10, 0, -1, -5];
    check_sort(numbers);
}

#[test]
fn test_already_sorted_vector() {
    let mut numbers = [-1i, 0, 3, 6, 99];
    check_sort(numbers);
}

#[test]
fn test_random_numbers() {
    let mut rng = task_rng();
    let mut numbers = rng.gen_iter::<int>().take(500).collect::<Vec<int>>();
    check_sort(numbers.as_mut_slice());
}
