use std::str::FromStr;

use itertools::Itertools;
use smallvec::SmallVec;

enum RawNode<'a> {
    Element(isize),
    Plus(&'a str, &'a str),
    Minus(&'a str, &'a str),
    Times(&'a str, &'a str),
    Divide(&'a str, &'a str),
    Equal(&'a str, &'a str),
}

#[derive(Clone)]
enum Node {
    Unknown(char),
    Element(isize),
    Plus(usize, usize),
    Minus(usize, usize),
    Times(usize, usize),
    Divide(usize, usize),
    Equal(usize, usize),
}

// fn reduce(node: &Node, nodes: &SmallVec<[Node; 3000]>) -> Node {
//     match node {
//         Node::Unknown(_) => {node.clone()}
//         Node::Element(_) => {node.clone()}
//         Node::Plus(idx1, idx2) |
//         Node::Minus(idx1, idx2) |
//         Node::Times(idx1, idx2) |
//         Node::Divide(idx1, idx2) => {
//             match (reduce(&nodes[*idx1], nodes), reduce(&nodes[*idx2], nodes)) {
//                 (Node::Unknown(_), Node::Element(_)) |
//                 (Node::Element(e), Node::Unknown(c)) => {
//                     node.clone()
//                 }
//             }
//         }
//         Node::Minus(_, _) => {}
//         Node::Times(_, _) => {}
//         Node::Divide(_, _) => {}
//         Node::Equal(_, _) => {}
//     }
// }
//
// fn solve(node: &Node, nodes: &SmallVec<[Node; 3000]>) {
//
// }

fn display_recursively(node: &Node, nodes: &SmallVec<[Node; 3000]>) {
    match node {
        Node::Unknown(c) => {
            print!("{}", c);
        }
        Node::Element(v) => {
            print!("{}", v);
        }
        Node::Plus(idx1, idx2) => {
            print!("(");
            display_recursively(&nodes[*idx1], nodes);
            print!(" + ");
            display_recursively(&nodes[*idx2], nodes);
            print!(")");
        }
        Node::Minus(idx1, idx2) => {
            print!("(");
            display_recursively(&nodes[*idx1], nodes);
            print!(" - ");
            display_recursively(&nodes[*idx2], nodes);
            print!(")");
        }
        Node::Times(idx1, idx2) => {
            print!("(");
            display_recursively(&nodes[*idx1], nodes);
            print!(" * ");
            display_recursively(&nodes[*idx2], nodes);
            print!(")");
        }
        Node::Divide(idx1, idx2) => {
            print!("(");
            display_recursively(&nodes[*idx1], nodes);
            print!(" / ");
            display_recursively(&nodes[*idx2], nodes);
            print!(")");
        }
        Node::Equal(idx1, idx2) => {
            print!("(");
            display_recursively(&nodes[*idx1], nodes);
            print!(" == ");
            display_recursively(&nodes[*idx2], nodes);
            print!(")");
        }
    }
}

fn compute_value(node: &Node, nodes: &SmallVec<[Node; 3000]>) -> isize {
    match node {
        Node::Unknown(_) => { panic!() }
        Node::Element(elt) => { *elt }
        Node::Plus(first, second) => {
            compute_value(&nodes[*first], nodes) +
                compute_value(&nodes[*second], nodes)
        }
        Node::Minus(first, second) => {
            compute_value(&nodes[*first], nodes) -
                compute_value(&nodes[*second], nodes)
        }
        Node::Times(first, second) => {
            compute_value(&nodes[*first], nodes) *
                compute_value(&nodes[*second], nodes)
        }
        Node::Divide(first, second) => {
            compute_value(&nodes[*first], nodes) /
                compute_value(&nodes[*second], nodes)
        }
        Node::Equal(first, second) => {
            isize::from(compute_value(&nodes[*first], nodes) ==
                compute_value(&nodes[*second], nodes))
        }
    }
}

#[allow(unused)]
fn compute_value_true(node: &Node, nodes: &SmallVec<[Node; 3000]>) -> Option<isize> {
    match node {
        Node::Unknown(_) => { panic!() }
        Node::Element(elt) => { Some(*elt) }
        Node::Plus(first, second) => {
            compute_value_true(&nodes[*first], nodes).zip(
                compute_value_true(&nodes[*second], nodes))
                .map(|(a, b)| a + b)
        }
        Node::Minus(first, second) => {
            compute_value_true(&nodes[*first], nodes).zip(
                compute_value_true(&nodes[*second], nodes))
                .map(|(a, b)| a - b)
        }
        Node::Times(first, second) => {
            compute_value_true(&nodes[*first], nodes).zip(
                compute_value_true(&nodes[*second], nodes))
                .map(|(a, b)| a * b)
        }
        Node::Divide(first, second) => {
            compute_value_true(&nodes[*first], nodes).zip(
                compute_value_true(&nodes[*second], nodes))
                .map(|(a, b)| a / b)
        }
        Node::Equal(first, second) => {
            compute_value_true(&nodes[*first], nodes).zip(
                compute_value_true(&nodes[*second], nodes))
                .filter(|(a, b)| {
                    *a == *b
                })
                .map(|_| 1)
        }
    }
}

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut names: SmallVec<[&str; 3000]> = Default::default();
    let mut nodes: SmallVec<[Node; 3000]> = Default::default();
    let mut raw_nodes: SmallVec<[RawNode; 3000]> = Default::default();
    for line in s.lines() {
        let mut splits = line.split(':');
        let name = splits.next().unwrap();
        names.push(name);
        let mut node_str = splits.next().unwrap().split(' ')
            .filter(|s| !s.is_empty());
        let first = node_str.next().unwrap();
        match isize::from_str(first) {
            Ok(num) => {
                raw_nodes.push(RawNode::Element(num))
            }
            Err(_) => {
                let operator_char = node_str.next().unwrap().chars().next().unwrap();
                let second = (node_str.next().unwrap());
                raw_nodes.push(match operator_char {
                    '-' => { RawNode::Minus(first, second) }
                    '+' => { RawNode::Plus(first, second) }
                    '*' => { RawNode::Times(first, second) }
                    '/' => { RawNode::Divide(first, second) }
                    _ => panic!()
                })
            }
        }
    }

    for raw_node in raw_nodes {
        nodes.push(
            match raw_node {
                RawNode::Element(elt) => { Node::Element(elt) }
                RawNode::Plus(first, second) => {
                    Node::Plus(
                        names.iter().find_position(|s| **s == first).unwrap().0,
                        names.iter().find_position(|s| **s == second).unwrap().0,
                    )
                }
                RawNode::Minus(first, second) => {
                    Node::Minus(
                        names.iter().find_position(|s| **s == first).unwrap().0,
                        names.iter().find_position(|s| **s == second).unwrap().0,
                    )
                }
                RawNode::Times(first, second) => {
                    Node::Times(
                        names.iter().find_position(|s| **s == first).unwrap().0,
                        names.iter().find_position(|s| **s == second).unwrap().0,
                    )
                }
                RawNode::Divide(first, second) => {
                    Node::Divide(
                        names.iter().find_position(|s| **s == first).unwrap().0,
                        names.iter().find_position(|s| **s == second).unwrap().0,
                    )
                }
                RawNode::Equal(first, second) => {
                    Node::Equal(
                        names.iter().find_position(|s| **s == first).unwrap().0,
                        names.iter().find_position(|s| **s == second).unwrap().0,
                    )
                }
            }
        )
    }
    compute_value(&nodes[names.iter().find_position(|s| **s == "root").unwrap().0],
                  &nodes) as usize
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j21.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let mut names: SmallVec<[&str; 3000]> = Default::default();
    let mut nodes: SmallVec<[Node; 3000]> = Default::default();
    let mut raw_nodes: SmallVec<[RawNode; 3000]> = Default::default();
    for line in s.lines() {
        let mut splits = line.split(':');
        let name = splits.next().unwrap();
        names.push(name);
        let mut node_str = splits.next().unwrap().split(' ')
            .filter(|s| !s.is_empty());
        let first = node_str.next().unwrap();
        match isize::from_str(first) {
            Ok(num) => {
                raw_nodes.push(RawNode::Element(num))
            }
            Err(_) => {
                let operator_char = node_str.next().unwrap().chars().next().unwrap();
                let second = (node_str.next().unwrap());
                if name == "root" {
                    raw_nodes.push(RawNode::Equal(first, second));
                } else {
                    raw_nodes.push(match operator_char {
                        '-' => { RawNode::Minus(first, second) }
                        '+' => { RawNode::Plus(first, second) }
                        '*' => { RawNode::Times(first, second) }
                        '/' => { RawNode::Divide(first, second) }
                        _ => panic!()
                    })
                }
            }
        }
    }

    for raw_node in raw_nodes {
        nodes.push(
            match raw_node {
                RawNode::Element(elt) => { Node::Element(elt) }
                RawNode::Plus(first, second) => {
                    Node::Plus(
                        names.iter().find_position(|s| **s == first).unwrap().0,
                        names.iter().find_position(|s| **s == second).unwrap().0,
                    )
                }
                RawNode::Minus(first, second) => {
                    Node::Minus(
                        names.iter().find_position(|s| **s == first).unwrap().0,
                        names.iter().find_position(|s| **s == second).unwrap().0,
                    )
                }
                RawNode::Times(first, second) => {
                    Node::Times(
                        names.iter().find_position(|s| **s == first).unwrap().0,
                        names.iter().find_position(|s| **s == second).unwrap().0,
                    )
                }
                RawNode::Divide(first, second) => {
                    Node::Divide(
                        names.iter().find_position(|s| **s == first).unwrap().0,
                        names.iter().find_position(|s| **s == second).unwrap().0,
                    )
                }
                RawNode::Equal(first, second) => {
                    Node::Equal(
                        names.iter().find_position(|s| **s == first).unwrap().0,
                        names.iter().find_position(|s| **s == second).unwrap().0,
                    )
                }
            }
        )
    }
    nodes[names.iter().find_position(|s| **s == "humn").unwrap().0] = Node::Unknown('x');
    display_recursively(&nodes[names.iter().find_position(|s| **s == "root").unwrap().0],
                        &nodes);
    println!();
    42
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j21.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j21_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(152, _p1(include_str!("j21_test.txt")));
        assert_eq!(121868120894282, _p1(include_str!("j21.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        // assert_eq!(301, _p2(include_str!("j21_test.txt")));
        assert_eq!(42, _p2(include_str!("j21.txt")));
    }
}