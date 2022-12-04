use std::ops::RangeInclusive;
use std::str::FromStr;

#[inline(always)]
fn get_range(s: &str) -> RangeInclusive<u8> {
    let mut split = s.split('-');
    let split_start = u8::from_str(split.next().unwrap()).unwrap();
    let split_end = u8::from_str(split.next().unwrap()).unwrap();
    split_start..=split_end
}

#[inline(always)]
fn get_ranges(s: &str) -> (RangeInclusive<u8>, RangeInclusive<u8>) {
    let mut split = s.split(',');
    let pair1 = split.next().unwrap();
    let pair2 = split.next().unwrap();

    (get_range(pair1), get_range(pair2))
}

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut total = 0u16;
    for line in s.lines() {
        let (range1, range2) = get_ranges(line);

        if range1.start() <= range2.start() && range1.end() >= range2.end() ||
            range2.start() <= range1.start() && range2.end() >= range1.end() {
            total += 1
        }
    }
    total as usize
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j4.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let mut total = 0u16;
    for line in s.lines() {
        let (range1, range2) = get_ranges(line);

        if range1.into_iter().any(|elt| range2.contains(&elt)) {
            total += 1
        }
    }
    total as usize
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j4.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j4_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(2, _p1(include_str!("j4_test.txt")));
        assert_eq!(424, _p1(include_str!("j4.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(4, _p2(include_str!("j4_test.txt")));
        assert_eq!(804, _p2(include_str!("j4.txt")));
    }
}