#[inline(always)]
pub fn name_to_u16(name: char) -> u16 {
    match name {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'X' => 0,
        'Y' => 1,
        'Z' => 2,
        _ => panic!(),
    }
}

#[inline(always)]
pub fn predicted_shape(prev_input: u16, name: char) -> u16 {
    match name {
        'X' => (prev_input + 2) % 3,
        'Y' => prev_input,
        'Z' => (prev_input + 1) % 3,
        _ => panic!("{}", name),
    }
}

#[inline(always)]
fn compute_score(first: u16, second: u16) -> u16 {
    let winner = if first == ((second + 1) % 3) {
        0u16
    } else if second == ((first + 1) % 3) {
        6u16
    } else {
        3u16
    };
    second + 1 + winner
}

#[inline(always)]
fn get_chars(line: &str) -> (char, char) {
    let mut chars = line.chars();
    let a = chars.next().unwrap();
    chars.next();
    (a, chars.next().unwrap())
}

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut total = 0u16;
    for line in s.lines() {
        let (a, b) = get_chars(line);
        let first = name_to_u16(a);
        let second = name_to_u16(b);

        total += compute_score(first, second);
    }
    total as usize
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j2.txt"))
}


#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let mut total = 0u16;
    for line in s.lines() {
        let (a, b) = get_chars(line);
        let first = name_to_u16(a);
        let second = predicted_shape(first, b);

        total += compute_score(first, second);
    }
    total as usize
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
        assert_eq!(15, _p1(include_str!("j2_test.txt")));
        assert_eq!(14069, _p1(include_str!("j2.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(12, _p2(include_str!("j2_test.txt")));
        assert_eq!(12411, _p2(include_str!("j2.txt")));
    }
}