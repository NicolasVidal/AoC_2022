
#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    for line in s.lines() {

    }
    42
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j8.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    for line in s.lines() {

    }
    42
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j8.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j8_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(42, _p1(include_str!("j8_test.txt")));
        assert_eq!(42, _p1(include_str!("j8.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(42, _p2(include_str!("j8_test.txt")));
        assert_eq!(42, _p2(include_str!("j8.txt")));
    }
}