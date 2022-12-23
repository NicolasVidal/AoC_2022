use smallvec::SmallVec;

use crate::j23::Cell::*;
use crate::j23::Direction::*;

const TEST_INITIAL_HEIGHT: usize = 7;
const TEST_INITIAL_WIDTH: usize = 7;
const INITIAL_SIZE_HEIGHT: usize = 75;
const INITIAL_SIZE_WIDTH: usize = 75;

const LEFT_OFFSET: usize = 15;
const RIGHT_OFFSET: usize = 53;
const TOP_OFFSET: usize = 14;
const BOTTOM_OFFSET: usize = 56;

#[allow(unused)]
const TEST_HEIGHT: usize = TEST_INITIAL_HEIGHT + TOP_OFFSET + BOTTOM_OFFSET;

#[allow(unused)]
const TEST_WIDTH: usize = TEST_INITIAL_WIDTH + LEFT_OFFSET + RIGHT_OFFSET;

const PROBLEM_HEIGHT: usize = INITIAL_SIZE_HEIGHT + TOP_OFFSET + BOTTOM_OFFSET;
const PROBLEM_WIDTH: usize = INITIAL_SIZE_WIDTH + LEFT_OFFSET + RIGHT_OFFSET;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Elf {
    wish: Option<(u8, u8)>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Cell {
    Empty,
    Elf(Elf),
}

type GridLine<const WIDTH: usize> = SmallVec<[Cell; WIDTH]>;
type Grid<const HEIGHT: usize, const WIDTH: usize> = SmallVec<[GridLine<WIDTH>; HEIGHT]>;

type GridWishLine<const WIDTH: usize> = SmallVec<[(u8, SmallVec<[(u8, u8); 4]>); WIDTH]>;
type GridWish<const HEIGHT: usize, const WIDTH: usize> = SmallVec<[GridWishLine<WIDTH>; HEIGHT]>;

fn get_north_neighbours<const HEIGHT: usize, const WIDTH: usize>(row: usize, col: usize, grid: &Grid<HEIGHT, WIDTH>) -> SmallVec<[Cell; 3]> {
    let mut neighbours: SmallVec<[Cell; 3]> = Default::default();
    if row > 0 {
        if col > 0 {
            neighbours.push(grid[row - 1][col - 1]);
        } else {
            panic!();
        }
        neighbours.push(grid[row - 1][col]);
        if col < (WIDTH - 1) {
            neighbours.push(grid[row - 1][col + 1]);
        } else {
            panic!();
        }
    }
    neighbours
}

fn get_south_neighbours<const HEIGHT: usize, const WIDTH: usize>(row: usize, col: usize, grid: &Grid<HEIGHT, WIDTH>) -> SmallVec<[Cell; 3]> {
    let mut neighbours: SmallVec<[Cell; 3]> = Default::default();
    if row < (HEIGHT - 1) {
        if col < (WIDTH - 1) {
            neighbours.push(grid[row + 1][col + 1]);
        } else {
            panic!();
        }
        neighbours.push(grid[row + 1][col]);
        if col > 0 {
            neighbours.push(grid[row + 1][col - 1]);
        } else {
            panic!();
        }
    }
    neighbours
}

fn get_east_neighbours<const HEIGHT: usize, const WIDTH: usize>(row: usize, col: usize, grid: &Grid<HEIGHT, WIDTH>) -> SmallVec<[Cell; 3]> {
    let mut neighbours: SmallVec<[Cell; 3]> = Default::default();
    if col < (WIDTH - 1) {
        if row > 0 {
            neighbours.push(grid[row - 1][col + 1]);
        } else {
            panic!();
        }
        neighbours.push(grid[row][col + 1]);
        if row < (HEIGHT - 1) {
            neighbours.push(grid[row + 1][col + 1]);
        } else {
            panic!();
        }
    }
    neighbours
}

fn get_west_neighbours<const HEIGHT: usize, const WIDTH: usize>(row: usize, col: usize, grid: &Grid<HEIGHT, WIDTH>) -> SmallVec<[Cell; 3]> {
    let mut neighbours: SmallVec<[Cell; 3]> = Default::default();
    if col > 0 {
        if row < (HEIGHT - 1) {
            neighbours.push(grid[row + 1][col - 1]);
        } else {
            panic!();
        }
        neighbours.push(grid[row][col - 1]);
        if row > 0 {
            neighbours.push(grid[row - 1][col - 1]);
        } else {
            panic!();
        }
    }
    neighbours
}

fn get_all_neighbours_coords<const HEIGHT: usize, const WIDTH: usize>(row: usize, col: usize) -> SmallVec<[(usize, usize); 8]> {
    let mut neighbours: SmallVec<[(usize, usize); 8]> = Default::default();
    if row > 0 {
        if col > 0 {
            neighbours.push((row - 1, col - 1));
        }
        neighbours.push((row - 1, col));
        if col < (WIDTH - 1) {
            neighbours.push((row - 1, col + 1));
        }
    }
    if col < (WIDTH - 1) {
        neighbours.push((row, col + 1));
    }
    if row < (HEIGHT - 1) {
        if col < (WIDTH - 1) {
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

fn update_wishes<const HEIGHT: usize, const WIDTH: usize>(grid_wishes: &mut GridWish<HEIGHT, WIDTH>, grid: &mut Grid<HEIGHT, WIDTH>, directions: &[Direction; 4]) -> bool {
    let mut wishes_changed = false;
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            match grid[row][col] {
                Empty => {}
                Elf(mut elf) => {
                    if get_all_neighbours_coords::<HEIGHT, WIDTH>(row, col).into_iter().all(|(r, c)|
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
                            grid_wishes[wished_row][wished_col].1.push((row as u8, col as u8));
                            elf.wish = Some((wished_row as u8, wished_col as u8));
                            grid[row][col] = Elf(elf);
                        }
                    }
                }
            }
        }
    }

    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            match &grid_wishes[row][col] {
                (v, elves) if *v > 1 => {
                    for (n_row, n_col) in elves.iter() {
                        match &mut grid[*n_row as usize][*n_col as usize] {
                            Empty => { panic!() }
                            Elf(elf) => {
                                elf.wish = None;
                            }
                        }
                    }
                }
                (v, _) if *v == 1 => {
                    wishes_changed = true;
                }
                _ => {}
            }
        }
    }
    wishes_changed
}

fn update_global_directions(directions: &mut [Direction; 4]) {
    let start_pos = 0;
    for i in start_pos..3 {
        directions.swap(i, i + 1);
    }
}

fn update_positions<const HEIGHT: usize, const WIDTH: usize>(grid: &mut Grid<HEIGHT, WIDTH>) {
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            match grid[row][col] {
                Empty => {}
                Elf(mut elf) => {
                    if let Some((wished_row, wished_col)) = elf.wish {
                        elf.wish = None;
                        grid[wished_row as usize][wished_col as usize] = Elf(elf);
                        grid[row][col] = Empty
                    }
                }
            }
        }
    }
}

fn clear_wishes<const HEIGHT: usize, const WIDTH: usize>(grid_wishes: &mut GridWish<HEIGHT, WIDTH>) {
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            grid_wishes[row][col] = (0, Default::default());
        }
    }
}

#[allow(unused)]
pub fn _p1<const HEIGHT: usize, const WIDTH: usize>(s: &str) -> usize {
    let mut grid: Grid<HEIGHT, WIDTH> = Default::default();
    let mut grid_wish: GridWish<HEIGHT, WIDTH> = Default::default();

    create_grid_wish(&mut grid_wish);

    create_grid(s, &mut grid);

    let mut directions = [North, South, West, East];

    for _ in 0..10 {
        update_wishes(&mut grid_wish, &mut grid, &directions);
        update_positions(&mut grid);
        update_global_directions(&mut directions);
        clear_wishes(&mut grid_wish);
    }

    let mut min_row = usize::MAX;
    let mut min_col = usize::MAX;
    let mut max_row = 0;
    let mut max_col = 0;

    for (row, line) in grid.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            match c {
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
fn print_grid_wishes<const HEIGHT: usize, const WIDTH: usize>(grid_wish: &GridWish<HEIGHT, WIDTH>) {
    for r in grid_wish.iter() {
        for (c, _) in r.iter() {
            print!("{}", match *c {
                e if e > 0 => e.to_string(),
                _ => '.'.to_string(),
            });
        }
        println!();
    }
    println!();
}

#[allow(unused)]
fn print_grid<const HEIGHT: usize, const WIDTH: usize>(grid: &Grid<HEIGHT, WIDTH>) {
    for r in grid.iter() {
        for c in r.iter() {
            print!("{}", match c {
                Empty => { '.' }
                Elf(_) => { '#' }
            });
        }
        println!();
    }
    println!();
}

fn count_elves<const HEIGHT: usize, const WIDTH: usize>(grid: &Grid<HEIGHT, WIDTH>) -> usize {
    let mut total = 0usize;
    for r in grid.iter() {
        for c in r.iter() {
            match c {
                Empty => {}
                Elf(_) => { total += 1 }
            }
        }
    }
    total
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1::<PROBLEM_HEIGHT, PROBLEM_WIDTH>(include_str!("j23.txt"))
}

#[allow(unused)]
pub fn _p2<const HEIGHT: usize, const WIDTH: usize>(s: &str) -> usize {
    let mut grid: Grid<HEIGHT, WIDTH> = Default::default();
    let mut grid_wish: GridWish<HEIGHT, WIDTH> = Default::default();

    create_grid_wish(&mut grid_wish);

    create_grid(s, &mut grid);

    let mut directions = [North, South, West, East];

    for id in 0.. {
        if !update_wishes(&mut grid_wish, &mut grid, &directions) {
            return id + 1;
        }
        update_positions(&mut grid);
        update_global_directions(&mut directions);
        clear_wishes(&mut grid_wish);
    }
    panic!()
}

fn create_grid<const HEIGHT: usize, const WIDTH: usize>(s: &str, grid: &mut Grid<HEIGHT, WIDTH>) {
    for row in 0..TOP_OFFSET {
        grid.push(Default::default());
        for _ in 0..WIDTH {
            grid[row].push(Empty);
        }
    }
    for (row, line) in s.lines().enumerate() {
        grid.push(Default::default());
        for _ in 0..LEFT_OFFSET {
            grid[row + TOP_OFFSET].push(Empty);
        }
        for c in line.chars() {
            grid[row + TOP_OFFSET].push(match c {
                '.' => Empty,
                '#' => Elf(Elf {
                    wish: None,
                }),
                _ => panic!()
            });
        }
        for _ in 0..RIGHT_OFFSET {
            grid[row + TOP_OFFSET].push(Empty);
        }
    }
    let init_size = grid.len();
    for row in 0..BOTTOM_OFFSET {
        grid.push(Default::default());
        for _ in 0..WIDTH {
            grid[row + init_size].push(Empty);
        }
    }
}

fn create_grid_wish<const HEIGHT: usize, const WIDTH: usize>(grid_wish: &mut GridWish<HEIGHT, WIDTH>) {
    for row in 0..HEIGHT {
        grid_wish.push(Default::default());
        for _ in 0..WIDTH {
            grid_wish[row].push((0, Default::default()));
        }
    }
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2::<PROBLEM_HEIGHT, PROBLEM_WIDTH>(include_str!("j23.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j23_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(110, _p1::<TEST_HEIGHT, TEST_WIDTH>(include_str!("j23_test.txt")));
        assert_eq!(4249, _p1::<PROBLEM_HEIGHT, PROBLEM_WIDTH>(include_str!("j23.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(20, _p2::<TEST_HEIGHT, TEST_WIDTH>(include_str!("j23_test.txt")));
        assert_eq!(980, _p2::<PROBLEM_HEIGHT, PROBLEM_WIDTH>(include_str!("j23.txt")));
    }
}