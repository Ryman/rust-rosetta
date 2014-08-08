// Implements http://rosettacode.org/wiki/IBAN

extern crate num;
use std::num::One;
use num::bigint::{BigInt, ToBigInt};

#[cfg(not(test))]
fn main() {
    if is_valid("GB82 WEST 1234 5698 7654 32") {
        println!("IBAN correctly validated!");
    } else {
        println!("Invalid IBAN!");
    }
}

// Returns true if the IBAN is valid
// See http://en.wikipedia.org/wiki/International_Bank_Account_Number#Validating_the_IBAN
fn is_valid(iban: &str) -> bool {
    // Discard whitespace
    let trimmed = iban.chars().filter(|c| !c.is_whitespace()).collect::<String>();
    let iban = trimmed.as_slice();

    // Check length of the IBAN
    if iban.len() < 2 || country_length(iban.slice_to(2)) != Some(iban.len()) {
        return false
    }

    // Rearrange (first four characters go to the back)
    let rearranged = iban.slice_from(4).to_string().append(iban.slice_to(4));

    // Expand letters to digits
    match parse_digits(rearranged.as_slice()) {
        // Check if the remainder is one
        Some(x) => x % 97u.to_bigint().unwrap() == One::one(),
        None => false
    }
}

// Returns a BigInt made from the digits and letters of the IBAN
fn parse_digits(s: &str) -> Option<BigInt> {
    let mut ns = String::with_capacity(s.len() + 10);

    // Copy the digits to the vector and expand the letters to digits
    // We convert the characters to Ascii to be able to transform the vector in a String directly
    for c in s.chars() {
        match c.to_digit(36) {
            Some(d) => ns.push_str(d.to_string().as_slice()),
            None => return None
        };
    }

    from_str(ns.as_slice())
}

fn country_length(country_code: &str) -> Option<uint> {
    let countries = [
        ("AL", 28),
        ("AD", 24),
        ("AT", 20),
        ("AZ", 28),
        ("BE", 16),
        ("BH", 22),
        ("BA", 20),
        ("BR", 29),
        ("BG", 22),
        ("CR", 21),
        ("HR", 21),
        ("CY", 28),
        ("CZ", 24),
        ("DK", 18),
        ("DO", 28),
        ("EE", 20),
        ("FO", 18),
        ("FI", 18),
        ("FR", 27),
        ("GE", 22),
        ("DE", 22),
        ("GI", 23),
        ("GR", 27),
        ("GL", 18),
        ("GT", 28),
        ("HU", 28),
        ("IS", 26),
        ("IE", 22),
        ("IL", 23),
        ("IT", 27),
        ("KZ", 20),
        ("KW", 30),
        ("LV", 21),
        ("LB", 28),
        ("LI", 21),
        ("LT", 20),
        ("LU", 20),
        ("MK", 19),
        ("MT", 31),
        ("MR", 27),
        ("MU", 30),
        ("MC", 27),
        ("MD", 24),
        ("ME", 22),
        ("NL", 18),
        ("NO", 15),
        ("PK", 24),
        ("PS", 29),
        ("PL", 28),
        ("PT", 25),
        ("RO", 24),
        ("SM", 27),
        ("SA", 24),
        ("RS", 22),
        ("SK", 24),
        ("SI", 19),
        ("ES", 24),
        ("SE", 24),
        ("CH", 21),
        ("TN", 24),
        ("TR", 26),
        ("AE", 23),
        ("GB", 22),
        ("VG", 24)
    ];

    countries.iter()
             .find(|&&(country, _)| country == country_code)
             .map(|&(_, length)| length)
}

#[test]
fn test_valid() {
    assert!(is_valid("GB82 WEST 1234 5698 7654 32"));
    assert!(is_valid("BE18 0016 5492 3565"));
}

#[test]
fn test_wrong() {
    assert!(!is_valid("ASDA FJAS DMAF BKDB AKGS DH"));
    assert!(!is_valid("XX82 WEST 1234 5698 7654 32"));
    assert!(!is_valid("BE18 0016 5492 3566"));
    assert!(!is_valid("BE18 0016 5492 3565 6"));
    assert!(!is_valid("BE18 0016 5492 356"));
    assert!(!is_valid(""));
}
