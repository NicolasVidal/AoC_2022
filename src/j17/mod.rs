use std::fmt::{Display, Formatter, Write};

use smallvec::{SmallVec, smallvec};

#[derive(Eq, PartialEq, Debug)]
enum CellType {
    Empty,
    Fixed,
}

impl Display for CellType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            CellType::Empty => '.',
            CellType::Fixed => '#',
        })
    }
}

const EXPECTED_MAX_ROWS: usize = 20000;

const CHECK_PERIOD: usize = 500;

fn simulation(s: &str, num_rocks: usize) -> usize {
    let mut y_top_history: SmallVec<[usize; 200]> = smallvec![];

    let mut chamber: SmallVec<[SmallVec<[CellType; 7]>; EXPECTED_MAX_ROWS]> = smallvec![];
    for row in 0..EXPECTED_MAX_ROWS {
        chamber.push(smallvec![]);
        for _ in 0..7 {
            chamber[row].push(CellType::Empty);
        }
    }

    let mut rock_count = 0;

    let mut commands = s.chars().peekable();

    let mut old_top_y = 0;
    let mut top_y = 0;
    let mut dropped_lines = 0;

    let mut rock_type = 0;

    let mut falling_rock_cells: SmallVec<[(usize, usize); 5]> = smallvec![];

    while rock_count < num_rocks {
        // Spawn rock
        match rock_type {
            0 => {
                for col in 2usize..(2 + 4) {
                    falling_rock_cells.push((top_y + 3, col));
                }
            }
            1 => {
                falling_rock_cells.push((top_y + 3, 2 + 1));
                falling_rock_cells.push((top_y + 4, 2));
                falling_rock_cells.push((top_y + 4, 2 + 1));
                falling_rock_cells.push((top_y + 4, 2 + 2));
                falling_rock_cells.push((top_y + 5, 2 + 1));
            }
            2 => {
                falling_rock_cells.push((top_y + 3, 2));
                falling_rock_cells.push((top_y + 3, 2 + 1));
                falling_rock_cells.push((top_y + 3, 2 + 2));
                falling_rock_cells.push((top_y + 4, 2 + 2));
                falling_rock_cells.push((top_y + 5, 2 + 2));
            }
            3 => {
                falling_rock_cells.push((top_y + 3, 2));
                falling_rock_cells.push((top_y + 4, 2));
                falling_rock_cells.push((top_y + 5, 2));
                falling_rock_cells.push((top_y + 6, 2));
            }
            4 => {
                falling_rock_cells.push((top_y + 3, 2));
                falling_rock_cells.push((top_y + 3, 3));
                falling_rock_cells.push((top_y + 4, 2));
                falling_rock_cells.push((top_y + 4, 3));
            }
            _ => panic!()
        }

        // Fall rock
        loop {
            match commands.next() {
                Some('>') => {
                    if falling_rock_cells.iter().all(|(row, col)| *col + 1 < 7 &&
                        chamber[*row][(*col + 1)] == CellType::Empty) {
                        for (_, col) in falling_rock_cells.iter_mut() {
                            *col += 1;
                        }
                    }
                }
                Some('<') => {
                    if falling_rock_cells.iter().all(|(row, col)| *col > 0 &&
                        chamber[*row][(*col - 1)] == CellType::Empty) {
                        for (_, col) in falling_rock_cells.iter_mut() {
                            *col -= 1;
                        }
                    }
                }
                None => {
                    commands = s.chars().peekable();
                    continue;
                }
                _ => panic!()
            }

            if falling_rock_cells.iter().all(|(row, col)| *row > 0 &&
                chamber[*row - 1][*col] == CellType::Empty) {
                for (row, _) in falling_rock_cells.iter_mut() {
                    *row -= 1;
                }
            } else {
                break;
            }
        }

        // Fix cells
        for (row, col) in falling_rock_cells.drain(..) {
            chamber[row][col] = CellType::Fixed;
            top_y = top_y.max(row + 1);
        }

        // Update rock type
        rock_type = (rock_type + 1) % 5;

        rock_count += 1;

        if top_y >= EXPECTED_MAX_ROWS - 10 {
            let mut mask = [false; 7];
            let mut max_row = 0;

            for row in (0..top_y).rev() {
                for (col, m) in mask.iter_mut().enumerate() {
                    if chamber[row][col] == CellType::Fixed {
                        *m = true;
                    }
                }
                if mask.iter().all(|elt| *elt) {
                    max_row = row;
                    break;
                }
            }

            for _ in chamber.drain(..max_row) {}
            for _ in 0..max_row {
                chamber.push(SmallVec::from([(); 7].map(|_| CellType::Empty)))
            }
            top_y -= max_row;

            dropped_lines += max_row;
        }

        // Check for periodicity
        if rock_count % CHECK_PERIOD == 0 {
            y_top_history.push((top_y + dropped_lines) - old_top_y);

            if y_top_history.len() >= 7 {
                let first_window = &y_top_history[1..=3];
                for idx in 4..(y_top_history.len() - 3) {
                    if first_window == &y_top_history[idx..(idx + 3)] {
                        let period = idx - 1;
                        let period_counts = (num_rocks - CHECK_PERIOD) / CHECK_PERIOD;
                        let div = period_counts / period;
                        let remainder = period_counts % period;
                        let total = y_top_history[0] + div * y_top_history[1..(1 + period)].iter().sum::<usize>()
                            + y_top_history[1..(1 + remainder)].iter().sum::<usize>();

                        return total;
                    }
                }
            }

            old_top_y = top_y + dropped_lines
        }
    }

    top_y + dropped_lines
}

#[allow(unused)]
fn print_chamber(chamber: &mut SmallVec<[CellType; 140000]>,
                 falling_rock_cells: &SmallVec<[(usize, usize); 5]>) {
    for row in 0..=10 {
        let row = 10 - row;

        for col in 0..7 {
            if falling_rock_cells.contains(&(row, col)) {
                print!("@");
            } else {
                print!("{}", match chamber[row * 7 + col] {
                    CellType::Empty => '.',
                    CellType::Fixed => '#',
                });
            }
        }
        println!();
    }
    println!();
}

#[allow(unused)]
fn print_chamber_part(chamber: &SmallVec<[SmallVec<[CellType; 7]>; EXPECTED_MAX_ROWS]>,
                      start_row: usize,
                      end_row: usize) {
    for row in (start_row..=end_row).rev() {
        for col in 0..7 {
            print!("{}", match chamber[row][col] {
                CellType::Empty => '.',
                CellType::Fixed => '#',
            });
        }
        println!();
    }
    println!();
}


#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    simulation(s, 2022)
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j17.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    simulation(s, 1000000000000)
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j17.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j17_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(3068, _p1(include_str!("j17_test.txt")));
        assert_eq!(3141, _p1(include_str!("j17.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(1514285714288, _p2(include_str!("j17_test.txt")));
        assert_eq!(1561739130391, _p2(include_str!("j17.txt")));
    }
}