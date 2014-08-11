// Given a set, generate its power set, which is the set of all subsets of that
// set: http://rosettacode.org/wiki/Power_set

// If set == {}
//   return {{}}
// else if set == {a} U rest
//   return power_set(rest) U ({a} U each set in power_set(rest))
fn power_set<T: Clone, I: Iterator<T>>(mut items: I) -> Vec<Vec<T>> {
    match items.next() {
        None => vec![vec![]],
        Some(item) => {
            power_set(items).move_iter().fold(vec![], |power, set| {
                power.append_one(set.clone())
                     .append_one(set.append_one(item.clone()))
            })
        }
    }
}

#[cfg(not(test))]
fn main() {
    let set = &[1i, 2, 3, 4];
    let power = power_set(set.iter());
    println!("Set      : {}", set);
    println!("Power Set: {}", power);
}

#[test]
fn test() {
    let set = Vec::<int>::new();
    let power = power_set(set.iter());
    assert_eq!(power, vec![vec![]]);

    let set = &[1i, 2, 3];
    let power = power_set(set.iter().map(|&i| i));
    assert_eq!(power, vec![vec![], vec![1], vec![2], vec![2, 1],
                          vec![3], vec![3, 1], vec![3, 2], vec![3, 2, 1]]);
}
