use std::str::FromStr;

pub fn _p1(s: &str) -> usize {
    let mut tot = 0;
    let mut max = 0;
    for l in s.lines() {
        match usize::from_str(l) {
            Ok(num) => {
                tot += num;
            }
            Err(_) => {
                if tot > max {
                    max = tot
                }
                tot = 0;
            }
        }
    }
    if tot > max {
        max = tot
    }
    max
}

pub fn p1() -> usize {
    _p1(include_str!("j1.txt"))
}

pub fn _p2(s: &str) -> usize {
    let mut tot = 0;
    let mut max = vec!();

    for l in s.lines() {
        match usize::from_str(l) {
            Ok(num) => {
                tot += num;
            }
            Err(_) => {
                max.push(tot);
                tot = 0
            }
        }
    }
    max.push(tot);
    max.sort();
    max.reverse();
    max[..3].into_iter().sum()
}

pub fn p2() -> usize {
    _p2(include_str!("j1.txt"))
}

#[cfg(test)]
mod j1_tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(24000, _p1(include_str!("j1_test.txt")));
        assert_eq!(24000, _p1(include_str!("j1.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(45000, _p2(include_str!("j1_test.txt")));
        assert_eq!(202585, _p2(include_str!("j1.txt")));
    }
}