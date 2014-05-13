// Implements http://rosettacode.org/wiki/Anagrams

extern crate collections;

use collections::{HashMap, HashSet};
use std::str;

#[cfg(test)]
use std::io::MemReader;

#[cfg(not(test))]
use std::io::{File, BufferedReader};
use std::cmp::max;

fn sort_string(string: &str) -> ~str {
    let mut chars: Vec<char> = string.chars().collect();
    chars.sort();
    str::from_chars(chars.as_slice())
}

// Returns groups of anagrams where each group consists of a set
// containing the words
fn get_anagrams<T: Buffer>(mut reader: T) -> HashMap<~str, HashSet<~str>> {
    let mut groups: HashMap<~str, HashSet<~str>> = HashMap::new();

    // Make groups of words according to the letters they contain
    for line in reader.lines() {
        let s = line.unwrap().trim().to_owned();
        groups.insert_or_update_with(
                                     // The key
                                     sort_string(s.clone()),
                                     // The value
                                     {
                                         let mut set = HashSet::new();
                                         set.insert(s.clone());
                                         set
                                     },
                                     // The closure to update the value
                                     |_, group| { group.insert(s.clone()); });
    }

    groups
}

// Returns the groups of anagrams that contain the most words in them
fn get_biggest_groups(groups: &HashMap<~str, HashSet<~str>>) ->
 HashMap<~str, HashSet<~str>> {
    let max_length =
        groups.iter().fold(0,
                           |current_max, (_, group)|
                               max(current_max, group.len()));
    groups.iter().filter(|&(_, group)|
                             group.len() ==
                                 max_length).map(|(x, y)|
                                                     (x.clone(),
                                                      y.clone())).collect()
}

#[cfg(not(test))]
fn main() {
    let path = Path::new("src/resources/unixdict.txt");
    let reader = BufferedReader::new(File::open(&path));

    let anagram_groups = get_anagrams(reader);
    let biggest_groups = get_biggest_groups(&anagram_groups);

    // Print the words in the biggest groups of anagrams
    for (_, group) in biggest_groups.iter() {
        for word in group.iter() { print!("{} " , * word) }
        println!("")
    }
}

#[test]
fn basic_test() {
    // Groups of anagrams
    let group1: HashSet<~str> =
        vec!("lane" . to_owned ( ) , "neal" . to_owned ( ) , "lean" . to_owned
             ( )).move_iter().collect();
    let group2: HashSet<~str> =
        vec!("angel" . to_owned ( ) , "angle" . to_owned ( ) , "galen" .
             to_owned ( )).move_iter().collect();
    let group3: HashSet<~str> =
        vec!("glare" . to_owned ( ) , "large" . to_owned
             ( )).move_iter().collect();

    // Prepare the input for the program
    // We will get a string like "lane\nneal\nlean\nangel\nangle..."
    let mut word_iter =
        group1.iter().chain(group2.iter().chain(group3.iter()));
    let mut words = StrBuf::new();

    words.push_str(word_iter.next().unwrap().as_slice());
    for word in word_iter {
        words.push_str("\n");
        words.push_str(word.as_slice());
    }

    // Here begins the real testing
    let all_groups =
        get_anagrams(MemReader::new(words.to_str().bytes().collect()));
    let biggest_groups = get_biggest_groups(&all_groups);

    // Groups 1, 2 and 3 are contained in "all_groups"
    assert!(all_groups . iter ( ) . any
            ( | ( _ , group ) | * group == group1 ));
    assert!(all_groups . iter ( ) . any
            ( | ( _ , group ) | * group == group2 ));
    assert!(all_groups . iter ( ) . any
            ( | ( _ , group ) | * group == group3 ));

    // Groups 1 and 2 are contained in "biggest_groups". Group 3 is not.
    assert!(biggest_groups . iter ( ) . any
            ( | ( _ , group ) | * group == group1 ));
    assert!(biggest_groups . iter ( ) . any
            ( | ( _ , group ) | * group == group2 ));
    assert!(biggest_groups . iter ( ) . all
            ( | ( _ , group ) | * group != group3 ));
}
