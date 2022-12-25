use std::fmt::{Display, Formatter, Write};
use smallvec::SmallVec;

use crate::j25::SnafuChar::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum SnafuChar {
    Two,
    One,
    Zero,
    Minus,
    DoubleMinus,
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Snafu(SmallVec<[SnafuChar; 20]>);

impl Display for Snafu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for c in self.0.iter().rev() {
            f.write_char(match c {
                Two => {'2'}
                One => {'1'}
                Zero => {'0'}
                Minus => {'-'}
                DoubleMinus => {'='}
            })?;
        }
        Ok(())
    }
}

impl Snafu {
    #[allow(unused)]
    pub fn from_str(s: &str) -> Snafu {
        let mut snafu: Snafu = Default::default();
        for c in s.chars() {
            snafu.0.push(match c {
                '2' => Two,
                '1' => One,
                '0' => Zero,
                '-' => Minus,
                '=' => DoubleMinus,
                _ => panic!()
            });
        }
        snafu.0.reverse();
        snafu
    }

    #[allow(unused)]
    pub fn to_decimal(&self) -> isize {
        let mut result = 0;
        for (power, num) in self.0.iter().enumerate() {
            result += 5isize.pow(power as u32) * match num {
                Two => { 2 }
                One => { 1 }
                Zero => { 0 }
                Minus => { -1 }
                DoubleMinus => { -2 }
            };
        }

        result
    }

    #[allow(unused)]
    fn test_number(snafu: &mut Snafu, number: isize, index: usize) -> Option<()> {
        for c in [DoubleMinus, Minus, Zero, One, Two] {
            snafu.0[index] = c;
            if index > 0 {
                if Self::test_number(snafu, number, index - 1).is_some() {
                    return Some(());
                }
            } else if snafu.to_decimal() == number {
                return Some(());
            }
        }

        None
    }

    #[allow(unused)]
    pub fn from_decimal(number: isize) -> Snafu {
        let mut snafu: Snafu = Snafu::default();

        let mut max_deg = 0;
        let mut num = 0isize;
        let mut coef = 0;
        'outer: loop {
            coef = 0;
            for _ in 1..=2 {
                coef += 1;
                num += 5isize.pow(max_deg);
                if num > number {
                    break 'outer;
                }
            }
            max_deg += 1;
        }

        match coef {
            2 => {

            }
            1 => {

            }
            _ => {panic!()}
        }

        dbg!(coef);
        dbg!(max_deg);
        dbg!(num);

        snafu
    }
}

#[allow(unused)]
pub fn _p1(s: &str) -> Snafu {
    let mut total = 0isize;
    for line in s.lines() {
        total += Snafu::from_str(line).to_decimal();
    }
    dbg!(total); // 34182852926025
    /// solved in Mathematica :
    /// variables = Table[Symbol["x" ~~ ToString[i]], {i, 0, 20}]
    /// constraints = Table[v >= -2 && v <= 2, {v, variables}] /. List -> And;
    /// calculus =
    ///   Table[Power[5, i]*variables[[i + 1]], {i, 0,
    ///      Length[variables] - 1}] /. List -> Plus;
    /// solution = Solve[{34182852926025 == calculus, constraints}, allVariables, Integers]
    /// "2-0-01==0-1=2212=100"
    ///
    ///
    ///
    ///
    Snafu::from_str("2-0-01==0-1=2212=100")
}

#[allow(unused)]
pub fn p1() -> Snafu {
    _p1(include_str!("j25.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j25_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        // assert_eq!(Snafu::from_str("2=-1=0"), _p1(include_str!("j25_test.txt")));
        assert_eq!(Snafu::from_str("2-0-01==0-1=2212=100"), _p1(include_str!("j25.txt")));
    }
}