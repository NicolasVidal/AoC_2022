
#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    for line in s.lines() {

    }
    42
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j14.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    for line in s.lines() {

    }
    42
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j14.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j14_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(42, _p1(include_str!("j14_test.txt")));
        assert_eq!(42, _p1(include_str!("j14.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(42, _p2(include_str!("j14_test.txt")));
        assert_eq!(42, _p2(include_str!("j14.txt")));
    }
}