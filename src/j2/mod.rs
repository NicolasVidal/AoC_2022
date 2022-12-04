use crate::j2::Outcome::{Draw, Lose, Win};
use crate::j2::PlayerMove::{Paper, Rock, Scissors};

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Outcome {
    Lose,
    Win,
    Draw,
}

impl Outcome {
    #[inline(always)]
    pub fn from_char(c: char) -> Outcome {
        match c {
            'X' => Lose,
            'Y' => Draw,
            'Z' => Win,
            _ => panic!(),
        }
    }

    #[inline(always)]
    pub fn compute_outcome(p1: PlayerMove, p2: PlayerMove) -> Outcome {
        match (p1, p2) {
            (Rock, Paper) => Lose,
            (Rock, Scissors) => Win,
            (Rock, Rock) => Draw,
            (Paper, Scissors) => Lose,
            (Paper, Rock) => Win,
            (Paper, Paper) => Draw,
            (Scissors, Rock) => Lose,
            (Scissors, Paper) => Win,
            (Scissors, Scissors) => Draw,
        }
    }

    #[inline(always)]
    pub fn compute_value(&self) -> u16 {
        match self {
            Lose => 0,
            Draw => 3,
            Win => 6,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum PlayerMove {
    Rock,
    Paper,
    Scissors,
}

impl PlayerMove {
    #[inline(always)]
    pub fn from_char(c: char) -> PlayerMove {
        match c {
            'A' | 'X' => Rock,
            'B' | 'Y' => Paper,
            'C' | 'Z' => Scissors,
            _ => panic!(),
        }
    }

    #[inline(always)]
    pub fn from_desired_outcome_for_second_player(first_move: PlayerMove, desired_outcome: Outcome) -> PlayerMove {
        match (first_move, desired_outcome) {
            (m, Draw) => m,
            (Rock, Win) => Paper,
            (Paper, Win) => Scissors,
            (Scissors, Win) => Rock,
            (Rock, Lose) => Scissors,
            (Paper, Lose) => Rock,
            (Scissors, Lose) => Paper,
        }
    }

    #[inline(always)]
    pub fn compute_value(&self) -> u16 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

#[inline(always)]
fn compute_score(first: PlayerMove, second: PlayerMove) -> u16 {
    Outcome::compute_outcome(second, first).compute_value() + second.compute_value()
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
        let first = PlayerMove::from_char(a);
        let second = PlayerMove::from_char(b);

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
        let first = PlayerMove::from_char(a);
        let second = PlayerMove::from_desired_outcome_for_second_player(first,
                                                                        Outcome::from_char(b));

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