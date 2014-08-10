// Implements http://rosettacode.org/wiki/Knapsack_problem/0-1

#![feature(macro_rules)]

use std::cmp::max;

// This struct is used to store our items that we want in our knap-sack.
//
// Show is for displaying the fields.
#[deriving(Show)]
pub struct Want<'a> {
    name: &'a str,
    weight: uint,
    value: uint
}

// Global, immutable allocation of our items.
// This is so we can reference this in multiple functions.
// We use a macro to generate the array for convenience.
macro_rules! items(
    ($($name:expr, $weight:expr, $value:expr),*) => (
        static ITEMS : &'static [Want<'static>] = &[
            $( Want { name: $name, weight: $weight, value: $value} ),*
        ];
    )
)

items!("map", 9, 150,
       "compass", 13, 35,
       "water", 153, 200,
       "sandwich", 50, 160,
       "glucose", 15, 60,
       "tin", 68, 45,
       "banana", 27, 60,
       "apple", 39, 40,
       "cheese", 23, 30,
       "beer", 52, 10,
       "suntancream", 11, 70,
       "camera", 32, 30,
       "T-shirt", 24, 15,
       "trousers", 48, 10,
       "umbrella", 73, 40,
       "waterproof trousers", 42, 70,
       "waterproof overclothes", 43, 75,
       "note-case", 22, 80,
       "sunglasses", 7, 20,
       "towel", 18, 12,
       "socks", 4, 50,
       "book", 30, 10)


// This is a bottom-up dynamic programming solution to the 0-1 knap-sack problem.
//      maximize value
//      subject to weights <= max_weight
fn knap_01_dp<'a>(xs: &[Want<'a>], mut capacity: uint) -> Vec<Want<'a>> {
    // Save this value, so we don't have to make repeated calls.
    let xs_len = xs.len();

    // Imagine we wrote a recursive function(item, max_weight) that returns a
    // uint corresponding to the maximum cumulative value by considering a
    // subset of items such that the combined weight <= max_weight.
    //
    // fn best_value(item: uint, max_weight: uint) -> uint{
    //     if item == 0 {
    //         return 0;
    //     }
    //     if xs[item - 1].weight > max_weight {
    //         return best_value(item - 1, max_weight, xs);
    //     }
    //     return max(best_value(item - 1, max_weight, xs),
    //                best_value(item - 1, max_weight - xs[item - 1].weight, xs)
    //                + xs[item - 1].value);
    //     }
    //
    // best_value(xs_len, max_weight) is equal to the maximum value that we
    // can add to the bag.
    //
    // The problem with using this function is that it performs redudant
    // calculations.
    //
    // The dynamic programming solution is to precompute all of the values we
    // need and put them into a 2D array.
    //
    // In a similar vein, the top-down solution would be to memoize the
    // function then compute the results on demand.
    let zeroes = Vec::from_elem(capacity + 1, 0u);
    let mut best = Vec::from_elem(xs_len + 1, zeroes);

    // loop over the items
    for i in range(0, xs_len) {
        // loop over the weights
        for w in range(1, capacity + 1) {
            // do we have room in our knapsack?
            let item = if xs[i].weight > w {
                // if we don't, then we'll say that the value doesn't change
                // when considering this item
                best[i][w]
            } else {
                // if we do, then we have to see if the value we gain by adding
                // the item, given the weight, is better than not adding the item
                max(best[i][w], best[i][w - xs[i].weight] + xs[i].value)
            };

            *best.get_mut(i + 1).get_mut(w) = item;
        }
    }

    // we built up the solution space through a forward pass over the data,
    // now we have to traverse backwards to get the solution
    range(1, xs_len + 1).rev().fold(Vec::with_capacity(xs_len), |mut result, i| {
        // We can check if an item should be added to the knap-sack by comparing
        // best_value with and without this item. If best_value added this
        // item then so should we.
        if best[i][capacity] != best[i - 1][capacity] {
            result.push(xs[i - 1]);
            // we remove the weight of the object from the bag's remaining capacity
            capacity -= xs[i - 1].weight;
        }

        result
    })
}

#[cfg(not(test))]
fn main () {
    let xs = knap_01_dp(ITEMS, 400);

    // Print the items.
    // We have to reverse the order because we solved the problem backwards.
    for i in xs.iter().rev() {
        println!("Item: {}, Weight: {}, Value: {}", i.name, i.weight, i.value);
    }

    // Print the sum of weights.
    let weights = xs.iter().fold(0, |a, &b| a + b.weight);
    println!("Total Weight: {}", weights);

    // Print the sum of the values.
    let values = xs.iter().fold(0, |a, &b| a + b.value);
    println!("Total Value: {}", values);
}

#[test]
fn test_dp_results() {
    let dp_results = knap_01_dp(ITEMS, 400);
    let dp_weights = dp_results.iter().fold(0, |a, &b| a + b.weight);
    let dp_values = dp_results.iter().fold(0, |a, &b| a + b.value);
    assert_eq!(dp_weights, 396);
    assert_eq!(dp_values, 1030);
}
