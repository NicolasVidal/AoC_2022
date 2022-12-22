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
                       grid: &SmallVec<[SmallVec<[Cell; 200]>; 200]>, instruction: Instruction,
                       on_cube: bool, test_wrapping: bool) {
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

                let mut potential_new_orientation = orientation.clone();
                if let Some(orientation) = must_wrap {
                    if on_cube {
                        new_row = *r as isize;
                        new_col = *c as isize;
                        // dbg!(new_row, new_col, potential_new_orientation.clone());
                        if test_wrapping {
                            apply_test_folding(&mut new_row, &mut new_col, &mut potential_new_orientation);
                        } else {
                            apply_folding(&mut new_row, &mut new_col, &mut potential_new_orientation);
                        }
                        // dbg!(new_row, new_col, potential_new_orientation.clone());
                    } else {
                        let (mut row, mut col) = (*r as isize, *c as isize);
                        loop {
                            let (ghost_row, ghost_col) = match orientation {
                                Orientation::Right => { (row, col + 1) }
                                Orientation::Down => { (row + 1, col) }
                                Orientation::Left => { (row, col - 1) }
                                Orientation::Up => { (row - 1, col) }
                            };
                            if ghost_col < 0
                                || ghost_col as usize >= grid[0].len()
                                || ghost_row < 0
                                || ghost_row as usize >= grid.len()
                                || grid[ghost_row as usize][ghost_col as usize] == Cell::Void {
                                break;
                            } else {
                                (row, col) = (ghost_row, ghost_col);
                            }
                        }
                        (new_row, new_col) = (row, col);
                    }
                }

                match grid[new_row as usize][new_col as usize] {
                    Cell::Void => {
                        panic!();
                    }
                    Cell::Empty => {
                        (*r, *c) = (new_row as usize, new_col as usize);
                        if on_cube {
                            *orientation = potential_new_orientation;
                        }
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
    for c in s.lines().nth(grid.len() + 1).unwrap().chars() {
        // for c in "10R1L10".chars() {
        match c {
            c if ('0'..='9').contains(&c) => {
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
        execute_instruction(&mut r, &mut c, &mut orientation, &grid, instruction.clone(), false, false);
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


fn apply_test_folding(r: &mut isize, c: &mut isize, orientation: &mut Orientation) {
    match orientation {
        Orientation::Right => {
            if *r >= 0 && *r <= 3 {
                *r = 11 - *r % 4;
                *c = 15;
                *orientation = Orientation::Left;
                return;
            }
            if *r >= 4 && *r <= 7 {
                *c = 15 - *r % 4;
                *r = 8;
                *orientation = Orientation::Down;
                return;
            }
            if *r >= 8 && *r <= 11 {
                *r = 3 - *r % 4;
                *c = 11;
                *orientation = Orientation::Left;
            }
        }
        Orientation::Down => {
            if *c >= 0 && *c <= 3 {
                *c = 11 - *c % 4;
                *r = 11;
                *orientation = Orientation::Up;
                return;
            }
            if *c >= 4 && *c <= 7 {
                *r = 11 - *c % 4;
                *c = 8;
                *orientation = Orientation::Right;
                return;
            }
            if *c >= 8 && *c <= 11 {
                *c = 3 - *c % 4;
                *r = 7;
                *orientation = Orientation::Up;
                return;
            }
            if *c >= 12 && *c <= 15 {
                *r = 4 + *c % 4;
                *c = 0;
                *orientation = Orientation::Right;
            }
        }
        Orientation::Left => {
            if *r >= 0 && *r <= 3 {
                *c = 4 + *r % 4;
                *r = 4;
                *orientation = Orientation::Down;
                return;
            }
            if *r >= 4 && *r <= 7 {
                *c = 15 - *r % 4;
                *r = 11;
                *orientation = Orientation::Up;
                return;
            }
            if *r >= 8 && *r <= 11 {
                *c = 7 - *r % 4;
                *r = 7;
                *orientation = Orientation::Up;
            }
        }
        Orientation::Up => {
            if *c >= 0 && *c <= 3 {
                *c = 11 - *c % 4;
                *r = 0;
                *orientation = Orientation::Down;
                return;
            }
            if *c >= 4 && *c <= 7 {
                *r = *c % 4;
                *c = 8;
                *orientation = Orientation::Right;
                return;
            }
            if *c >= 8 && *c <= 11 {
                *c = 3 - *c % 4;
                *r = 4;
                *orientation = Orientation::Down;
                return;
            }
            if *c >= 12 && *c <= 15 {
                *r = 7 - *c % 4;
                *c = 11;
                *orientation = Orientation::Left;
            }
        }
    }
}

fn apply_folding(r: &mut isize, c: &mut isize, orientation: &mut Orientation) {
    match orientation {
        Orientation::Right => {
            if *r >= 0 && *r <= 49 {
                *r = 149 - *r % 50;
                *c = 99;
                *orientation = Orientation::Left;
                return;
            }
            if *r >= 50 && *r <= 99 {
                *c = 100 + *r % 50;
                *r = 0;
                *orientation = Orientation::Up;
                return;
            }
            if *r >= 100 && *r <= 149 {
                *r = 49 - *r % 50;
                *c = 149;
                *orientation = Orientation::Left;
                return;
            }
            if *r >= 150 && *r <= 199 {
                *c = 50 + *r % 50;
                *r = 149;
                *orientation = Orientation::Up;
            }
        }
        Orientation::Down => {
            if *c >= 0 && *c <= 49 {
                *c = 100 + *c % 50;
                *r = 0;
                *orientation = Orientation::Down;
                return;
            }
            if *c >= 50 && *c <= 99 {
                *r = 150 + *c % 50;
                *c = 49;
                *orientation = Orientation::Left;
                return;
            }
            if *c >= 100 && *c <= 149 {
                *r = 50 + *c % 50;
                *c = 99;
                *orientation = Orientation::Left;
            }
        }
        Orientation::Left => {
            if *r >= 0 && *r <= 49 {
                *r = 149 - *r % 50;
                *c = 0;
                *orientation = Orientation::Right;
                return;
            }
            if *r >= 50 && *r <= 99 {
                *c = *r % 50;
                *r = 100;
                *orientation = Orientation::Down;
                return;
            }
            if *r >= 100 && *r <= 149 {
                *r = 49 - *r % 50;
                *c = 50;
                *orientation = Orientation::Right;
                return;
            }
            if *r >= 150 && *r <= 199 {
                *c = 50 + *r % 50;
                *r = 0;
                *orientation = Orientation::Down;
            }
        }
        Orientation::Up => {
            if *c >= 0 && *c <= 49 {
                *r = 50 + *c % 50;
                *c = 50;
                *orientation = Orientation::Right;
                return;
            }
            if *c >= 50 && *c <= 99 {
                *r = 150 + *c % 50;
                *c = 0;
                *orientation = Orientation::Right;
                return;
            }
            if *c >= 100 && *c <= 149 {
                *c %= 50;
                *r = 199;
                *orientation = Orientation::Up;
            }
        }
    }
}

#[allow(unused)]
pub fn _p2(s: &str, test_wrapping: bool) -> usize {
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
    for c in s.lines().nth(grid.len() + 1).unwrap().chars() {
        // for c in "10R1L10".chars() {
        match c {
            c if ('0'..='9').contains(&c) => {
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
        execute_instruction(&mut r, &mut c, &mut orientation, &grid, instruction.clone(), true, test_wrapping);
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
pub fn p2() -> usize {
    _p2(include_str!("j22.txt"), false)
}

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
        assert_eq!(5031, _p2(include_str!("j22_test.txt"), true));
        assert_eq!(55343, _p2(include_str!("j22.txt"), false));
    }
}