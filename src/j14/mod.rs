use std::str::FromStr;

use smallvec::{SmallVec, smallvec};

#[derive(Copy, Clone, Debug)]
enum CellType {
    Empty,
    Sand,
    Rock,
}

impl Default for CellType {
    #[inline(always)]
    fn default() -> Self {
        CellType::Empty
    }
}

#[inline(always)]
fn get_row_and_col_min_max(s: &str) -> (i32, i32, i32, i32) {
    let mut row_min = 0;
    let mut row_max = 0;
    let mut col_min = i32::MAX;
    let mut col_max = 500;

    for line in s.lines() {
        let coordinates_split = line.split(" -> ");
        for coord_str in coordinates_split {
            let mut split_x_y = coord_str.split(',');
            let col = i32::from_str(split_x_y.next().unwrap()).unwrap();
            let row = i32::from_str(split_x_y.next().unwrap()).unwrap();

            row_min = row_min.min(row);
            row_max = row_max.max(row);
            col_min = col_min.min(col);
            col_max = col_max.max(col);
        }
    }
    (row_min, row_max, col_min, col_max)
}

#[inline(always)]
fn fill_rocks(s: &str, row_min: i32, col_min: i32, rows: i32, cols: i32) -> SmallVec<[CellType; 11270]> {
    let mut cells: SmallVec<[CellType; 11270]> = smallvec![];

    for _ in 0..(rows * cols) {
        cells.push(CellType::Empty);
    }

    for line in s.lines() {
        let mut last_coordinate = None; //(row, col)
        let coordinates_split = line.split(" -> ");
        for coord_str in coordinates_split {
            let mut split_x_y = coord_str.split(',');
            let target_col = i32::from_str(split_x_y.next().unwrap()).unwrap() - col_min;
            let target_row = i32::from_str(split_x_y.next().unwrap()).unwrap() - row_min;

            match last_coordinate {
                None => {}
                Some((from_row, from_col)) => {
                    let row_range = if target_row >= from_row {
                        from_row..=target_row
                    } else {
                        target_row..=from_row
                    };

                    let col_range = if target_col >= from_col {
                        from_col..=target_col
                    } else {
                        target_col..=from_col
                    };

                    for row in row_range {
                        for col in col_range.clone() {
                            cells[(row * cols + col) as usize] = CellType::Rock;
                        }
                    }
                }
            }
            last_coordinate = Some((target_row, target_col));
        }
    }
    cells
}

#[inline(always)]
fn drop_sand_until_filled_or_fall_off(row_min: i32, col_min: i32, rows: i32, cols: i32, cells: &mut SmallVec<[CellType; 11270]>) -> usize {
    let mut rest_units = 0;
    'outer: loop {
        let mut sand_coord = (0 - row_min, 500 - col_min);

        loop {
            if sand_coord.0 >= rows - 1 {
                break 'outer;
            }

            if let CellType::Empty = cells[((sand_coord.0 + 1) * cols + sand_coord.1) as usize] {
                sand_coord.0 += 1;
                continue;
            }

            if sand_coord.1 == 0 {
                break 'outer;
            }

            if let CellType::Empty = cells[((sand_coord.0 + 1) * cols + sand_coord.1 - 1) as usize] {
                sand_coord.0 += 1;
                sand_coord.1 -= 1;
                continue;
            }

            if sand_coord.1 == cols - 1 {
                break 'outer;
            }

            if let CellType::Empty = cells[((sand_coord.0 + 1) * cols + sand_coord.1 + 1) as usize] {
                sand_coord.0 += 1;
                sand_coord.1 += 1;
                continue;
            }

            rest_units += 1;
            cells[(sand_coord.0 * cols + sand_coord.1) as usize] = CellType::Sand;

            break;
        }

        if sand_coord.0 == 0 - row_min && sand_coord.1 == 500 - col_min {
            break;
        }
    }

    // dbg!((rows, cols));
    // for row in 0..rows {
    //     for col in 0..cols {
    //         print!("{}", match cells[(row * cols + col) as usize] {
    //             CellType::Empty => '.',
    //             CellType::Rock => '#',
    //             CellType::Sand => 'o',
    //         });
    //     }
    //     println!()
    // }
    // println!();

    rest_units as usize
}

#[inline(always)]
fn compute_rows_and_cols(row_min: i32, row_max: i32, col_min: i32, col_max: i32) -> (i32, i32) {
    let rows = row_max - row_min + 1;
    let cols = col_max - col_min + 1;
    (rows, cols)
}

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let (mut row_min, mut row_max, mut col_min, mut col_max) = get_row_and_col_min_max(s);

    let (rows, cols) = compute_rows_and_cols(row_min, row_max, col_min, col_max);

    let mut cells = fill_rocks(s, row_min, col_min, rows, cols);

    drop_sand_until_filled_or_fall_off(row_min, col_min, rows, cols, &mut cells)
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j14.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let (mut row_min, mut row_max, mut col_min, mut col_max) = get_row_and_col_min_max(s);

    // Increase world size
    row_max += 2;
    col_min = col_min.min(500 - row_max - 1);
    col_max = col_max.max(500 + row_max + 1);

    let (rows, cols) = compute_rows_and_cols(row_min, row_max, col_min, col_max);

    let mut cells = fill_rocks(s, row_min, col_min, rows, cols);

    // Fill bottom with rocks
    for col in 0..cols {
        cells[(row_max * cols + col) as usize] = CellType::Rock
    }

    drop_sand_until_filled_or_fall_off(row_min, col_min, rows, cols, &mut cells)
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j14.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j14_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(24, _p1(include_str!("j14_test.txt")));
        assert_eq!(1199, _p1(include_str!("j14.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(93, _p2(include_str!("j14_test.txt")));
        assert_eq!(23925, _p2(include_str!("j14.txt")));
    }
}