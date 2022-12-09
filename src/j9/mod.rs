use std::collections::HashSet;
use std::ops::{AddAssign, Range, Sub};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Vec2D((i32, i32));

impl AddAssign for Vec2D {
    fn add_assign(&mut self, rhs: Self) {
        self.0.0 += rhs.0.0;
        self.0.1 += rhs.0.1;
    }
}

impl Sub for Vec2D {
    type Output = Vec2D;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2D((self.0.0 - rhs.0.0, self.0.1 - rhs.0.1))
    }
}

impl From<(i32, i32)> for Vec2D {
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

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let grid_range = -6..6;
    let mut unique_positions = HashSet::new();
    let mut head_pos: Vec2D = (0i32, 0i32).into();
    let mut tail_pos: Vec2D = (0i32, 0i32).into();
    unique_positions.insert(tail_pos);
    for line in s.lines() {
        let mut chars = line.split(' ');
        let direction = chars.next().unwrap().chars().next().unwrap();
        let steps = usize::from_str(chars.next().unwrap()).unwrap();
        let movement = get_movement_direction(direction);
        for _ in 0..steps {
            head_pos += movement.into();
            let diff = head_pos - tail_pos;
            if diff.0.0.abs() <= 1 && diff.0.1.abs() <= 1 {
                continue;
            }
            tail_pos = match diff.0 {
                (2, 0) => (head_pos - (1, 0).into()),
                (-2, 0) => (head_pos - (-1, 0).into()),
                (0, 2) => (head_pos - (0, 1).into()),
                (0, -2) => (head_pos - (0, -1).into()),
                (2, 1) => (head_pos - (1, 0).into()),
                (-2, 1) => (head_pos - (-1, 0).into()),
                (1, 2) => (head_pos - (0, 1).into()),
                (1, -2) => (head_pos - (0, -1).into()),
                (2, -1) => (head_pos - (1, 0).into()),
                (-2, -1) => (head_pos - (-1, 0).into()),
                (-1, 2) => (head_pos - (0, 1).into()),
                (-1, -2) => (head_pos - (0, -1).into()),
                (2, -2) => (head_pos - (1, -1).into()),
                (-2, -2) => (head_pos - (-1, -1).into()),
                (-2, 2) => (head_pos - (-1, 1).into()),
                (-2, -2) => (head_pos - (-1, -1).into()),
                _ => panic!()
            };
            unique_positions.insert(tail_pos);
        }
    }
    unique_positions.len()
}

fn get_movement_direction(direction: char) -> Vec2D {
    match direction {
        'R' => (0, 1),
        'L' => (0, -1),
        'U' => (1, 0),
        'D' => (-1, 0),
        _ => { panic!() }
    }.into()
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j9.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let grid_range = -6..6;
    let mut unique_positions = HashSet::new();

    let mut knots: [Vec2D; 10] = [(0i32, 0i32).into(); 10];
    let mut head_pos = knots[0];
    let mut tail_pos = knots[knots.len() - 1];

    unique_positions.insert(tail_pos);
    for line in s.lines() {
        let mut chars = line.split(' ');
        let direction = chars.next().unwrap().chars().next().unwrap();
        let steps = usize::from_str(chars.next().unwrap()).unwrap();
        let movement = get_movement_direction(direction);

        for _ in 0..steps {
            knots[0] += movement.into();
            for i in 0..(knots.len() - 1) {
                let diff = knots[i] - knots[i + 1];
                if diff.0.0.abs() <= 1 && diff.0.1.abs() <= 1 {
                    continue;
                }
                knots[i + 1] = match diff.0 {
                    (2, 0) => (knots[i] - (1, 0).into()),
                    (-2, 0) => (knots[i] - (-1, 0).into()),
                    (0, 2) => (knots[i] - (0, 1).into()),
                    (0, -2) => (knots[i] - (0, -1).into()),
                    (2, 1) => (knots[i] - (1, 0).into()),
                    (-2, 1) => (knots[i] - (-1, 0).into()),
                    (1, 2) => (knots[i] - (0, 1).into()),
                    (1, -2) => (knots[i] - (0, -1).into()),
                    (2, -1) => (knots[i] - (1, 0).into()),
                    (-2, -1) => (knots[i] - (-1, 0).into()),
                    (-1, 2) => (knots[i] - (0, 1).into()),
                    (-1, -2) => (knots[i] - (0, -1).into()),
                    (2, -2) => (knots[i] - (1, -1).into()),
                    (-2, -2) => (knots[i] - (-1, -1).into()),
                    (-2, 2) => (knots[i] - (-1, 1).into()),
                    (2, 2) => (knots[i] - (1, 1).into()),
                    _ => {
                        dbg!(diff);
                        panic!()
                    }
                };
            }
            unique_positions.insert(knots[knots.len() - 1]);
        }
    }
    unique_positions.len()
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