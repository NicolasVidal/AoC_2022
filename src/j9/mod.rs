use std::ops::{AddAssign, Range, Sub};
use std::str::FromStr;

use smallvec::{SmallVec, smallvec};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Vec2D((i32, i32));

impl AddAssign for Vec2D {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.0.0 += rhs.0.0;
        self.0.1 += rhs.0.1;
    }
}

impl Sub for Vec2D {
    type Output = Vec2D;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2D((self.0.0 - rhs.0.0, self.0.1 - rhs.0.1))
    }
}

impl From<(i32, i32)> for Vec2D {
    #[inline(always)]
    fn from(tuple: (i32, i32)) -> Self {
        Self {
            0: tuple
        }
    }
}

#[allow(unused)]
fn print_grid(range: Range<i32>, tail_pos: Vec2D, head_pos: Vec2D) {
    for row in range.clone() {
        for col in range.clone() {
            if head_pos == (row, col).into() {
                print!("H");
            } else if tail_pos == (row, col).into() {
                print!("T");
            } else {
                print!("_");
            }
        }
        println!()
    }
    println!()
}

#[inline(always)]
fn get_movement_direction(direction: char) -> Vec2D {
    match direction {
        'R' => (0, 1),
        'L' => (0, -1),
        'U' => (1, 0),
        'D' => (-1, 0),
        _ => { panic!() }
    }.into()
}

pub fn solve<const ROPE_LENGTH: usize,
    const EXPECTED_MAX_TAIL_MOVEMENTS: usize,
    const EXPECTED_VIRTUAL_GRID_CELLS: usize>(s: &str) -> usize {
    let mut found_positions: SmallVec<[Vec2D; EXPECTED_MAX_TAIL_MOVEMENTS]> = smallvec![];

    let mut knots: [Vec2D; ROPE_LENGTH] = [(0i32, 0i32).into(); ROPE_LENGTH];
    let tail_pos = knots[knots.len() - 1];
    found_positions.push(tail_pos);
    for line in s.lines() {
        let mut chars = line.split(' ');
        let direction = chars.next().unwrap().chars().next().unwrap();
        let steps = usize::from_str(chars.next().unwrap()).unwrap();
        let movement = get_movement_direction(direction);

        'outer: for _ in 0..steps {
            knots[0] += movement.into();
            for i in 0..(ROPE_LENGTH - 1) {
                let diff = knots[i] - knots[i + 1];
                if diff.0.0.abs() <= 1 && diff.0.1.abs() <= 1 {
                    continue 'outer;
                }
                knots[i + 1] = match diff.0 {
                    (2, -2) => knots[i] - (1, -1).into(),
                    (-2, -2) => knots[i] - (-1, -1).into(),
                    (-2, 2) => knots[i] - (-1, 1).into(),
                    (2, 2) => knots[i] - (1, 1).into(),
                    (2, _) => knots[i] - (1, 0).into(),
                    (-2, _) => knots[i] - (-1, 0).into(),
                    (_, 2) => knots[i] - (0, 1).into(),
                    (_, -2) => knots[i] - (0, -1).into(),
                    _ => {
                        dbg!(diff);
                        panic!()
                    }
                };
            }
            let tail_pos = knots[knots.len() - 1];
            found_positions.push(tail_pos);
        }
    }
    let mut bottom_most = 0i32;
    let mut left_most = 0i32;
    let mut up_most = 0i32;
    let mut right_most = 0i32;

    for elt in found_positions.iter() {
        bottom_most = bottom_most.min(elt.0.0);
        left_most = left_most.min(elt.0.1);
        up_most = up_most.max(elt.0.0);
        right_most = right_most.max(elt.0.1);
    }
    let grid_width = right_most - left_most + 1;

    let mut grid = SmallVec::from([false; EXPECTED_VIRTUAL_GRID_CELLS]);

    for elt in found_positions.iter() {
        let row = elt.0.0 - bottom_most;
        let col = elt.0.1 - left_most;
        let idx = row * grid_width + col;
        grid[idx as usize] = true;
    }

    grid.into_iter().filter(|elt| *elt).count()
}

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    solve::<2, 20000, 89112>(s)
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j9.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    solve::<10, 3000, 83296>(s)
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j9.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j9_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(13, _p1(include_str!("j9_test.txt")));
        assert_eq!(6406, _p1(include_str!("j9.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(1, _p2(include_str!("j9_test.txt")));
        assert_eq!(36, _p2(include_str!("j9_test2.txt")));
        assert_eq!(2643, _p2(include_str!("j9.txt")));
    }
}