use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::str::FromStr;

fn guess_at_line(s: &str, target_line: i32, is_p2: bool, p2_range: RangeInclusive<i32>) -> Option<usize> {
    let mut set = HashSet::with_capacity(*p2_range.end() as usize + 1);
    let mut forbidden_beacons_on_line = HashSet::with_capacity(1000);
    for line in s.lines() {
        let mut parts = line.split('=');
        parts.next().unwrap();
        let x = i32::from_str(parts.next().unwrap().split(',').next().unwrap()).unwrap();
        let y = i32::from_str(parts.next().unwrap().split(':').next().unwrap()).unwrap();
        let b_x = i32::from_str(parts.next().unwrap().split(',').next().unwrap()).unwrap();
        let b_y = i32::from_str(parts.next().unwrap()).unwrap();

        let distance = (x - b_x).abs() + (y - b_y).abs();

        let over_reach = (distance) - (y - target_line).abs();

        if b_y == target_line && (!is_p2 || p2_range.contains(&b_x)) {
            forbidden_beacons_on_line.insert(b_x);
        }

        for col in 0..=over_reach {
            if !is_p2 || p2_range.contains(&(x + col)) {
                set.insert(x + col);
            }
            if !is_p2 || p2_range.contains(&(x - col)) {
                set.insert(x - col);
            }
        }
    }

    if is_p2 {
        let max_items = *p2_range.clone().end() as usize + 1;
        if set.len() < max_items {
            Some((p2_range.clone().find(|elt| !set.contains(elt)).unwrap() * 4000000 + target_line) as usize)
        } else {
            None
        }
    } else {
        Some(set.len() - set.intersection(&forbidden_beacons_on_line).count())
    }
}

fn guess_at_line_p2(s: &str, p2_range: RangeInclusive<i32>) -> Option<usize> {
    let max_items = *p2_range.clone().end() as usize + 1;
    let mut set = Vec::with_capacity(max_items);
    // let mut beacons = vec![];
    let mut sonars = vec![];
    for _ in 0..max_items {
        set.push(false);
    }
    for line in s.lines() {
        let mut parts = line.split('=');
        parts.next().unwrap();
        let x = i32::from_str(parts.next().unwrap().split(',').next().unwrap()).unwrap();
        let y = i32::from_str(parts.next().unwrap().split(':').next().unwrap()).unwrap();
        let b_x = i32::from_str(parts.next().unwrap().split(',').next().unwrap()).unwrap();
        let b_y = i32::from_str(parts.next().unwrap()).unwrap();

        let range = (x - b_x).abs() + (y - b_y).abs();
        sonars.push((y, x, range));
        // beacons.push((b_y, b_x));
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

    // let mut beacon_pairs = vec![];

    // 'beacon: for beacon in beacons.iter() {
    //     let closest_beacon = beacons.iter()
    //         .filter(|b| **b != *beacon)
    //         .min_by_key(|b| (b.0 - beacon.0).abs() + (b.1 - beacon.1).abs()).unwrap();
    //
    //     let row_range = closest_beacon.0.min(beacon.0)..=closest_beacon.0.max(beacon.0);
    //     let col_range = closest_beacon.1.min(beacon.1)..=closest_beacon.1.max(beacon.1);
    //
    //     for sonar in sonars.iter() {
    //         if row_range.contains(&sonar.0) &&
    //             col_range.contains(&sonar.1) {
    //             continue 'beacon;
    //         }
    //     }
    //     beacon_pairs.push((row_range, col_range));
    // }

    // dbg!(beacon_pairs.len());
    // dbg!(&beacon_pairs);

    // for (row_range, col_range) in beacon_pairs {
    //     for row in row_range {
    //         'col_loop: for col in col_range.clone() {
    //             for &(s_row, s_col, range) in sonars.iter() {
    //                 if range <= (s_row - row).abs() + (s_col - col).abs() {
    //                     continue 'col_loop;
    //                 }
    //             }
    //             return Some(col as usize * 4_000_000 + row as usize);
    //         }
    //     }
    // }
    //
    // 'border_loop: for row in p2_range.clone() {
    //     for &(s_row, s_col, range) in sonars.iter() {
    //         if range <= (s_row - row).abs() + (s_col - 0).abs() {
    //             continue 'border_loop;
    //         }
    //     }
    //     return Some(0 as usize * 4_000_000 + row as usize);
    // }
    // 'border_loop: for row in p2_range.clone() {
    //     for &(s_row, s_col, range) in sonars.iter() {
    //         if range <= (s_row - row).abs() + (s_col - *p2_range.clone().end()).abs() {
    //             continue 'border_loop;
    //         }
    //     }
    //     return Some(*p2_range.clone().end() as usize * 4_000_000 + row as usize);
    // }
    // 'border_loop: for col in p2_range.clone() {
    //     for &(s_row, s_col, range) in sonars.iter() {
    //         if range <= (s_row - 0).abs() + (s_col - col).abs() {
    //             continue 'border_loop;
    //         }
    //     }
    //     return Some(col as usize * 4_000_000 + 0 as usize);
    // }
    // 'border_loop: for col in p2_range.clone() {
    //     for &(s_row, s_col, range) in sonars.iter() {
    //         if range <= (s_row - *p2_range.clone().end()).abs() + (s_col - col).abs() {
    //             continue 'border_loop;
    //         }
    //     }
    //     return Some(col as usize * 4_000_000 + *p2_range.clone().end() as usize);
    // }

    panic!();
}

#[allow(unused)]
pub fn _p1(s: &str, target_line: i32) -> usize {
    guess_at_line(s, target_line, false, 0..=0).unwrap()
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