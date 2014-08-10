// Implements http://rosettacode.org/wiki/Letter_frequency

use std::collections::HashMap;

fn count_chars<T: Iterator<char>>(mut chars: T) -> HashMap<char, uint> {
    let mut map = HashMap::new();
    for letter in chars {
        map.insert_or_update_with(letter, 1u, |_, count| *count += 1);
    }
    map
}

#[cfg(not(test))]
fn main() {
    use std::io::fs::File;
    use std::io::BufferedReader;

    let file = File::open(&Path::new("resources/unixdict.txt"));
    let mut reader = BufferedReader::new(file);

    println!("{}", count_chars(reader.chars().map(|c| c.unwrap())));
}

#[test]
fn test_empty() {
    let map = count_chars("".chars());
    assert_eq!(map.len(), 0);
}

#[test]
fn test_basic() {
    let map = count_chars("aaaabbbbc".chars());

    assert_eq!(map.len(), 3);
    assert_eq!(*map.get(&'a'), 4);
    assert_eq!(*map.get(&'b'), 4);
    assert_eq!(*map.get(&'c'), 1);
}
