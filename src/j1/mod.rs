use std::str::FromStr;

use itertools::Itertools;

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    s.lines()
        .fold((usize::MIN, 0usize),
              |(max, total), line|
                  usize::from_str(line)
                      .map(|value| (max.max(total), total + value))
                      .unwrap_or((max.max(total), 0usize)))
        .0
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j1.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    s.lines()
        .batching(|it|
            it.map_while(|line| usize::from_str(line).ok())
                .fold(None, |acc: Option<usize>, value| acc.map(|prev_acc| prev_acc + value)
                    .or(Some(value)))
        )
        .sorted()
        .rev()
        .take(3)
        .sum()
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j1.txt"))
}

#[cfg(test)]
mod j1_tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(24000, _p1(include_str!("j1_test.txt")));
        assert_eq!(68775, _p1(include_str!("j1.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(45000, _p2(include_str!("j1_test.txt")));
        assert_eq!(202585, _p2(include_str!("j1.txt")));
    }
}