// Implements http://rosettacode.org/wiki/Happy_numbers

use std::collections::treemap::TreeSet;

fn digits(mut n: uint) -> Vec<uint> {
    let mut ds = vec![];
    if n == 0 {
        return vec![0];
    }
    while n > 0 {
        ds.push(n % 10);
        n /= 10;
    }
    ds.reverse();
    ds
}

fn is_happy(mut x: uint) -> bool {
    let mut past = TreeSet::new();
    while x != 1 {
        // Take the sum of the squares of the digits of x
        x = digits(x).iter().fold(0, |a, &b| a + b * b);

        // The number is not happy if there is an endless loop
        if past.contains(&x) {
            return false
        }

        past.insert(x);
    }
    true
}

#[cfg(not(test))]
fn main() {
    use std::iter::count;
    // Print the first 8 happy numbers
    let v = count(1u, 1).filter(|x| is_happy(*x))
                        .take(8)
                        .collect::<Vec<uint>>();
    println!("{}", v)
}

#[test]
fn test_digits() {
    fn t(n: uint, expected: &[uint]) {
        assert_eq!(digits(n).as_slice(), expected);
    }

    t(0, [0]);
    t(1, [1]);
    t(2, [2]);
    t(10, [1, 0]);
    t(11, [1, 1]);
    t(101, [1, 0, 1]);
    t(1000, [1, 0, 0, 0]);
}

#[test]
fn test_is_happy() {
    let happys = [1u, 7, 10, 13, 19, 23, 28, 31, 1607, 1663];
    let unhappys = [0u, 2, 3, 4, 5, 6, 8, 9, 29, 1662];

    assert!(happys.iter().all(|&n| is_happy(n)));
    assert!(unhappys.iter().all(|&n| !is_happy(n)));
}
