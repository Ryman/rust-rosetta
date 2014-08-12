//Implements http://rosettacode.org/wiki/Primality_by_Trial_Division

use std::iter::range_step_inclusive;

trait Prime { fn is_prime(&self) -> bool; }

impl Prime for int {
    fn is_prime(&self) -> bool {
        if *self < 0 || (self % 2 == 0 && *self != 2) {
            return false;
        }

        let limit = (*self as f32).sqrt() as int;

        // We test if the number is divisible by any odd number up to the limit
        range_step_inclusive(3, limit, 2).all(|x| self % x != 0)
    }
}

#[cfg(not(test))]
fn main() {
    println!("{:b}", 15485863.is_prime()); // The 1 000 000th prime.
    println!("{:b}", 62773913.is_prime()); // The product of the 1000th and 1001st primes.
}

#[test]
fn test_one() {
    assert!(1.is_prime());
}

#[test]
fn test_two() {
    assert!(2.is_prime());
}

#[test]
fn test_negative() {
    assert!(!(-2).is_prime());
}

#[test]
fn test_many() {
    let primes = [3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
    assert!(primes.iter().all(|&x| x.is_prime()));
}
