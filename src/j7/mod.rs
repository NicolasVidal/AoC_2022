use std::str::FromStr;

use smallvec::SmallVec;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Element {
    Dir { name: &'static str, elements: Box<SmallVec<[Element; 16]>> },
    File { name: &'static str, size: usize },
}

#[inline(always)]
fn get_element_at_path<'a>(root: &'a mut Element, path: &Vec<&'static str>) -> &'a mut Element {
    let mut current = root;
    for p in path {
        current = match current {
            Element::Dir { elements, .. } => {
                elements.iter_mut().find(|elt| match elt {
                    Element::Dir { name, .. } => {name == p}
                    _ => {false}
                }).unwrap()
            }
            Element::File { .. } => { panic!() }
        }
    }
    current
}

#[inline(always)]
fn compute_dir_of_size_at_most_than_max_size_and_return_total_dir_size(root: &Element, max_size: usize, total: &mut usize) -> usize {
    let mut self_size = 0;
    match root {
        Element::Dir { elements, .. } => {
            for elt in elements.iter() {
                self_size += compute_dir_of_size_at_most_than_max_size_and_return_total_dir_size(elt, max_size, total);
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
fn find_smallest_dir(root: &Element, min_size: usize, smallest_large_enough: &mut usize) -> usize {
    let mut self_size = 0;
    match root {
        Element::Dir { elements, .. } => {
            for elt in elements.iter() {
                self_size += find_smallest_dir(elt, min_size, smallest_large_enough);
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


fn build_file_tree(s: &'static str) -> Element {
    let mut root = Element::Dir { name: "/", elements: Box::new(SmallVec::new()) };
    {
        let mut current = &mut root;

        let mut path = vec!();

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
                            current = &mut root;
                            current = get_element_at_path(current, &path);
                        }
                        "ls" => {}
                        _ => panic!()
                    }
                }
                "dir" => {
                    match current {
                        Element::Dir { elements, .. } => {
                            let elt_name = split.next().unwrap();
                            if !elements.iter().any(|elt| match elt {
                                Element::Dir { name, .. } => { *name == elt_name }
                                _ => false
                            }) {
                                elements.push(Element::Dir { name: elt_name, elements: Box::new(SmallVec::new()) });
                            }
                        }
                        Element::File { .. } => { panic!() }
                    }
                }
                size => {
                    match current {
                        Element::Dir { elements, .. } => {
                            let elt_name = split.next().unwrap();
                            if !elements.iter().any(|elt| match elt {
                                Element::File { name, .. } => { *name == elt_name }
                                _ => false
                            }) {
                                elements.push(Element::File { name: elt_name, size: usize::from_str(size).unwrap() });
                            }
                        }
                        Element::File { .. } => { panic!() }
                    }
                }
            }
        }
    }
    root
}

#[allow(unused)]
pub fn _p1(s: &'static str) -> usize {
    let total_size = 0usize;

    let root = build_file_tree(s);
    let mut total_size = 0usize;
    compute_dir_of_size_at_most_than_max_size_and_return_total_dir_size(&root, 100000, &mut total_size);
    total_size
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j7.txt"))
}

#[allow(unused)]
pub fn _p2(s: &'static str) -> usize {
    let root = build_file_tree(s);
    let mut total_size = 0usize;
    let total_used_space = compute_dir_of_size_at_most_than_max_size_and_return_total_dir_size(&root, 100000, &mut total_size);
    let bytes_to_free = 30000000 - (70000000 - total_used_space);
    let mut smallest_large_enough = usize::MAX;
    find_smallest_dir(&root, bytes_to_free, &mut smallest_large_enough);
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