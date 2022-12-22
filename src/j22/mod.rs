use itertools::Itertools;
use smallvec::{SmallVec, smallvec};

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Cell {
    Void,
    Empty,
    Rock,
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum Instruction {
    Move(isize),
    TurnRight,
    TurnLeft,
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum Orientation {
    Right,
    Down,
    Left,
    Up,
}

#[allow(unused)]
fn print_grid(
    r: usize, c: usize,
    grid: &SmallVec<[SmallVec<[Cell; 200]>; 200]>) {
    for (row, line) in grid.iter().enumerate() {
        for (col, cell) in line.iter().enumerate() {
            print!("{}", match cell {
                _ if col == c && row == r => {
                    'X'
                }
                Cell::Void => { ' ' }
                Cell::Empty => { '.' }
                Cell::Rock => { '#' }
            });
        }
        println!()
    }
    println!()
}

fn execute_instruction(r: &mut usize, c: &mut usize, orientation: &mut Orientation,
                       grid: &SmallVec<[SmallVec<[Cell; 200]>; 200]>, instruction: Instruction) {
    match instruction {
        Instruction::Move(v) => {
            let mut move_left_count = v;
            loop {
                if move_left_count == 0 {
                    break;
                }
                let mut must_wrap = None;
                let (mut new_row, mut new_col) = match orientation {
                    Orientation::Right => { (*r as isize, *c as isize + 1) }
                    Orientation::Down => { (*r as isize + 1, *c as isize) }
                    Orientation::Left => { (*r as isize, *c as isize - 1) }
                    Orientation::Up => { (*r as isize - 1, *c as isize) }
                };
                if new_col < 0 {
                    must_wrap = Some(Orientation::Right);
                } else if new_col as usize >= grid[0].len() {
                    must_wrap = Some(Orientation::Left);
                } else if new_row < 0 {
                    must_wrap = Some(Orientation::Down);
                } else if new_row as usize >= grid.len() {
                    must_wrap = Some(Orientation::Up);
                } else if grid[new_row as usize][new_col as usize] == Cell::Void {
                    must_wrap = Some(match orientation {
                        Orientation::Right => { Orientation::Left }
                        Orientation::Down => { Orientation::Up }
                        Orientation::Left => { Orientation::Right }
                        Orientation::Up => { Orientation::Down }
                    });
                }

                if let Some(orientation) = must_wrap {
                    let (mut row, mut col) = (*r as isize, *c as isize);
                    loop {
                        let (ghost_row, ghost_col) = match orientation {
                            Orientation::Right => { (row, col + 1) }
                            Orientation::Down => { (row + 1, col) }
                            Orientation::Left => { (row, col - 1) }
                            Orientation::Up => { (row - 1, col) }
                        };
                        if ghost_col < 0 {
                            break;
                        } else if ghost_col as usize >= grid[0].len() {
                            break;
                        } else if ghost_row < 0 {
                            break;
                        } else if ghost_row as usize >= grid.len() {
                            break;
                        } else if grid[ghost_row as usize][ghost_col as usize] == Cell::Void {
                            break;
                        } else {
                            (row, col) = (ghost_row, ghost_col);
                        }
                    }
                    (new_row, new_col) = (row, col);
                }

                match grid[new_row as usize][new_col as usize] {
                    Cell::Void => {
                        panic!();
                    }
                    Cell::Empty => {
                        (*r, *c) = (new_row as usize, new_col as usize);
                        move_left_count -= 1;
                    }
                    Cell::Rock => {
                        break;
                    }
                }
            }
        }
        Instruction::TurnRight => {
            *orientation = match orientation {
                Orientation::Right => { Orientation::Down }
                Orientation::Down => { Orientation::Left }
                Orientation::Left => { Orientation::Up }
                Orientation::Up => { Orientation::Right }
            }
        }
        Instruction::TurnLeft => {
            *orientation = match orientation {
                Orientation::Right => { Orientation::Up }
                Orientation::Down => { Orientation::Right }
                Orientation::Left => { Orientation::Down }
                Orientation::Up => { Orientation::Left }
            }
        }
    }
}

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut max_row_size = 0usize;
    let mut grid: SmallVec<[SmallVec<[Cell; 200]>; 200]> = smallvec![];
    let mut instructions: SmallVec<[Instruction; 5000]> = smallvec![];
    for (row, line) in s.lines().enumerate() {
        if line.is_empty() {
            break;
        }
        grid.push(smallvec![]);
        for (col, c) in line.chars().enumerate() {
            grid[row].push(match c {
                ' ' => Cell::Void,
                '#' => Cell::Rock,
                '.' => Cell::Empty,
                _ => { panic!() }
            });
            max_row_size = max_row_size.max(col)
        }
    }
    max_row_size += 1;
    for row in grid.iter_mut() {
        for _ in 0..(max_row_size - row.len()) {
            row.push(Cell::Void);
        }
    }

    let mut val = 0;
    for c in s.lines().skip(grid.len() + 1).next().unwrap().chars() {
    // for c in "10R1L10".chars() {
        match c {
            c if c <= '9' && c >= '0' => {
                val = val * 10 + (c as u8 - b'0') as isize
            }
            'R' => {
                if val > 0 {
                    instructions.push(Instruction::Move(val));
                    val = 0;
                }
                instructions.push(Instruction::TurnRight);
            }
            'L' => {
                if val > 0 {
                    instructions.push(Instruction::Move(val));
                    val = 0;
                }
                instructions.push(Instruction::TurnLeft);
            }
            _ => panic!()
        }
    }
    if val > 0 {
        instructions.push(Instruction::Move(val));
        val = 0;
    }

    let mut r = 0;
    let mut c = grid[0].iter().find_position(|elt| **elt == Cell::Empty).unwrap().0;
    let mut orientation = Orientation::Right;

    for instruction in instructions.iter() {
        execute_instruction(&mut r, &mut c, &mut orientation, &grid, instruction.clone());
        // print_grid(r, c, &grid);
    }

    (r + 1) * 1000 + (c + 1) * 4 + match orientation {
        Orientation::Right => { 0 }
        Orientation::Down => { 1 }
        Orientation::Left => { 2 }
        Orientation::Up => { 3 }
    }
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j22.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    for line in s.lines() {}
    42
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j22.txt"))
}

// 20538 too high

#[cfg(test)]
#[allow(unused)]
mod j22_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(6032, _p1(include_str!("j22_test.txt")));
        assert_eq!(20494, _p1(include_str!("j22.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(42, _p2(include_str!("j22_test.txt")));
        assert_eq!(42, _p2(include_str!("j22.txt")));
    }
}