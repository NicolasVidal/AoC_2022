use std::str::FromStr;

use itertools::Itertools;
use smallvec::{SmallVec, smallvec};

#[allow(unused)]
pub fn _p1(s: &str) -> isize {
    mix(s, 1, 1)
}

fn mix(s: &str, multiplier: isize, mix_times: usize) -> isize {
    let mut indexes: SmallVec<[usize; 5000]> = smallvec![];
    let mut results: SmallVec<[isize; 5000]> = smallvec![];
    for (idx, line) in s.lines().enumerate() {
        let num = isize::from_str(line).unwrap() * multiplier;
        indexes.push(idx);
        results.push(num);
    }

    let num_elements = results.len();
    for _ in 0..mix_times {
        for elt in 0..results.len() {
            let v = results[elt];
            let idx = indexes[elt];

            let mut new_position = (idx as isize + v) % (num_elements as isize);

            while new_position < 0 {
                new_position += results.len() as isize;
            }

            let mut inserted_position = (idx as isize + v) % (num_elements as isize - 1);

            while inserted_position < 0 {
                inserted_position += num_elements as isize - 1;
            }

            let mut other_order: SmallVec<[usize; 5000]> = (0..num_elements).filter(|idx| *idx != elt).collect();

            other_order.sort_unstable_by(|elt1, elt2| indexes[*elt1].cmp(&indexes[*elt2]));

            other_order.insert(inserted_position as usize, elt);

            for idx in 0..num_elements {
                indexes[other_order[(idx + inserted_position as usize) % num_elements]] = (new_position as usize + idx) % num_elements;
            }
        }
    }

    let zero_original_index = results.iter().find_position(|elt| **elt == 0).unwrap().0;
    let zero_index = indexes[zero_original_index];
    [1000usize, 2000, 3000].iter().map(|elt|
        results[indexes.iter().find_position(|idx| **idx == ((elt + zero_index) % results.len())).unwrap().0]).sum()
}

#[allow(unused)]
pub fn p1() -> isize {
    _p1(include_str!("j20.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> isize {
    mix(s, 811589153, 10)
}

#[allow(unused)]
pub fn p2() -> isize {
    _p2(include_str!("j20.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j20_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(3, _p1(include_str!("j20_test.txt")));
        assert_eq!(10707, _p1(include_str!("j20.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(1623178306, _p2(include_str!("j20_test.txt")));
        assert_eq!(2488332343098, _p2(include_str!("j20.txt")));
    }
}