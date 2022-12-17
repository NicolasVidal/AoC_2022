use std::fmt::{Display, Formatter, Write};
use itertools::Itertools;
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

const EXPECTED_CHAMBER_SIZE: usize = 7 * 200000;

fn simulation(s: &str, num_rocks: usize) -> usize {
    // let mut chambers: SmallVec<[SmallVec<[CellType; EXPECTED_CHAMBER_SIZE]>; 100]> = smallvec![];
    let mut chamber: SmallVec<[CellType; EXPECTED_CHAMBER_SIZE]> = smallvec![];
    for _ in 0..EXPECTED_CHAMBER_SIZE {
        chamber.push(CellType::Empty);
    }

    let mut rock_count = 0;

    let mut commands = s.chars().peekable();

    let mut old_top_y = 0;
    let mut top_y = 0;

    let mut rock_type = 0;

    let mut falling_rock_cells: SmallVec<[(usize, usize); 5]> = smallvec![];

    let mut old_rocks: SmallVec<[usize; 100]> = smallvec![];
    let mut old_top_y_differences: SmallVec<[usize; 100]> = smallvec![];
    let mut old_top_y_rows: SmallVec<[usize; 100]> = smallvec![];
    let mut matching_sequence: Option<SmallVec<[usize; 100]>> = None;

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
                        chamber[*row * 7 + (*col + 1)] == CellType::Empty) {
                        for (_, col) in falling_rock_cells.iter_mut() {
                            *col += 1;
                        }
                    }
                }
                Some('<') => {
                    if falling_rock_cells.iter().all(|(row, col)| *col > 0 &&
                        chamber[*row * 7 + (*col - 1)] == CellType::Empty) {
                        for (_, col) in falling_rock_cells.iter_mut() {
                            *col -= 1;
                        }
                    }
                }
                None => {
                    let diff = top_y - old_top_y;
                    old_top_y = top_y;
                    if matching_sequence == None {
                        match old_top_y_differences.iter().rev().find_position(|elt| **elt == diff) {
                            None => {
                                old_top_y_differences.push(diff);
                                old_top_y_rows.push(top_y);
                                old_rocks.push(rock_count);
                            }
                            Some((pos, _)) => {
                                let origin_pos = old_top_y_differences.len() - 1 - pos;
                                old_top_y_differences.push(diff);
                                old_top_y_rows.push(top_y);
                                old_rocks.push(rock_count);

                                if pos * 2 < old_top_y_differences.len() &&
                                    (0..=pos).all(|idx| old_top_y_differences[origin_pos - idx] ==
                                        old_top_y_differences[old_top_y_differences.len() - 1 - idx]) {
                                    println!("Found sequence : {}", old_top_y_differences.iter().skip(
                                        old_top_y_differences.len() - 1 -
                                            pos).join(","));
                                    let mut sequence = smallvec![];
                                    for elt in old_top_y_differences
                                        .iter()
                                        .skip(old_top_y_differences.len() - 1 - pos) {
                                        sequence.push(*elt)
                                    }

                                    dbg!(origin_pos);
                                    dbg!(pos);
                                    dbg!(&old_top_y_differences);
                                    dbg!(&old_top_y_rows);

                                    let idx_start = old_top_y_differences.len() - 1 - pos - sequence.len() - 1;
                                    let idx_end = old_top_y_differences.len() - 1 - pos - 1;

                                    dbg!(idx_start);
                                    dbg!(idx_end);

                                    dbg!(old_top_y_rows[old_top_y_differences.len() - 1 - pos]);
                                    dbg!(old_top_y_rows[old_top_y_differences.len() - 1 - pos * 2]);

                                    let oldest_start = old_top_y_rows[old_top_y_differences.len() - 1 - pos - sequence.len() - 1];
                                    let seq_diff = old_top_y_rows[old_top_y_differences.len() - 1 - pos - 1] - oldest_start;

                                    dbg!(old_rocks[oldest_start]);
                                    dbg!(old_rocks[old_top_y_differences.len() - 1 - pos * 2]);

                                    print_chamber_part(&chamber, oldest_start, oldest_start + 10);
                                    print_chamber_part(&chamber, oldest_start + seq_diff, oldest_start + seq_diff  + 10);

                                    for row in oldest_start..(oldest_start + seq_diff) {
                                        println!("{}", chamber.iter().skip(row * 7).take(7).join(""));
                                        println!("{}", chamber.iter().skip((row + seq_diff) * 7).take(7).join(""));
                                        for col in 0..7 {
                                            assert_eq!(chamber[row * 7 + col], chamber[(row + seq_diff) * 7 + col]);
                                        }
                                        println!()
                                    }

                                    panic!();
                                    matching_sequence = Some(sequence);
                                }
                            }
                        }
                    }
                    commands = s.chars().peekable();
                    continue;
                }
                _ => panic!()
            }

            if falling_rock_cells.iter().all(|(row, col)| *row > 0 &&
                chamber[(row - 1) * 7 + col] == CellType::Empty) {
                for (row, _) in falling_rock_cells.iter_mut() {
                    *row -= 1;
                }
            } else {
                break;
            }
        }

        // Fix cells
        for (row, col) in falling_rock_cells.drain(..) {
            chamber[row * 7 + col] = CellType::Fixed;
            top_y = top_y.max(row + 1);
        }

        // Update rock type
        rock_type = (rock_type + 1) % 5;

        rock_count += 1;


        if let Some(seq) = &matching_sequence {
            // let line = y_top
        }
    }

    top_y
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
fn print_chamber_part(chamber: &SmallVec<[CellType; EXPECTED_CHAMBER_SIZE]>,
                      start_row: usize,
                      end_row: usize) {
    for row in (start_row..=end_row).rev() {

        for col in 0..7 {
            // if falling_rock_cells.contains(&(row, col)) {
            //     print!("@");
            // } else
            {
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
pub fn _p1(s: &str) -> usize {
    simulation(s, 2022)
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j17.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    simulation(s, 5000)
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
        assert_eq!(42, _p2(include_str!("j17.txt")));
    }
}