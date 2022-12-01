use std::ops::Add;
use std::str::FromStr;

#[inline(always)]
fn update_maximums<T: Ord + Add<Output=T>, const N: usize>(maximums: &mut [T; N], mut new_value: T) {
    let mut last_idx = 0;
    for elt in maximums.iter() {
        if *elt < new_value {
            last_idx += 1
        }
    }
    for i in (0..last_idx).rev() {
        std::mem::swap(&mut maximums[i], &mut new_value);
    }
}

#[inline(always)]
fn compute_max_sums<T: Ord + Default + Copy + FromStr + Add<Output=T> + std::iter::Sum, const N: usize>(s: &str) -> T {
    let mut maximums = [T::default(); N];
    let mut total = T::default();

    for line in s.lines() {
        match T::from_str(line) {
            Ok(value) => {
                total = total + value;
            }
            Err(_) => {
                update_maximums(&mut maximums, total);
                total = T::default();
            }
        }
    }
    update_maximums(&mut maximums, total);
    maximums.into_iter().sum()
}

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    compute_max_sums::<u32, 1>(s) as usize
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j1.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    compute_max_sums::<u32, 3>(s) as usize
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