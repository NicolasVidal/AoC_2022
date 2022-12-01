use std::array::IntoIter;
use std::ops::Add;
use std::str::FromStr;

use itertools::Itertools;

struct Maximums<T: Ord + Default + Copy, const N: usize> {
    bests: [T; N],
}

impl<T: Ord + Default + Copy, const N: usize> Default for Maximums<T, N> {
    fn default() -> Self {
        Maximums::<T, N> {
            bests: [T::default(); N],
        }
    }
}

impl<T: Ord + Default + Copy, const N: usize> IntoIterator for Maximums<T, N> {
    type Item = T;
    type IntoIter = IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.bests.into_iter()
    }
}

impl<T: Ord + Default + Copy, const N: usize> Maximums<T, N> {
    fn update(&mut self, mut new_value: T) {
        let mut last_idx = 0;
        for elt in self.bests.iter() {
            if *elt < new_value {
                last_idx += 1
            }
        }
        for elt in self.bests.as_mut_slice()[0..last_idx].iter_mut().rev() {
            std::mem::swap(elt, &mut new_value);
        }
    }
    fn update_self(mut self, new_value: T) -> Self {
        self.update(new_value);
        self
    }
}

fn compute_max_sums<T: Ord + Default + Copy + FromStr + Add<Output=T> + std::iter::Sum, const N: usize>(s: &str) -> T {
    s.lines()
        .batching(|it|
            it.map_while(|line| T::from_str(line).ok())
                .fold(None, |acc: Option<T>, value| acc.map(|prev_acc| prev_acc + value)
                    .or(Some(value)))
        )
        .fold(Maximums::<T, N>::default(), |acc, new_value| acc.update_self(new_value))
        .into_iter()
        .sum()
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