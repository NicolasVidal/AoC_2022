use itertools::Itertools;

#[inline(always)]
fn get_letter_score(single: char) -> u8 {
    if ('a'..='z').contains(&single) {
        single as u8 - 'a' as u8 + 1
    } else {
        single as u8 - 'A' as u8 + 27
    }
}

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut total = 0u16;
    for line in s.lines() {
        let (left, right) = line.split_at(line.len() / 2);

        let single = left.chars().into_iter()
            .filter(|c| right.chars().contains(c)).next().unwrap();

        total += get_letter_score(single) as u16;
    }
    total as usize
}


#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j3.txt"))
}


#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let mut total = 0u16;
    for mut group in &s.lines().chunks(3) {
        let first = group.next().unwrap();
        let second = group.next().unwrap();
        let third = group.next().unwrap();

        let single = first.chars().into_iter()
            .filter(|c| second.chars().contains(c) && third.chars().contains(c))
            .next().unwrap();

        total += get_letter_score(single) as u16;
    }
    total as usize
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j3.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j3_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(157, _p1(include_str!("j3_test.txt")));
        assert_eq!(8109, _p1(include_str!("j3.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(70, _p2(include_str!("j3_test.txt")));
        assert_eq!(2738, _p2(include_str!("j3.txt")));
    }
}