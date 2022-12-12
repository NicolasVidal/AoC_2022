use smallvec::{SmallVec, smallvec};

#[derive(Debug, Clone)]
struct Link {
    pub node: usize,
    pub weight: i32,
}

#[derive(Debug, Clone)]
struct NodeToExplore {
    pub node: usize,
    pub tag: i32,
}

#[derive(Debug, Clone)]
struct NodeNeighbours {
    pub neighbours: SmallVec<[Link; 4]>,
}

pub fn compute_path_cost<const EXPECTED_NODES_COUNT: usize,
    const EXPECTED_NODES_TAG: usize>(s: &str, any_square: bool) -> usize {
    let rows = s.lines().enumerate().count();
    let cols = s.lines().next().unwrap().chars().count();

    let mut nodes_height: SmallVec<[i32; EXPECTED_NODES_COUNT]> = smallvec![];

    let mut start_node = 0;
    let mut end_node = 0;

    for (row, line) in s.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            nodes_height.push(match char {
                'S' => {
                    start_node = row * cols + col;
                    0i32
                }
                'E' => {
                    end_node = row * cols + col;
                    (b'z' - b'a') as i32
                }
                c => (c as u8 - b'a') as i32,
            });
        }
    }

    let mut nodes_neighbours: SmallVec<[NodeNeighbours; EXPECTED_NODES_COUNT]> = smallvec![];

    for node in 0..nodes_height.len() {
        let mut node_neighbours = NodeNeighbours {
            neighbours: smallvec![]
        };
        let row = node / cols;
        let col = node % cols;
        let height = nodes_height[node];

        if col > 0 {
            let target_node = node - 1;
            if nodes_height[target_node] - height <= 1 {
                node_neighbours.neighbours.push(Link {
                    node: target_node,
                    weight: 1,
                })
            }
        }

        if col < cols - 1 {
            let target_node = node + 1;
            if nodes_height[target_node] - height <= 1 {
                node_neighbours.neighbours.push(Link {
                    node: target_node,
                    weight: 1,
                })
            }
        }

        if row > 0 {
            let target_node = node - cols;
            if nodes_height[target_node] - height <= 1 {
                node_neighbours.neighbours.push(Link {
                    node: target_node,
                    weight: 1,
                })
            }
        }

        if row < rows - 1 {
            let target_node = node + cols;
            if nodes_height[target_node] - height <= 1 {
                node_neighbours.neighbours.push(Link {
                    node: target_node,
                    weight: 1,
                })
            }
        }

        nodes_neighbours.push(node_neighbours);
    }

    // Dijkstra

    let mut nodes_explored: SmallVec<[bool; EXPECTED_NODES_COUNT]> = smallvec![];
    let mut nodes_tags: SmallVec<[NodeToExplore; EXPECTED_NODES_TAG]> = smallvec![];

    for _ in 0..nodes_height.len() {
        nodes_explored.push(false);
    }

    if any_square {
        for (idx, height) in nodes_height.iter().enumerate() {
            if *height == 0 {
                nodes_tags.push(NodeToExplore {
                    node: idx,
                    tag: 0,
                })
            }
        }
    } else {
        nodes_tags.push(NodeToExplore {
            node: start_node,
            tag: 0,
        });
    }

    while !nodes_tags.is_empty() {
        // We don't need to sort as the grid is uniform
        // nodes_tags.sort_by_key(|n| n.tag);

        if nodes_explored[nodes_tags[0].node] {
            nodes_tags.remove(0);
            continue;
        }

        if nodes_tags[0].node == end_node {
            return nodes_tags[0].tag as usize;
        }

        for neighbour in nodes_neighbours[nodes_tags[0].node].neighbours.iter() {
            if nodes_explored[neighbour.node] {
                continue;
            }

            nodes_tags.push(NodeToExplore {
                node: neighbour.node,
                tag: nodes_tags[0].tag + neighbour.weight,
            })
        }

        nodes_explored[nodes_tags[0].node] = true;
        nodes_tags.remove(0);
    }

    panic!("No Path Found, wtf ?")
}


#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    compute_path_cost::<2501, 10004>(s, false)
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j12.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    compute_path_cost::<2501, 10004>(s, true)
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j12.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j12_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(31, _p1(include_str!("j12_test.txt")));
        assert_eq!(330, _p1(include_str!("j12.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(29, _p2(include_str!("j12_test.txt")));
        assert_eq!(321, _p2(include_str!("j12.txt")));
    }
}