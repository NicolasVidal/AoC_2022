use std::ops::Add;
use std::str::FromStr;

use itertools::Itertools;

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    42
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j2.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    42
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j2.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j2_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(24000, _p1(include_str!("j2_test.txt")));
        assert_eq!(68775, _p1(include_str!("j2.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(45000, _p2(include_str!("j2_test.txt")));
        assert_eq!(202585, _p2(include_str!("j2.txt")));
    }
}