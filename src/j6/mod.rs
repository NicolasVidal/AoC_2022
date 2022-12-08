fn get_start_position<const MIN_CHARS: usize>(s: &str) -> usize {
    let mut last_chars = ['$'; MIN_CHARS];
    'outer: for (i, c) in s.lines().next().unwrap().chars().enumerate() {
        last_chars[i % MIN_CHARS] = c;
        if i >= MIN_CHARS {
            for c1 in 0..(MIN_CHARS - 1) {
                for c2 in (c1 + 1)..MIN_CHARS {
                    if last_chars[c1] == last_chars[c2] {
                        continue 'outer;
                    }
                }
            }
            return i + 1;
        }
    }
    panic!("{}", format!("there were no unique {MIN_CHARS} characters following each other"));
}

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    get_start_position::<4>(s)
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j6.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    get_start_position::<14>(s)
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j6.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j6_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(7, _p1(include_str!("j6_test.txt")));
        assert_eq!(1929, _p1(include_str!("j6.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(19, _p2(include_str!("j6_test.txt")));
        assert_eq!(3298, _p2(include_str!("j6.txt")));
    }
}