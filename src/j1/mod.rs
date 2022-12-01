use std::str::FromStr;

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut tot = 0u32;
    let mut max = 0u32;
    for l in s.lines() {
        match u32::from_str(l) {
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
    max as usize
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j1.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let mut tot = 0u32;
    let mut max = [0u32; 3];

    for l in s.lines() {
        match u32::from_str(l) {
            Ok(num) => {
                tot += num;
            }
            Err(_) => {
                let mut last_idx = 0;

                for elt in max.iter() {
                    if *elt < tot {
                        last_idx += 1
                    }
                }
                for elt in max.as_mut_slice()[0..last_idx].iter_mut().rev() {
                    std::mem::swap(elt, &mut tot);
                }
                tot = 0u32
            }
        }
    }
    for elt in max.iter_mut() {
        if *elt < tot {
            std::mem::swap(elt, &mut tot);
        }
    }
    max.into_iter().sum::<u32>() as usize
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j1.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j1_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(24000, _p1(include_str!("j1_test.txt")));
        assert_eq!(68775, _p1(include_str!("j1.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(45000, _p2(include_str!("j1_test.txt")));
        assert_eq!(202585, _p2(include_str!("j1.txt")));
    }
}