use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::str::FromStr;

use smallvec::{smallvec, SmallVec};

fn guess_at_line(s: &str, target_line: i32) -> Option<usize> {
    let mut set = HashSet::with_capacity(5127798);
    let mut forbidden_beacons_on_line = HashSet::with_capacity(1);
    for line in s.lines() {
        let mut parts = line.split('=');
        parts.next().unwrap();
        let x = i32::from_str(parts.next().unwrap().split(',').next().unwrap()).unwrap();
        let y = i32::from_str(parts.next().unwrap().split(':').next().unwrap()).unwrap();
        let b_x = i32::from_str(parts.next().unwrap().split(',').next().unwrap()).unwrap();
        let b_y = i32::from_str(parts.next().unwrap()).unwrap();

        let distance = (x - b_x).abs() + (y - b_y).abs();

        let over_reach = (distance) - (y - target_line).abs();

        if b_y == target_line {
            forbidden_beacons_on_line.insert(b_x);
        }

        for col in 0..=over_reach {
            set.insert(x + col);
            set.insert(x - col);
        }
    }

    Some(set.len() - set.intersection(&forbidden_beacons_on_line).count())
}

fn guess_at_line_p2(s: &str, p2_range: RangeInclusive<i32>) -> Option<usize> {
    let mut sonars: SmallVec<[(i32, i32, i32); 32]> = smallvec![];
    for line in s.lines() {
        let mut parts = line.split('=');
        parts.next().unwrap();
        let x = i32::from_str(parts.next().unwrap().split(',').next().unwrap()).unwrap();
        let y = i32::from_str(parts.next().unwrap().split(':').next().unwrap()).unwrap();
        let b_x = i32::from_str(parts.next().unwrap().split(',').next().unwrap()).unwrap();
        let b_y = i32::from_str(parts.next().unwrap()).unwrap();

        let range = (x - b_x).abs() + (y - b_y).abs();
        sonars.push((y, x, range));
    }

    for &(ss_row, ss_col, ss_range) in sonars.iter() {
        for rel_col in 0..=(ss_range + 1) {
            let rel_row = (ss_range + 1) - rel_col;
            let points = [
                (ss_row + rel_row, ss_col + rel_col),
                (ss_row - rel_row, ss_col - rel_col),
                (ss_row - rel_row, ss_col + rel_col),
                (ss_row + rel_row, ss_col - rel_col),
            ];
            'points: for (row, col) in points {
                for &(s_row, s_col, range) in sonars.iter() {
                    if range >= (s_row - row).abs() + (s_col - col).abs()
                        || !p2_range.contains(&row)
                        || !p2_range.contains(&col)
                    {
                        continue 'points;
                    }
                }
                return Some(col as usize * 4_000_000 + row as usize);
            }
        }
    }

    panic!("Hidden beacon not found, this should not happen !");
}

#[allow(unused)]
pub fn _p1(s: &str, target_line: i32) -> usize {
    guess_at_line(s, target_line).unwrap()
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j15.txt"), 2000000)
}

#[allow(unused)]
pub fn _p2(s: &str, search_space: RangeInclusive<i32>) -> usize {
    match guess_at_line_p2(s, search_space) {
        None => {}
        Some(v) => { return v; }
    }
    panic!()
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j15.txt"), 0..=4000000)
}

#[cfg(test)]
#[allow(unused)]
mod j15_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(26, _p1(include_str!("j15_test.txt"), 10));
        assert_eq!(5127797, _p1(include_str!("j15.txt"), 2000000));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(56000011, _p2(include_str!("j15_test.txt"), 0..=20));
        assert_eq!(12518502636475, _p2(include_str!("j15.txt"), 0..=4000000));
    }
}