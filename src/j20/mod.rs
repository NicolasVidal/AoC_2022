use std::str::FromStr;

use itertools::Itertools;
use smallvec::{SmallVec, smallvec};

#[allow(unused)]
pub fn _p1(s: &str) -> isize {
    // let mut initial_order: SmallVec<[isize; 5000]> = smallvec![];
    let mut indexes: SmallVec<[usize; 5000]> = smallvec![];
    let mut indexes_cache: SmallVec<[usize; 5000]> = smallvec![];
    let mut results: SmallVec<[isize; 5000]> = smallvec![];
    for (idx, line) in s.lines().enumerate() {
        let num = isize::from_str(line).unwrap();
        indexes.push(idx);
        results.push(num);
    }

    let find_elt_at_pos_func = |pos: usize, results: &SmallVec<[isize; 5000]>,
                                indexes: &SmallVec<[usize; 5000]>| {
        results[indexes.iter().find_position(|idx| pos == **idx).unwrap().0]
    };

    let num_elements = results.len();
    // dbg!(results.iter().zip(indexes.iter()).sorted_by_key(|(r, i)| **i).map(|(r, _)| *r).join(", "));
    for elt in 0..results.len() {
        let idx = indexes[elt];
        let v = results[elt];

        let movement = if v > 0 { 1isize } else { -1isize };
        for _ in 0..v.abs() {
            let mut next_pos = (indexes[elt] as isize + movement) % num_elements as isize;
            if next_pos < 0 {
                next_pos = num_elements as isize + next_pos;
            }
            let next_idx = indexes.iter().find_position(|idx|**idx == next_pos as usize).unwrap().0;
            indexes.swap(next_idx, elt);
            // std::mem::swap(&mut indexes[next_pos as usize], &mut indexes[elt])
        }
        //
        // let mut new_position = (idx as isize + v) % (results.len() as isize);
        // dbg!(new_position);
        // while new_position < 0 {
        //     new_position = results.len() as isize + new_position;
        // }
        // let new_position = new_position as usize;
        // println!("Want to move {} from {} to {}", v, idx, new_position);
        // for other_idx in indexes.drain(..) {
        //     if idx == new_position {
        //         indexes_cache.push(other_idx);
        //         continue;
        //     }
        //     if idx == other_idx {
        //         indexes_cache.push(new_position);
        //         continue;
        //     }
        //     if v > 0 && new_position < idx && {
        //         indexes_cache.push((other_idx + 1) % results.len());
        //     }
        //     if ((idx + 1)..=new_position).contains(&other_idx) ||
        //         ((new_position)..idx).contains(&other_idx) {
        //         if new_position > idx {
        //             indexes_cache.push((other_idx - 1) % results.len());
        //         } else if new_position < idx {
        //             indexes_cache.push((other_idx + 1) % results.len());
        //         }
        //         continue;
        //     }
        //     indexes_cache.push(other_idx);
        // }
        // std::mem::swap(&mut indexes, &mut indexes_cache);
        // dbg!(indexes.iter().join(", "));
        // if new_position != idx {
        //     println!("{} moves from {} to {} between {} and {}:", v, idx, new_position,
        //              if new_position == 0 { "nothing".to_string() } else { find_elt_at_pos_func(new_position - 1, &results, &indexes).to_string() },
        //              if new_position == results.len() - 1 { "nothing".to_string() } else { find_elt_at_pos_func(new_position + 1, &results, &indexes).to_string() });
        // } else {
        //     println!("{} doesn't move", v);
        // }
        // println!("{}", results.iter().zip(indexes.iter()).sorted_by_key(|(r, i)| **i).map(|(r, _)| *r).join(", "));
    }

    // dbg!(&results);
    // dbg!(&indexes);
    let zero_original_index = (results.iter().find_position(|elt| **elt == 0).unwrap().0);
    let zero_index = indexes[zero_original_index];
    [1000usize, 2000, 3000].iter().map(|elt|
        results[indexes.iter().find_position(|idx| **idx == (((elt + zero_index) % results.len()))).unwrap().0]).sum()
}

#[allow(unused)]
pub fn p1() -> isize {
    _p1(include_str!("j20.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    for line in s.lines() {}
    42
}

#[allow(unused)]
pub fn p2() -> usize {
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
        assert_eq!(42, _p1(include_str!("j20.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(42, _p2(include_str!("j20_test.txt")));
        assert_eq!(42, _p2(include_str!("j20.txt")));
    }
}