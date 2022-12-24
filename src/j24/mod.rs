use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::ops::Range;

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

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as a `usize`, for a shorter implementation.
#[derive(Debug)]
struct Edge {
    node: usize,
    cost: usize,
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal_range: Range<usize>) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if goal_range.contains(&position) { return Some(cost); }

        // Important as we may have already found a better way
        if cost > dist[position] { continue; }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = State { cost: cost + edge.cost, position: edge.node };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}


#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let (rows, cols, periodicity, edges) = compute_graph_and_edges(s);

    shortest_path(&edges, periodicity * rows * cols,
                  (periodicity * rows * cols + periodicity)..(periodicity * rows * cols + 2 * periodicity)).unwrap()
}

fn compute_graph_and_edges(s: &str) -> (usize, usize, usize, Vec<Vec<Edge>>) {
    let mut grid = vec!();
    for line in s.lines() {
        let mut row = vec!();
        for c in line.chars() {
            row.push(Cell::from_char(c));
        }
        grid.push(row)
    }

    let rows = grid.len() - 2;
    let cols = grid[0].len() - 2;

    let periodicity = (rows) * (cols);

    let mut world = Vec::with_capacity(periodicity + 1);
    for p in 0..(periodicity + 1) {
        world.push(Vec::with_capacity(rows));
        for row in 0..rows {
            world[p].push(Vec::with_capacity(cols));
            for _ in 0..cols {
                world[p][row].push(true)
            }
        }
    }

    for p in 0..(periodicity + 1) {
        for row in 0..rows {
            for col in 0..cols {
                let cell = &grid[row + 1][col + 1];
                if cell.blocked {
                    world[p][row][col] = false;
                }
                if cell.north_b {
                    let p = p as isize;
                    let mut row = row as isize;
                    let rows = rows as isize;
                    let new_p = p % rows;
                    if new_p > row {
                        row = row + rows
                    }
                    let (target_row, target_col) = (row - new_p, col);
                    world[p as usize][target_row as usize][target_col as usize] = false;
                }
                if cell.south_b {
                    let p = p as isize;
                    let mut row = row as isize;
                    let rows = rows as isize;
                    let new_p = p % rows;
                    if new_p + row > rows - 1 {
                        row = row - rows
                    }
                    let (target_row, target_col) = (row + new_p, col);
                    world[p as usize][target_row as usize][target_col as usize] = false;
                }
                if cell.east_b {
                    let p = p as isize;
                    let mut col = col as isize;
                    let cols = cols as isize;
                    let new_p = p % cols;
                    if new_p + col > cols - 1 {
                        col = col - cols
                    }
                    let (target_row, target_col) = (row, col + new_p);
                    world[p as usize][target_row as usize][target_col as usize] = false;
                }
                if cell.west_b {
                    let p = p as isize;
                    let mut col = col as isize;
                    let cols = cols as isize;
                    let new_p = p % cols;
                    if new_p > col {
                        col = col + cols
                    }
                    let (target_row, target_col) = (row, col - new_p);
                    world[p as usize][target_row as usize][target_col as usize] = false;
                }
            }
        }
    }

    // for p in 0..(periodicity + 1) {
    //     for row in 0..rows {
    //         for col in 0..cols {
    //             print!("{}", match world[p][row][col] {
    //                 true => { '.' }
    //                 false => { '#' }
    //             });
    //         }
    //         println!();
    //     }
    //     println!();
    // }

    let mut edges = Vec::with_capacity(world.len() * world[0].len() * world[0][0].len() + world.len());
    for p in 0..(periodicity) {
        for row in 0..rows {
            for col in 0..cols {
                let mut node_edges = Vec::with_capacity(5);
                let new_p = (p + 1) % periodicity;

                if row == 0 && col == 0 {
                    node_edges.push(
                        Edge {
                            node: periodicity * rows * cols + new_p,
                            cost: 1,
                        }
                    );
                }

                if row == rows - 1 && col == cols - 1 {
                    node_edges.push(
                        Edge {
                            node: periodicity * rows * cols + new_p + periodicity,
                            cost: 1,
                        }
                    );
                }

                if world[new_p][row][col] {
                    node_edges.push(
                        Edge {
                            node: new_p * rows * cols + row * cols + col,
                            cost: 1,
                        }
                    );
                }
                if row > 0 && world[new_p][row - 1][col] {
                    node_edges.push(
                        Edge {
                            node: new_p * rows * cols + (row - 1) * cols + col,
                            cost: 1,
                        });
                }
                if row < rows - 1 && world[new_p][row + 1][col] {
                    node_edges.push(
                        Edge {
                            node: new_p * rows * cols + (row + 1) * cols + col,
                            cost: 1,
                        });
                }
                if col > 0 && world[new_p][row][col - 1] {
                    node_edges.push(
                        Edge {
                            node: new_p * rows * cols + row * cols + col - 1,
                            cost: 1,
                        });
                }
                if col < cols - 1 && world[new_p][row][col + 1] {
                    node_edges.push(
                        Edge {
                            node: new_p * rows * cols + row * cols + col + 1,
                            cost: 1,
                        });
                }
                edges.push(node_edges);
            }
        }
    }

    for p in 0..(periodicity) {
        let new_p = (p + 1) % periodicity;
        edges.push(vec!(Edge {
            node: new_p * rows * cols,
            cost: 1,
        }, Edge {
            node: periodicity * rows * cols + new_p,
            cost: 1,
        }));
    }

    for p in 0..(periodicity) {
        let new_p = (p + 1) % periodicity;
        edges.push(vec!(Edge {
            node: new_p * rows * cols + (rows - 1) * cols + (cols - 1),
            cost: 1,
        }, Edge {
            node: periodicity * rows * cols + new_p + periodicity,
            cost: 1,
        }));
    }
    (rows, cols, periodicity, edges)
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j24.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let (rows, cols, periodicity, edges) = compute_graph_and_edges(s);

    let first_way_cost = shortest_path(&edges, periodicity * rows * cols,
                                      (periodicity * rows * cols + periodicity)..(periodicity * rows * cols + 2 * periodicity)).unwrap();

    let second_way_cost = shortest_path(&edges, periodicity * rows * cols + periodicity + (first_way_cost) % periodicity,
                                        (periodicity * rows * cols)..(periodicity * rows * cols + periodicity)).unwrap();

    let last_way_cost = shortest_path(&edges, periodicity * rows * cols + (first_way_cost + second_way_cost) % periodicity,
                                      (periodicity * rows * cols + periodicity)..(periodicity * rows * cols + 2 * periodicity)).unwrap();

    dbg!(periodicity);
    dbg!(first_way_cost, second_way_cost, last_way_cost);
    dbg!(first_way_cost + second_way_cost + last_way_cost)
}

// 763 too low

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
        assert_eq!(42, _p2(include_str!("j24.txt")));
    }
}