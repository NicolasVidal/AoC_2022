use smallvec::{SmallVec, smallvec};
use vec_collections::{AbstractVecSet, VecSet};

struct Cell {
    north_b: bool,
    east_b: bool,
    south_b: bool,
    west_b: bool,
    blocked: bool,
}

impl Cell {
    fn from_char(c: char) -> Self {
        Cell {
            north_b: c == '^',
            east_b: c == '>',
            south_b: c == 'v',
            west_b: c == '<',
            blocked: c == '#',
        }
    }
}

type Grid = SmallVec<[SmallVec<[Cell; 102]>; 37]>;
type World = SmallVec<[SmallVec<[bool; 102]>; 37]>;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Elt((u16, u8, u8));

struct ExploredNodes(VecSet<[Elt; 123927]>);

impl ExploredNodes {
    pub fn push(&mut self, elt: Elt) {
        match self.0.insert(elt) {
            true => {}
            false => {}
        }
    }

    pub fn contains(&self, elt: &Elt) -> bool {
        self.0.contains(elt)
    }
}

fn breadth_first_search(grid: &Grid, world: &mut World, (start_row, start_col): (usize, usize), (end_row, end_col): (usize, usize),
                        start_cost: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    let periodicity = (rows - 2) * (cols - 2);


    let mut nodes: SmallVec<[Elt; 1200]> = smallvec!(Elt((start_cost as u16, (start_row as u16).try_into().unwrap(), (start_col as u16).try_into().unwrap())));
    let mut childs: SmallVec<[Elt; 1200]> = smallvec!();
    let mut explored: ExploredNodes = ExploredNodes(VecSet::empty());
    explored.push(nodes[0]);

    let mut max_len = nodes.len();
    while !nodes.is_empty() {
        max_len = max_len.max(nodes.len());
        let mut generated = false;
        for elt in nodes.drain(..) {
            let (cost, row, col) = elt.0;
            if row == end_row as u8 && col == end_col as u8 {
                return cost as usize;
            }

            if !generated {
                generated = true;
                let p = (cost + 1) % periodicity as u16;
                for row in 0..rows {
                    for col in 0..cols {
                        world[row][col] = true;
                    }
                }
                for (row, grid_row) in grid.iter().enumerate().take(rows) {
                    for (col, cell) in grid_row.iter().enumerate().take(cols) {
                        if cell.blocked {
                            world[row][col] = false;
                        }
                        if cell.north_b {
                            let p = p as isize;
                            let mut row = row as isize;
                            let rows = rows as isize;
                            let new_p = p % (rows - 2);
                            if new_p >= row {
                                row = row + rows - 2
                            }
                            let (target_row, target_col) = (row - new_p, col);
                            world[target_row as usize][target_col] = false;
                        }
                        if cell.south_b {
                            let p = p as isize;
                            let mut row = row as isize;
                            let rows = rows as isize;
                            let new_p = p % (rows - 2);
                            if new_p + row >= rows - 1 {
                                row = row - rows + 2
                            }
                            let (target_row, target_col) = (row + new_p, col);
                            world[target_row as usize][target_col] = false;
                        }
                        if cell.east_b {
                            let p = p as isize;
                            let mut col = col as isize;
                            let cols = cols as isize;
                            let new_p = p % (cols - 2);
                            if new_p + col >= cols - 1 {
                                col = col - cols + 2
                            }
                            let (target_row, target_col) = (row, col + new_p);
                            world[target_row][target_col as usize] = false;
                        }
                        if cell.west_b {
                            let p = p as isize;
                            let mut col = col as isize;
                            let cols = cols as isize;
                            let new_p = p % (cols - 2);
                            if new_p >= col {
                                col = col + cols - 2
                            }
                            let (target_row, target_col) = (row, col - new_p);
                            world[target_row][target_col as usize] = false;
                        }
                    }
                }
            }

            let new_p = (cost as usize + 1) % periodicity;

            if world[u32::from(row) as usize][u32::from(col) as usize] &&
                !explored.contains(&Elt((new_p as u16, row, col))) {
                explored.push(Elt((new_p as u16, row, col)));
                childs.push(Elt((cost + 1, row, col)));
            }

            if row > 0 && world[u32::from(row) as usize - 1][u32::from(col) as usize] &&
                !explored.contains(&Elt((new_p as u16, row - 1, col))) {
                explored.push(Elt((new_p as u16, row - 1, col)));
                childs.push(Elt((cost + 1, row - 1, col)));
            }

            if row < rows as u8 - 1 && world[row as usize + 1][col as usize] &&
                !explored.contains(&Elt((new_p as u16, row + 1, col))) {
                explored.push(Elt((new_p as u16, row + 1, col)));
                childs.push(Elt((cost + 1, row + 1, col)));
            }

            if col > 0 && world[row as usize][col as usize - 1] &&
                !explored.contains(&Elt((new_p as u16, row, col - 1))) {
                explored.push(Elt((new_p as u16, row, col - 1)));
                childs.push(Elt((cost + 1, row, col - 1)));
            }

            if col < cols as u8 - 1 && world[row as usize][col as usize + 1] &&
                !explored.contains(&Elt((new_p as u16, row, col + 1))) {
                explored.push(Elt((new_p as u16, row, col + 1)));
                childs.push(Elt((cost + 1, row, col + 1)));
            }
        }
        std::mem::swap(&mut nodes, &mut childs);
    }
    panic!();
}

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let (grid, mut world) = compute_grid_and_create_world_state(s);

    breadth_first_search(&grid, &mut world, (0, 1), (grid.len() - 1, grid[0].len() - 2), 0)
}

fn compute_grid_and_create_world_state(s: &str) -> (Grid, World) {
    let mut grid: Grid = smallvec!();
    for line in s.lines() {
        let mut row = smallvec!();
        for c in line.chars() {
            row.push(Cell::from_char(c));
        }
        grid.push(row)
    }
    let mut world: World = smallvec![];
    for row in 0..grid.len() {
        world.push(smallvec![]);
        for _ in 0..grid[0].len() {
            world[row].push(true)
        }
    }
    (grid, world)
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j24.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let (grid, mut world) = compute_grid_and_create_world_state(s);

    let first_way_cost = breadth_first_search(&grid, &mut world, (0, 1), (grid.len() - 1, grid[0].len() - 2), 0);
    let second_way_cost = breadth_first_search(&grid, &mut world, (grid.len() - 1, grid[0].len() - 2), (0, 1), first_way_cost);
    breadth_first_search(&grid, &mut world, (0, 1), (grid.len() - 1, grid[0].len() - 2), second_way_cost)
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j24.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j24_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(18, _p1(include_str!("j24_test.txt")));
        assert_eq!(262, _p1(include_str!("j24.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(54, _p2(include_str!("j24_test.txt")));
        assert_eq!(785, _p2(include_str!("j24.txt")));
    }
}