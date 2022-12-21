use std::num::ParseIntError;
use std::str::FromStr;
use itertools::Itertools;
use smallvec::SmallVec;


enum RawNode<'a> {
    Element(isize),
    Plus(&'a str, &'a str),
    Minus(&'a str, &'a str),
    Times(&'a str, &'a str),
    Divide(&'a str, &'a str),
}

enum Node {
    Element(isize),
    Plus(usize, usize),
    Minus(usize, usize),
    Times(usize, usize),
    Divide(usize, usize),
}

fn compute_value(node: &Node, nodes: &SmallVec<[Node; 3000]>) -> isize {
    match node {
        Node::Element(elt) => {*elt}
        Node::Plus(first, second) => {
            compute_value(&nodes[*first], &nodes) +
            compute_value(&nodes[*second], &nodes)
        }
        Node::Minus(first, second) => {
            compute_value(&nodes[*first], &nodes) -
                compute_value(&nodes[*second], &nodes)
        }
        Node::Times(first, second) => {
            compute_value(&nodes[*first], &nodes) *
                compute_value(&nodes[*second], &nodes)
        }
        Node::Divide(first, second) => {
            compute_value(&nodes[*first], &nodes) /
                compute_value(&nodes[*second], &nodes)
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
            .filter(|s|!s.is_empty());
        let first = node_str.next().unwrap();
        match isize::from_str(first) {
            Ok(num) => {
                raw_nodes.push(RawNode::Element(num))
            }
            Err(_) => {
                let operator_char = node_str.next().unwrap().chars().next().unwrap();
                let second = (node_str.next().unwrap());
                raw_nodes.push(match operator_char {
                    '-' => {RawNode::Minus(first, second)}
                    '+' => {RawNode::Plus(first, second)}
                    '*' => {RawNode::Times(first, second)}
                    '/' => {RawNode::Divide(first, second)}
                    _ => panic!()
                })
            }
        }
    }

    for raw_node in raw_nodes {
        nodes.push(
            match raw_node {
                RawNode::Element(elt) => {Node::Element(elt)}
                RawNode::Plus(first, second) => {Node::Plus(
                    names.iter().find_position(|s|**s == first).unwrap().0,
                    names.iter().find_position(|s|**s == second).unwrap().0,
                )}
                RawNode::Minus(first, second) => {Node::Minus(
                    names.iter().find_position(|s|**s == first).unwrap().0,
                    names.iter().find_position(|s|**s == second).unwrap().0,
                )}
                RawNode::Times(first, second) => {Node::Times(
                    names.iter().find_position(|s|**s == first).unwrap().0,
                    names.iter().find_position(|s|**s == second).unwrap().0,
                )}
                RawNode::Divide(first, second) => {Node::Divide(
                    names.iter().find_position(|s|**s == first).unwrap().0,
                    names.iter().find_position(|s|**s == second).unwrap().0,
                )}
            }
        )
    }
    compute_value(&nodes[names.iter().find_position(|s|**s == "root").unwrap().0],
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
            .filter(|s|!s.is_empty());
        let first = node_str.next().unwrap();
        match isize::from_str(first) {
            Ok(num) => {
                raw_nodes.push(RawNode::Element(num))
            }
            Err(_) => {
                let operator_char = node_str.next().unwrap().chars().next().unwrap();
                let second = (node_str.next().unwrap());
                raw_nodes.push(match operator_char {
                    '-' => {RawNode::Minus(first, second)}
                    '+' => {RawNode::Plus(first, second)}
                    '*' => {RawNode::Times(first, second)}
                    '/' => {RawNode::Divide(first, second)}
                    _ => panic!()
                })
            }
        }
    }

    for raw_node in raw_nodes {
        nodes.push(
            match raw_node {
                RawNode::Element(elt) => {Node::Element(elt)}
                RawNode::Plus(first, second) => {Node::Plus(
                    names.iter().find_position(|s|**s == first).unwrap().0,
                    names.iter().find_position(|s|**s == second).unwrap().0,
                )}
                RawNode::Minus(first, second) => {Node::Minus(
                    names.iter().find_position(|s|**s == first).unwrap().0,
                    names.iter().find_position(|s|**s == second).unwrap().0,
                )}
                RawNode::Times(first, second) => {Node::Times(
                    names.iter().find_position(|s|**s == first).unwrap().0,
                    names.iter().find_position(|s|**s == second).unwrap().0,
                )}
                RawNode::Divide(first, second) => {Node::Divide(
                    names.iter().find_position(|s|**s == first).unwrap().0,
                    names.iter().find_position(|s|**s == second).unwrap().0,
                )}
            }
        )
    }
    compute_value(&nodes[names.iter().find_position(|s|**s == "root").unwrap().0],
                  &nodes) as usize
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
        assert_eq!(42, _p1(include_str!("j21.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(42, _p2(include_str!("j21_test.txt")));
        assert_eq!(42, _p2(include_str!("j21.txt")));
    }
}