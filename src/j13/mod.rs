use std::cmp::Ordering;
use std::iter::Peekable;
use std::str::Chars;

#[inline(always)]
fn is_digit(c: &char) -> bool {
    *c >= '0' && *c <= '9'
}

#[inline(always)]
fn are_digits(c1: &char, c2: &char) -> bool {
    is_digit(c1) && is_digit(c2)
}

#[inline(always)]
fn get_first_digit(chars: &mut Peekable<Chars>) -> u8 {
    let mut num = 0;
    while let Some(c) = chars.peek() {
        if !is_digit(c) {
            break;
        }
        num = 10 * num + (*c as u8 - b'0');
        chars.next().unwrap();
    }

    num
}

#[inline(always)]
fn compare_digits(first_chars: &mut Peekable<Chars>, second_chars: &mut Peekable<Chars>) -> Ordering {
    get_first_digit(first_chars).cmp(&get_first_digit(second_chars))
}

fn parse(first_chars: &mut Peekable<Chars>, second_chars: &mut Peekable<Chars>) -> Ordering {
    let mut first_left_to_match = 0;
    let mut second_left_to_match = 0;
    loop {
        match (first_chars.peek(), second_chars.peek()) {
            (None, None) => { panic!() }
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            (Some(v1), Some(v2)) => {
                match (v1, v2) {
                    (',', c) if !is_digit(c) && *c != ']' && second_left_to_match > first_left_to_match => return Ordering::Greater,
                    (c, ',') if !is_digit(c) && *c != ']' && first_left_to_match > second_left_to_match => return Ordering::Less,
                    (v1, v2) if v1 == v2 => {
                        first_chars.next().unwrap();
                        second_chars.next().unwrap();
                        continue;
                    }
                    (']', _) if second_left_to_match > first_left_to_match => {
                        first_chars.next().unwrap();
                        second_left_to_match -= 1;
                    }
                    (_, ']') if first_left_to_match > second_left_to_match => {
                        second_chars.next().unwrap();
                        first_left_to_match -= 1;
                    }
                    (v1, v2) if are_digits(v1, v2) => match compare_digits(first_chars, second_chars) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => continue,
                        Ordering::Greater => return Ordering::Greater,
                    },
                    ('[', ']') => return Ordering::Greater,
                    (']', '[') => return Ordering::Less,
                    (',', ']') => return Ordering::Greater,
                    (']', ',') => return Ordering::Less,
                    (c, ']') if is_digit(c) => return Ordering::Greater,
                    (']', c) if is_digit(c) => return Ordering::Less,
                    (c, '[') if is_digit(c) => {
                        second_chars.next().unwrap();
                        first_left_to_match += 1;
                    }
                    ('[', c) if is_digit(c) => {
                        first_chars.next().unwrap();
                        second_left_to_match += 1;
                    }
                    (',', c) if is_digit(c) => return Ordering::Less,
                    (c, ',') if is_digit(c) => return Ordering::Greater,
                    (a, b) => {
                        dbg!(a, b);
                        panic!()
                    }
                }
            }
        }
    }
}


#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut lines = s.lines();
    let mut total = 0;
    let mut pair_index = 0;

    loop {
        pair_index += 1;
        let first = lines.next().unwrap();
        let second = lines.next().unwrap();

        let mut first = first.chars().peekable();
        let mut second = second.chars().peekable();

        let ordering = parse(&mut first, &mut second);
        match ordering {
            Ordering::Less => { total += pair_index }
            Ordering::Equal => {}
            Ordering::Greater => {}
        }

        match lines.next() {
            None => { break; }
            Some(_) => {}
        }
    }
    total
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j13.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let mut lines = s.lines();
    // let mut packets = vec!();

    let mut total_under_2 = 0;
    let mut total_under_6 = 0;

    let two_str = "[[2]]";
    let six_str = "[[6]]";

    loop {
        let first_str = lines.next().unwrap();
        let second_str = lines.next().unwrap();

        let mut first = first_str.chars().peekable();
        let mut two = two_str.chars().peekable();

        match parse(&mut first, &mut two) {
            Ordering::Less => { total_under_2 += 1 }
            Ordering::Equal => {}
            Ordering::Greater => {}
        }

        let mut first = first_str.chars().peekable();
        let mut six = six_str.chars().peekable();

        match parse(&mut first, &mut six) {
            Ordering::Less => { total_under_6 += 1 }
            Ordering::Equal => {}
            Ordering::Greater => {}
        }

        let mut second = second_str.chars().peekable();
        let mut two = two_str.chars().peekable();

        match parse(&mut second, &mut two) {
            Ordering::Less => { total_under_2 += 1 }
            Ordering::Equal => {}
            Ordering::Greater => {}
        }

        let mut second = second_str.chars().peekable();
        let mut six = six_str.chars().peekable();

        match parse(&mut second, &mut six) {
            Ordering::Less => { total_under_6 += 1 }
            Ordering::Equal => {}
            Ordering::Greater => {}
        }

        match lines.next() {
            None => { break; }
            Some(_) => {}
        }
    }

    (total_under_2 + 1) * (total_under_6 + 1 + 1)
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j13.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j13_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(13, _p1(include_str!("j13_test.txt")));
        assert_eq!(6369, _p1(include_str!("j13.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(140, _p2(include_str!("j13_test.txt")));
        assert_eq!(25800, _p2(include_str!("j13.txt")));
    }
}