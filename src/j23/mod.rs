use smallvec::SmallVec;

use crate::j23::Cell::*;
use crate::j23::Direction::*;

const INITIAL_SIZE: usize = 75;
const OFFSET: usize = 10;
const SIDE_LENGTH: usize = INITIAL_SIZE + OFFSET * 2;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Elf {
    directions: [Direction; 4],
    wish: Option<(usize, usize)>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Cell {
    Empty,
    Elf(Elf),
}

type GridLine = SmallVec<[Cell; SIDE_LENGTH]>;
type Grid = SmallVec<[GridLine; SIDE_LENGTH]>;

type GridWishLine = SmallVec<[(usize, SmallVec<[(usize, usize); 4]>); SIDE_LENGTH]>;
type GridWish = SmallVec<[GridWishLine; SIDE_LENGTH]>;

fn get_north_neighbours(row: usize, col: usize, grid: &Grid) -> SmallVec<[Cell; 3]> {
    let mut neighbours: SmallVec<[Cell; 3]> = Default::default();
    if row > 0 {
        if col > 0 {
            neighbours.push(grid[row - 1][col - 1]);
        }
        neighbours.push(grid[row - 1][col]);
        if col < SIDE_LENGTH - 1 {
            neighbours.push(grid[row - 1][col + 1]);
        }
    }
    neighbours
}

fn get_south_neighbours(row: usize, col: usize, grid: &Grid) -> SmallVec<[Cell; 3]> {
    let mut neighbours: SmallVec<[Cell; 3]> = Default::default();
    if row < SIDE_LENGTH - 1 {
        if col < SIDE_LENGTH - 1 {
            neighbours.push(grid[row + 1][col + 1]);
        }
        neighbours.push(grid[row + 1][col]);
        if col > 0 {
            neighbours.push(grid[row + 1][col - 1]);
        }
    }
    neighbours
}

fn get_east_neighbours(row: usize, col: usize, grid: &Grid) -> SmallVec<[Cell; 3]> {
    let mut neighbours: SmallVec<[Cell; 3]> = Default::default();
    if col < SIDE_LENGTH - 1 {
        if col > 0 {
            neighbours.push(grid[row - 1][col + 1]);
        }
        neighbours.push(grid[row][col + 1]);
        if row < SIDE_LENGTH - 1 {
            neighbours.push(grid[row + 1][col + 1]);
        }
    }
    neighbours
}

fn get_west_neighbours(row: usize, col: usize, grid: &Grid) -> SmallVec<[Cell; 3]> {
    let mut neighbours: SmallVec<[Cell; 3]> = Default::default();
    if col > 0 {
        if row < SIDE_LENGTH - 1 {
            neighbours.push(grid[row + 1][col - 1]);
        }
        neighbours.push(grid[row][col - 1]);
        if col > 0 {
            neighbours.push(grid[row - 1][col - 1]);
        }
    }
    neighbours
}

fn get_all_neighbours_coords(row: usize, col: usize) -> SmallVec<[(usize, usize); 8]> {
    let mut neighbours: SmallVec<[(usize, usize); 8]> = Default::default();
    if row > 0 {
        if col > 0 {
            neighbours.push((row - 1, col - 1));
        }
        neighbours.push((row - 1, col));
        if col < SIDE_LENGTH - 1 {
            neighbours.push((row - 1, col + 1));
        }
    }
    if col < SIDE_LENGTH - 1 {
        neighbours.push((row, col + 1));
    }
    if row < SIDE_LENGTH - 1 {
        if col < SIDE_LENGTH - 1 {
            neighbours.push((row + 1, col + 1));
        }
        neighbours.push((row + 1, col));
        if col > 0 {
            neighbours.push((row + 1, col - 1));
        }
    }
    if col > 0 {
        neighbours.push((row, col - 1));
    }
    neighbours
}

fn update_wishes(grid_wishes: &mut GridWish, grid: &mut Grid, directions: &[Direction; 4]) {
    for row in 0..(SIDE_LENGTH - 1) {
        for col in 0..(SIDE_LENGTH - 1) {
            match grid[row][col] {
                Empty => {}
                Elf(mut elf) => {
                    if get_all_neighbours_coords(row, col).into_iter().all(|(r, c)|
                        grid[r][c] == Empty
                    ) {
                        continue;
                    }
                    match directions.iter().map(|d| match d {
                        North => { (d, get_north_neighbours(row, col, grid)) }
                        South => { (d, get_south_neighbours(row, col, grid)) }
                        West => { (d, get_west_neighbours(row, col, grid)) }
                        East => { (d, get_east_neighbours(row, col, grid)) }
                    }).find(|(_, neighbours)|
                        neighbours.len() == 3 &&
                            neighbours.iter().all(|n| *n == Empty)) {
                        None => {}
                        Some((dir, _)) => {
                            let (wished_row, wished_col) = match dir {
                                North => { (row - 1, col) }
                                South => { (row + 1, col) }
                                West => { (row, col - 1) }
                                East => { (row, col + 1) }
                            };
                            grid_wishes[wished_row][wished_col].0 += 1;
                            grid_wishes[wished_row][wished_col].1.push((row, col));
                            // dbg!(dir.clone());
                            // let start_pos = elf.directions.iter().find_position(|d| **d == *dir).unwrap().0;
                            // dbg!(elf.directions);
                            elf.wish = Some((wished_row, wished_col));
                            grid[row][col] = Elf(elf);
                        }
                    }
                }
            }
        }
    }
    for row in 0..(SIDE_LENGTH - 1) {
        for col in 0..(SIDE_LENGTH - 1) {
            match &grid_wishes[row][col] {
                (v, elves) if *v > 1 => {
                    for (n_row, n_col) in elves.iter() {
                        match &mut grid[*n_row][*n_col] {
                            Empty => { panic!() }
                            Elf(elf) => {
                                elf.wish = None;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn update_global_directions(directions: &mut [Direction; 4]) {
    let start_pos = 0;
    for i in start_pos..3 {
        directions.swap(i, i + 1);
    }
}

fn update_positions(grid: &mut Grid) {
    for row in 0..SIDE_LENGTH {
        for col in 0..SIDE_LENGTH {
            match grid[row][col] {
                Empty => {}
                Elf(mut elf) => {
                    if let Some((wished_row, wished_col)) = elf.wish {
                        elf.wish = None;
                        grid[wished_row][wished_col] = Elf(elf);
                        grid[row][col] = Empty
                    }
                }
            }
        }
    }
}

fn clear_wishes(grid_wishes: &mut GridWish) {
    for row in 0..SIDE_LENGTH {
        for col in 0..SIDE_LENGTH {
            grid_wishes[row][col] = (0, Default::default());
        }
    }
}

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut grid: Grid = Default::default();
    let mut grid_wish: GridWish = Default::default();

    for row in 0..SIDE_LENGTH {
        grid_wish.push(Default::default());
        for col in 0..SIDE_LENGTH {
            grid_wish[row].push((0, Default::default()));
        }
    }

    for row in 0..OFFSET {
        grid.push(Default::default());
        for _ in 0..SIDE_LENGTH {
            grid[row].push(Empty);
        }
    }
    for (row, line) in s.lines().enumerate() {
        grid.push(Default::default());
        for _ in 0..OFFSET {
            grid[row + OFFSET].push(Empty);
        }
        for (col, c) in line.chars().enumerate() {
            grid[row + OFFSET].push(match c {
                '.' => Empty,
                '#' => Elf(Elf {
                    wish: None,
                    directions: [North, South, West, East],
                }),
                _ => panic!()
            });
        }
        for _ in 0..OFFSET {
            grid[row + OFFSET].push(Empty);
        }
    }
    for row in 0..OFFSET {
        grid.push(Default::default());
        for _ in 0..SIDE_LENGTH {
            grid[row + SIDE_LENGTH - OFFSET].push(Empty);
        }
    }

    let mut directions = [North, South, West, East];

    // print_grid(&grid);
    for _ in 0..10 {
        update_wishes(&mut grid_wish, &mut grid, &directions);
        update_positions(&mut grid);
        // print_grid_wishes(&grid_wish);
        update_global_directions(&mut directions);
        clear_wishes(&mut grid_wish);
        // print_grid(&grid);
    }

    let mut min_row = usize::MAX;
    let mut min_col = usize::MAX;
    let mut max_row = 0;
    let mut max_col = 0;

    for row in 0..SIDE_LENGTH {
        for col in 0..SIDE_LENGTH {
            match grid[row][col] {
                Empty => {}
                Elf(_) => {
                    min_row = min_row.min(row);
                    min_col = min_col.min(col);
                    max_row = max_row.max(row);
                    max_col = max_col.max(col);
                }
            }
        }
    }

    (min_row..=max_row).count() * (min_col..=max_col).count() - count_elves(&grid)
}

#[allow(unused)]
fn print_grid_wishes(grid_wish: &GridWish) {
    for r in 0..SIDE_LENGTH {
        for c in 0..SIDE_LENGTH {
            print!("{}", match grid_wish[r][c] {
                (e, _) if e > 0 => e.to_string(),
                _ => '.'.to_string(),
            });
        }
        println!();
    }
    println!();
}

#[allow(unused)]
fn print_grid(grid: &Grid) {
    for r in 0..SIDE_LENGTH {
        for c in 0..SIDE_LENGTH {
            print!("{}", match grid[r][c] {
                Empty => { '.' }
                Elf(_) => { '#' }
            });
        }
        println!();
    }
    println!();
}

fn count_elves(grid: &Grid) -> usize {
    let mut total = 0usize;
    for r in 0..SIDE_LENGTH {
        for c in 0..SIDE_LENGTH {
            match grid[r][c] {
                Empty => {}
                Elf(_) => { total += 1 }
            }
        }
    }
    total
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j23.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    for line in s.lines() {}
    42
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j23.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j23_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        // assert_eq!(42, _p1(include_str!("j23_small_test.txt")));
        // assert_eq!(110, _p1(include_str!("j23_test.txt")));
        assert_eq!(4249, _p1(include_str!("j23.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(42, _p2(include_str!("j23_test.txt")));
        assert_eq!(42, _p2(include_str!("j23.txt")));
    }
}