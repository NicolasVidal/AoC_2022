use std::str::FromStr;

use smallvec::{SmallVec, smallvec};

const ESTIMATED_NODES_COUNT: usize = 512;
const ESTIMATED_CHILDS_COUNT: usize = 16;
const ESTIMATED_PATH_DEPTH_COUNT: usize = 16;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Element {
    Dir { name: &'static str, elements: SmallVec<[usize; ESTIMATED_CHILDS_COUNT]> },
    File { name: &'static str, size: usize },
}

#[inline(always)]
fn get_element_at_path(nodes: &SmallVec<[Element; ESTIMATED_NODES_COUNT]>, root: usize, path: &SmallVec<[&str; ESTIMATED_PATH_DEPTH_COUNT]>) -> usize {
    let mut current = root;
    for p in path {
        current = *(match &nodes[current] {
            Element::Dir { elements, .. } => {
                elements.iter().find(|elt| match nodes[**elt] {
                    Element::Dir { name, .. } => { name == *p }
                    _ => { false }
                }).unwrap()
            }
            Element::File { .. } => { panic!() }
        })
    }
    current
}

#[inline(always)]
fn compute_dir_of_size_at_most_than_max_size_and_return_total_dir_size(nodes: &SmallVec<[Element; ESTIMATED_NODES_COUNT]>, root: usize, max_size: usize, total: &mut usize) -> usize {
    let mut self_size = 0;
    match &nodes[root] {
        Element::Dir { elements, .. } => {
            for elt in elements.iter() {
                self_size += compute_dir_of_size_at_most_than_max_size_and_return_total_dir_size(nodes, *elt, max_size, total);
            }
            if self_size <= max_size {
                *total += self_size;
            }
        }
        Element::File { size, .. } => {
            self_size = *size;
        }
    }
    self_size
}

#[inline(always)]
fn find_smallest_dir(nodes: &SmallVec<[Element; ESTIMATED_NODES_COUNT]>, root: usize, min_size: usize, smallest_large_enough: &mut usize) -> usize {
    let mut self_size = 0;
    match &nodes[root] {
        Element::Dir { elements, .. } => {
            for elt in elements.iter() {
                self_size += find_smallest_dir(nodes, *elt, min_size, smallest_large_enough);
            }
            if self_size >= min_size && self_size < *smallest_large_enough {
                *smallest_large_enough = self_size;
            }
        }
        Element::File { size, .. } => {
            self_size = *size;
        }
    }
    self_size
}


fn build_file_tree(s: &'static str) -> SmallVec<[Element; ESTIMATED_NODES_COUNT]> {
    let mut nodes: SmallVec<[Element; ESTIMATED_NODES_COUNT]> = smallvec![];
    let root = Element::Dir { name: "/", elements: smallvec![] };
    nodes.push(root);
    {
        let mut current = 0usize;

        let mut path: SmallVec<[&str; ESTIMATED_PATH_DEPTH_COUNT]> = smallvec![];

        for line in s.lines().skip(1) {
            if line.starts_with("ls") {
                continue;
            }
            let mut split = line.split(' ');

            match split.next().unwrap() {
                "$" => {
                    match split.next().unwrap() {
                        "cd" => {
                            let name = split.next().unwrap();
                            match name {
                                ".." => {
                                    path.pop();
                                }
                                new_dir => {
                                    path.push(new_dir);
                                }
                            }
                            current = 0;
                            current = get_element_at_path(&nodes, current, &path);
                        }
                        "ls" => {}
                        _ => panic!()
                    }
                }
                "dir" => {
                    let elt = match &nodes[current] {
                        Element::Dir { elements, .. } => {
                            let elt_name = split.next().unwrap();
                            if !elements.iter().any(|elt| match &nodes[*elt] {
                                Element::Dir { name, .. } => { *name == elt_name }
                                _ => false
                            }) {
                                Some(Element::Dir { name: elt_name, elements: smallvec![] })
                            } else {
                                None
                            }
                        }
                        Element::File { .. } => { panic!(); }
                    };
                    if let Some(node) = elt {
                        let idx = nodes.len();
                        nodes.push(node);
                        match &mut nodes[current] {
                            Element::Dir { elements, .. } => { elements.push(idx); }
                            Element::File { .. } => { panic!() }
                        }
                    }
                }
                size => {
                    let elt = match &nodes[current] {
                        Element::Dir { elements, .. } => {
                            let elt_name = split.next().unwrap();
                            if !elements.iter().any(|elt| match &nodes[*elt] {
                                Element::File { name, .. } => { *name == elt_name }
                                _ => false
                            }) {
                                Some(Element::File { name: elt_name, size: usize::from_str(size).unwrap() })
                            } else {
                                None
                            }
                        }
                        Element::File { .. } => { panic!() }
                    };
                    if let Some(node) = elt {
                        let idx = nodes.len();
                        nodes.push(node);
                        match &mut nodes[current] {
                            Element::Dir { elements, .. } => { elements.push(idx); }
                            Element::File { .. } => { panic!() }
                        }
                    }
                }
            }
        }
    }
    nodes
}

#[allow(unused)]
pub fn _p1(s: &'static str) -> usize {
    let total_size = 0usize;

    let nodes = build_file_tree(s);
    let mut total_size = 0usize;
    compute_dir_of_size_at_most_than_max_size_and_return_total_dir_size(&nodes, 0, 100000, &mut total_size);
    total_size
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j7.txt"))
}

#[allow(unused)]
pub fn _p2(s: &'static str) -> usize {
    let nodes = build_file_tree(s);
    let mut total_size = 0usize;
    let total_used_space = compute_dir_of_size_at_most_than_max_size_and_return_total_dir_size(&nodes, 0, 100000, &mut total_size);
    let bytes_to_free = 30000000 - (70000000 - total_used_space);
    let mut smallest_large_enough = usize::MAX;
    find_smallest_dir(&nodes, 0, bytes_to_free, &mut smallest_large_enough);
    smallest_large_enough
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j7.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j7_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(95437, _p1(include_str!("j7_test.txt")));
        assert_eq!(1447046, _p1(include_str!("j7.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(24933642, _p2(include_str!("j7_test.txt")));
        assert_eq!(578710, _p2(include_str!("j7.txt")));
    }
}