use std::ops::RangeInclusive;
use std::str::FromStr;

use smallvec::{smallvec, SmallVec};

fn guess_at_line(s: &str, target_line: i32) -> usize {
    let mut ranges: SmallVec<[RangeInclusive<i32>; 32]> = smallvec![];
    let mut ranges_sec: SmallVec<[RangeInclusive<i32>; 32]> = smallvec![];
    let mut beacons_column_on_lines: SmallVec<[i32; 32]> = smallvec![];
    for line in s.lines() {
        let mut parts = line.split('=');
        parts.next().unwrap();
        let x = i32::from_str(parts.next().unwrap().split(',').next().unwrap()).unwrap();
        let y = i32::from_str(parts.next().unwrap().split(':').next().unwrap()).unwrap();
        let b_x = i32::from_str(parts.next().unwrap().split(',').next().unwrap()).unwrap();
        let b_y = i32::from_str(parts.next().unwrap()).unwrap();

        let distance = (x - b_x).abs() + (y - b_y).abs();

        let over_reach = (distance) - (y - target_line).abs();

        if b_y == target_line && !beacons_column_on_lines.contains(&b_x) {
            beacons_column_on_lines.push(b_x);
        }

        if over_reach < 0 {
            continue;
        }

        let candidate_range = (x - over_reach)..=(x + over_reach);
        ranges.push(candidate_range);
        let mut old_count = ranges.len();
        loop {
            for candidate_range in ranges.drain(..) {
                let mut replaced = false;
                for range in ranges_sec.iter_mut() {
                    if range.start() <= candidate_range.start() && range.end() >= candidate_range.start() ||
                        candidate_range.start() <= range.start() && candidate_range.end() >= range.start() {
                        let new_range = (*range.start().min(candidate_range.start()))..=*(range.end().max(candidate_range.end()));
                        let _ = std::mem::replace(range, new_range);
                        replaced = true;
                        break;
                    }
                }
                if !replaced {
                    ranges_sec.push(candidate_range);
                }
            }
            std::mem::swap(&mut ranges, &mut ranges_sec);
            if old_count == ranges.len() {
                break;
            }
            old_count = ranges.len();
        }
    }
    ranges.into_iter().flatten().count() - beacons_column_on_lines.len()
}

fn guess_at_line_p2(s: &str, p2_range: RangeInclusive<i32>) -> usize {
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

    for &(s1_y, s1_x, range1) in sonars.iter() {
        for &(s2_y, s2_x, range2) in sonars.iter() {
            if s2_x - s1_x + s2_y - s1_y == range1 + range2 + 2 {
                'vert: for it in 0..=(range1 + 1) {
                    let col = it + s1_x;
                    let row = range1 + 1 - it + s1_y;

                    for &(s_row, s_col, range) in sonars.iter() {
                        if range >= (s_row - row).abs() + (s_col - col).abs()
                            || !p2_range.contains(&row)
                            || !p2_range.contains(&col)
                        {
                            continue 'vert;
                        }
                    }

                    return col as usize * 4_000_000 + row as usize;
                }
            }

            if s2_x - s1_x + s1_y - s2_y == range1 + range2 + 2 {
                'vert: for it in 0..=(range1 + 1) {
                    let col = it + s1_x;
                    let row = -range1 - 1 + it + s1_y;

                    for &(s_row, s_col, range) in sonars.iter() {
                        if range >= (s_row - row).abs() + (s_col - col).abs()
                            || !p2_range.contains(&row)
                            || !p2_range.contains(&col)
                        {
                            continue 'vert;
                        }
                    }

                    return col as usize * 4_000_000 + row as usize;
                }
            }
        }
    }

    panic!()
}

#[allow(unused)]
pub fn _p1(s: &str, target_line: i32) -> usize {
    guess_at_line(s, target_line)
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j15.txt"), 2000000)
}

#[allow(unused)]
pub fn _p2(s: &str, search_space: RangeInclusive<i32>) -> usize {
    guess_at_line_p2(s, search_space)
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