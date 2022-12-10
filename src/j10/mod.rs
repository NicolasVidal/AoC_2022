use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub struct P2Output([char; ROWS * COLS]);

impl Display for P2Output {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..ROWS {
            for col in 0..COLS {
                f.write_char(self.0[row * COLS + col])?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl From<&str> for P2Output {
    fn from(s: &str) -> Self {
        let mut chars = ['.'; ROWS * COLS];
        let mut cnt = 0;
        for line in s.lines() {
            for c in line.chars() {
                chars[cnt] = c;
                cnt += 1;
            }
        }
        P2Output(chars)
    }
}


#[inline(always)]
fn increase_and_check_cycle_and_draw_pixel(cycle: &mut i32,
                                           x: i32,
                                           total: &mut i32,
                                           pixels: &mut [char]) {
    if *cycle % 40 == x ||
        (*cycle) % 40 == (x - 1) % 40 ||
        (*cycle) % 40 == (x + 1) % 40 {
        pixels[(*cycle) as usize] = '#';
    }
    *cycle += 1;
    if (*cycle + 20) % 40 == 0 {
        *total += *cycle * x;
    }
}

#[inline(always)]
fn increase_and_check_cycle(cycle: &mut i32, x: i32, total: &mut i32) {
    *cycle += 1;
    if (*cycle + 20) % 40 == 0 {
        *total += *cycle * x;
    }
}

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut x: i32 = 1;

    let mut cycles = 0i32;
    let mut total = 0i32;

    for line in s.lines() {
        let mut words = line.split(' ');
        let command = words.next().unwrap();
        match command {
            "noop" => {
                increase_and_check_cycle(&mut cycles, x, &mut total);
            }
            "addx" => {
                increase_and_check_cycle(&mut cycles, x, &mut total);
                increase_and_check_cycle(&mut cycles, x, &mut total);
                x += i32::from_str(words.next().unwrap()).unwrap();
            }
            _ => panic!(),
        }
    }
    total as usize
}

const ROWS: usize = 6usize;
const COLS: usize = 40usize;

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j10.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> P2Output {
    let mut x: i32 = 1;

    let mut cycles = 0i32;
    let mut total = 0i32;

    let mut pixels = ['.'; ROWS * COLS];

    for line in s.lines() {
        let mut words = line.split(' ');
        let command = words.next().unwrap();
        match command {
            "noop" => {
                increase_and_check_cycle_and_draw_pixel(&mut cycles, x, &mut total, &mut pixels);
            }
            "addx" => {
                increase_and_check_cycle_and_draw_pixel(&mut cycles, x, &mut total, &mut pixels);
                increase_and_check_cycle_and_draw_pixel(&mut cycles, x, &mut total, &mut pixels);
                x += i32::from_str(words.next().unwrap()).unwrap();
            }
            _ => panic!(),
        }
    }
    P2Output(pixels)
}

#[allow(unused)]
pub fn p2() -> P2Output {
    _p2(include_str!("j10.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j10_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(13140, _p1(include_str!("j10_test.txt")));
        assert_eq!(12740, _p1(include_str!("j10.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(P2Output::from(include_str!("j10_test_p2_sol.txt")), _p2(include_str!("j10_test.txt")));
        assert_eq!(P2Output::from(include_str!("j10_p2_sol.txt")), _p2(include_str!("j10.txt")));
    }
}