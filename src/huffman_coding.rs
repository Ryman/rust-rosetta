// Implement data structures for a Huffman encoding tree:
// http://rosettacode.org/wiki/Huffman_coding

use std::collections::HashMap;
use std::collections::priority_queue::PriorityQueue;

#[cfg(not(test))]
fn main() {
    let to_encode = "this is an example for huffman encoding";
    let tree = HuffmanTree::new(to_encode);
    let table = tree.encoding_table();

    for (&ch, encoding) in table.iter() {
        println!("'{}': {}", ch, encoding);
    }
}

pub struct HuffmanTree {
    head: Node
}

// Each Node has a weight, representing the sum of the frequencies for all its
// children. It is either a Leaf (containing a character), or a Tree
// (containing two children)
#[deriving(Show)]
struct Node {
    weight: int,
    item: NodeData,
}

#[deriving(Show, PartialEq)]
enum NodeData {
    Tree(Box<Children>),
    Leaf(char),
}

#[deriving(Show, PartialEq)]
struct Children {
    left: Node,
    right: Node,
}

impl HuffmanTree {
    /// Takes a non-empty string (function will fail if string is empty) and
    /// computes the Huffman encoding tree for that string.
    pub fn new(input: &str) -> HuffmanTree {
        // Loop through all the characters in that string, adding them to a
        // HashMap of character to frequency.
        let mut freq = HashMap::new();
        for ch in input.chars() {
            freq.insert_or_update_with(ch, 1, |_, v| *v += 1);
        }

        // For each (character, frequency) pair in the HashMap, add a Leaf to a
        // PriorityQueue
        let mut queue = PriorityQueue::new();
        for (ch, freq) in freq.move_iter() {
            let new_node = Node {
                weight: freq,
                item: Leaf(ch),
            };

            queue.push(new_node);
        }

        // Pop two items with the least weight from the queue, combine them into
        // a tree as children. The parent node's weight is the sum of the
        // children's weight. Continue until one item is left on the queue, and
        // return that item.
        while queue.len() > 1 {
            match (queue.pop(), queue.pop()) {
                (Some(item1), Some(item2)) => {
                    let new_node = Node {
                        weight: item1.weight + item2.weight,
                        item: Tree(box Children {
                                            left: item1,
                                            right: item2,
                                        }),
                    };

                    queue.push(new_node);
                }
                _ => unreachable!()
            }
        }

        HuffmanTree { head: queue.pop().unwrap() }
    }

    // Traverse the Huffman Tree and build a table with an encoding for each
    // character.
    pub fn encoding_table(&self) -> HashMap<char, String> {
        fn inner(node: &Node, mut table: HashMap<char, String>, s: String)
                    -> HashMap<char, String> {
            match node.item {
                Tree(ref data) => {
                    table = inner(&data.left, table, s.clone().append("0"));
                    inner(&data.right, table, s.append("1"))
                }
                Leaf(ch) => {
                    table.insert(ch, s);
                    table
                }
            }
        }

        inner(&self.head, HashMap::new(), String::new())
    }
}

// Implementing comparison traits (Ord and all its dependencies) such that
// the Node with the greatest weight is the smallest in a comparison. Basically
// reversing all the comparison operators.
impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Node {}
impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.weight == other.weight
    }
}

// Attempts to construct a tree, and test that the construction is successful
//    7
//   ----
//  /    \
// 4:'4'  3
//      -----
//     /     \
//    2:'2'  1:'1'
#[test]
fn test_tree_construction() {
    let tree = HuffmanTree::new("4444221").head;
    assert_eq!(tree.weight, 7);

    let Children { ref left, ref right } = match tree.item {
        Tree(box data) => data,
        _ => fail!("Tree Missing Children!"),
    };

    assert_eq!(right.item, Leaf('4'));
    assert_eq!(right.weight, 4);
    assert_eq!(left.weight, 3);

    match left.item {
        Tree(ref data) => {
            match (&data.left, &data.right) {
                (&Node { weight: 1, item: Leaf('1') },
                    &Node { weight: 2, item: Leaf('2') }) => {}
                (&Node { weight: 2, item: Leaf('2') },
                    &Node { weight: 1, item: Leaf('1') }) => {}
                _ => fail!("Incorrect Leaf Nodes")
            }
        },
        Leaf(_) => fail!("Tree Missing Children!"),
    }
}

// Constructs a table:
//  '4': 1
//  '2': 01 OR 00
//  '1': 00    01
// And tests that the table was correctly constructed
#[test]
fn test_table_construction() {
    let table = HuffmanTree::new("4444221").encoding_table();

    let one  = table['1'].as_slice();
    let two  = table['2'].as_slice();
    assert_eq!("1", table['4'].as_slice());
    assert!((one == "01" && two == "00") ||
            (one == "00" && two == "01"));
}
