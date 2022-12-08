use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

use smallvec::{smallvec, SmallVec};

pub struct Chars(SmallVec<[char; 128]>);

impl Display for Chars {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for elt in self.0.iter() {
            f.write_char(*elt).unwrap();
        }
        Ok(())
    }
}

fn get_stacks(s: &str, keep_order: bool) -> Chars {
    let mut stack_of_stacks: SmallVec<[SmallVec<[char; 128]>; 128]> = smallvec!();
    let mut stack_lines: SmallVec<[&str; 128]> = smallvec!();
    let mut max_length = 0;

    for l in s.lines() {
        if l.is_empty() {
            break;
        }
        if l.len() > max_length {
            max_length = l.len();
        }
        stack_lines.push(l);
    }

    let num_stacks = (max_length + 1) / 4;

    for _ in 0..num_stacks {
        stack_of_stacks.push(smallvec!())
    }

    for (_, stack_line) in stack_lines.iter().rev().skip(1).enumerate() {
        for col in 0..num_stacks {
            let cell = (col * 4) + 1;
            if cell >= stack_line.len() {
                continue;
            }
            let c = stack_line.chars().nth(cell).unwrap();
            if c == ' ' {
                continue;
            }
            stack_of_stacks[col].push(stack_line.chars().nth(cell).unwrap());
        }
    }

    for line in s.lines().skip(stack_lines.len() + 1) {
        let mut words = line.split(' ');
        words.next().unwrap();
        let times = usize::from_str(words.next().unwrap()).unwrap();
        words.next().unwrap();
        let from = usize::from_str(words.next().unwrap()).unwrap() - 1;
        words.next().unwrap();
        let to = usize::from_str(words.next().unwrap()).unwrap() - 1;

        if !keep_order {
            for _ in 0..times {
                let popped = stack_of_stacks[from].pop().unwrap();
                stack_of_stacks[to].push(popped);
            }
        } else {
            let mut temp_vec: SmallVec<[char; 128]> = smallvec!();
            for _ in 0..times {
                let popped = stack_of_stacks[from].pop().unwrap();
                temp_vec.push(popped);
            }
            while !temp_vec.is_empty() {
                stack_of_stacks[to].push(temp_vec.pop().unwrap());
            }
        }
    }

    let mut buffer = Chars{0: smallvec![]};
    for mut stack in stack_of_stacks {
        buffer.0.push(stack.pop().unwrap())
    }

    buffer
}

#[allow(unused)]
pub fn _p1(s: &'static str) -> Chars {
    get_stacks(s, false)
}

#[allow(unused)]
pub fn p1() -> Chars {
    _p1(include_str!("j5.txt"))
}

#[allow(unused)]
pub fn _p2(s: &'static str) -> Chars {
    get_stacks(s, true)
}

#[allow(unused)]
pub fn p2() -> Chars {
    _p2(include_str!("j5.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j5_tests {
    use itertools::Itertools;

    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!("CMZ", _p1(include_str!("j5_test.txt")).0.into_iter().join(""));
        assert_eq!("BZLVHBWQF", _p1(include_str!("j5.txt")).0.into_iter().join(""));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!("MCD", _p2(include_str!("j5_test.txt")).0.into_iter().join(""));
        assert_eq!("TDGJQTZSL", _p2(include_str!("j5.txt")).0.into_iter().join(""));
    }
}