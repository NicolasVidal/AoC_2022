use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use itertools::Itertools;
use smallvec::{SmallVec, smallvec};

const ESTIMATED_MAX_NUMBER_OF_VALVES: usize = 60;
const ESTIMATED_MAX_NUMBER_OF_LETTERS: usize = 26 * 26;
const ESTIMATED_MAX_NUMBER_OF_VALVES_NEIGHBOURS: usize = 5;
const EMNOV: usize = ESTIMATED_MAX_NUMBER_OF_VALVES;


fn name_to_id(name: &str) -> usize {
    let mut name_chars = name.chars();
    (name_chars.next().unwrap() as u8 - b'A') as usize * 26 +
        (name_chars.next().unwrap() as u8 - b'A') as usize
}

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    find_maximum_reward_path(s, 30, false)
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j16.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    find_maximum_reward_path(s, 26, true)
}

fn find_maximum_reward_path(s: &str, time_left: usize, use_elephant: bool) -> usize {
    let mut valve_names: SmallVec<[&str; EMNOV]> = smallvec![];
    let mut valve_flow: SmallVec<[u32; EMNOV]> = smallvec![];
    let mut valve_neighbours_names: SmallVec<[SmallVec<[&str; ESTIMATED_MAX_NUMBER_OF_VALVES_NEIGHBOURS]>; EMNOV]> = smallvec![];
    let mut valve_neighbours: SmallVec<[SmallVec<[usize; ESTIMATED_MAX_NUMBER_OF_VALVES_NEIGHBOURS]>; EMNOV]> = smallvec![];
    let mut valve_states: SmallVec<[bool; EMNOV]> = smallvec![];
    let mut valve_names_to_ids: SmallVec<[Option<usize>; ESTIMATED_MAX_NUMBER_OF_LETTERS]> = smallvec![];

    for _ in 0..(26 * 26) {
        valve_names_to_ids.push(None)
    }

    let mut start_node = 0;
    for (id, line) in s.lines().enumerate() {
        let mut words = line.split(' ');
        words.next().unwrap();
        let name = words.next().unwrap();
        if name == "AA" {
            start_node = id;
        }
        valve_names.push(name);
        valve_names_to_ids[name_to_id(name)] = Some(id);
        words.next().unwrap();
        words.next().unwrap();
        let mut rate = words.next().unwrap().split('=');
        rate.next().unwrap();
        let flow = u32::from_str(rate.next().unwrap().split(';').next().unwrap()).unwrap();
        valve_flow.push(flow);
        words.next().unwrap();
        words.next().unwrap();
        words.next().unwrap();
        words.next().unwrap();
        valve_neighbours_names.push(smallvec![]);
        for word in words {
            let word = word.split(',').next().unwrap();
            valve_neighbours_names[id].push(word);
        }
        valve_states.push(false);
    }

    for neighbours_names in valve_neighbours_names.iter() {
        let mut neighbours = smallvec![];
        for neighbour_name in neighbours_names {
            neighbours.push(valve_names_to_ids[name_to_id(neighbour_name)].unwrap());
        }
        valve_neighbours.push(neighbours);
    }

    // // UnComment to show graph labels
    // print!("{{");
    // for (idx, v) in valve_flow.iter().enumerate() {
    //     print!("\"{}-{}\"", v, valve_names[idx]);
    //     if idx != valve_flow.len() - 1 {
    //         print!(",");
    //     }
    // }
    // print!("}}");
    // println!();

    let mut distances: SmallVec<[SmallVec<[usize; EMNOV]>; EMNOV]> = smallvec![];


    for start in 0..valve_names.len() {
        distances.push(smallvec![]);
        for end in 0..valve_names.len() {
            if valve_flow[end] > 0 {
                distances[start].push(dijkstra_simple(&valve_neighbours, start, end));
            } else {
                distances[start].push(0)
            }
        }
    }

    dijkstra(State {
        distances,
        current_valve: start_node,
        current_elephant_valve: start_node,
        current_score: 0,
        time_left,
        valve_flow,
        valve_neighbours,
        valve_states,
        valve_names: None, // Put this instead if you want to print the actions taken : Some(valve_names),
        from: None,
        from_elephant: None,
        player_move: true,
    }, use_elephant)
}

#[derive(Clone)]
struct State<'a> {
    distances: SmallVec<[SmallVec<[usize; EMNOV]>; EMNOV]>,
    current_valve: usize,
    current_elephant_valve: usize,
    current_score: u32,
    time_left: usize,
    valve_flow: SmallVec<[u32; EMNOV]>,
    valve_neighbours: SmallVec<[SmallVec<[usize; ESTIMATED_MAX_NUMBER_OF_VALVES_NEIGHBOURS]>; EMNOV]>,
    valve_states: SmallVec<[bool; EMNOV]>,
    valve_names: Option<SmallVec<[&'a str; EMNOV]>>,
    from: Option<usize>,
    from_elephant: Option<usize>,
    player_move: bool,
}

impl<'a> State<'a> {
    pub fn available_actions(&self) -> SmallVec<[usize; ESTIMATED_MAX_NUMBER_OF_VALVES]> {
        let mut actions = smallvec![];

        let node_id = if self.player_move { self.current_valve } else { self.current_elephant_valve };
        let from = if self.player_move { self.from } else { self.from_elephant };

        if !self.valve_states[node_id] && self.valve_flow[node_id] > 0 {
            actions.push(node_id)
        }

        for neighbour in self.valve_neighbours[node_id].iter() {
            if let Some(previous) = &from {
                if *previous == *neighbour {
                    continue;
                }
            }
            actions.push(*neighbour)
        }

        if actions.is_empty() {
            actions.push(usize::MAX);
        }

        actions
    }

    pub fn is_game_over(&self) -> bool {
        self.time_left == 0
    }

    pub fn act_with_action_id(&mut self, action_id: usize, use_elephant: bool) {
        let from = if self.player_move { &mut self.from } else { &mut self.from_elephant };
        if action_id < usize::MAX {
            let node_id = if self.player_move { &mut self.current_valve } else { &mut self.current_elephant_valve };

            if *node_id == action_id {
                self.valve_states[action_id] = true;
                self.current_score += self.valve_flow[action_id] * (self.time_left as u32 - 1);
                *from = None;
            } else {
                *from = Some(*node_id);
                *node_id = action_id;
            }
        } else {
            *from = None;
        }

        if !self.player_move || !use_elephant {
            self.time_left -= 1;
        }

        if use_elephant {
            self.player_move = !self.player_move;
        }
    }

    pub fn cost(&self) -> u32 {
        self.current_score as u32
    }

    pub fn heuristic(&self) -> u32 {
        self.valve_states.iter().enumerate().filter_map(|(idx, b)|
            if !b {
                Some(self.valve_flow[idx] * (self.time_left.saturating_sub(
                    self.distances[self.current_valve][idx].min(
                        self.distances[self.current_elephant_valve][idx]
                    )).saturating_sub(1) as u32
                ))
            } else { None }).sum::<u32>()
    }
}

#[derive(Clone, Debug)]
struct DijkstraNode {
    actions: SmallVec<[usize; 27 * 2]>,
    score: u32,
    heuristic: u32,
}

impl DijkstraNode {
    pub fn new(actions: SmallVec<[usize; 27 * 2]>, state: &State) -> Self {
        DijkstraNode {
            actions,
            score: state.cost(),
            heuristic: state.heuristic(),
        }
    }
}

impl PartialEq for DijkstraNode {
    fn eq(&self, other: &Self) -> bool {
        self.actions.eq(&other.actions)
    }
}

impl Eq for DijkstraNode {}

impl Hash for DijkstraNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.actions.hash(state)
    }
}

impl PartialOrd<Self> for DijkstraNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DijkstraNode {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.score + self.heuristic).cmp(&(other.score + other.heuristic))
    }
}

#[derive(Eq, PartialEq, Clone)]
struct SimpleDijkstraNode {
    cost: usize,
    position: usize,
}

impl SimpleDijkstraNode {}

impl PartialOrd<Self> for SimpleDijkstraNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SimpleDijkstraNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

fn dijkstra_simple(neighbours: &SmallVec<[SmallVec<[usize; ESTIMATED_MAX_NUMBER_OF_VALVES_NEIGHBOURS]>; EMNOV]>,
                   start: usize, end: usize) -> usize {
    let mut dist: Vec<_> = (0..neighbours.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(SimpleDijkstraNode { cost: 0, position: start });

    while let Some(SimpleDijkstraNode { cost, position }) = heap.pop() {
        if position == end { return cost; }

        if cost > dist[position] { continue; }

        for neighbour in &neighbours[position] {
            let next = SimpleDijkstraNode { cost: cost + 1, position: *neighbour };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next.clone());
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    panic!()
}


fn dijkstra(state: State, use_elephant: bool) -> usize {
    let mut left_to_explore = BinaryHeap::new();
    let mut dists = HashSet::new();

    let action: SmallVec<[usize; 27 * 2]> = smallvec![];

    let mut explored_actions = HashSet::new();
    let node = DijkstraNode::new(action, &state);
    left_to_explore.push(node.clone());
    dists.insert(node);

    let mut best_score = 0usize;

    while let Some(node) = left_to_explore.pop() {
        let mut cloned_state = state.clone();

        for a in node.actions.iter() {
            cloned_state.act_with_action_id(*a, use_elephant);
        }

        if cloned_state.current_score as usize > best_score {
            best_score = cloned_state.current_score as usize;
        }

        if cloned_state.is_game_over() {
            if let Some(valve_names) = &cloned_state.valve_names {
                println!("Actions taken : {}", node.actions.iter().map(|n|
                    if *n == usize::MAX { "__" } else {
                        valve_names[*n]
                    }).join(","));
            }
            return cloned_state.current_score as usize;
        }

        if let Some(old_node) = dists.get(&node) {
            if old_node.score > node.score {
                continue;
            }
        }

        for a in cloned_state.available_actions() {
            let mut cloned_actions = node.actions.clone();
            cloned_actions.push(a);

            let next_node = DijkstraNode::new(cloned_actions, &cloned_state);
            if let Some(old_node) = dists.get(&next_node) {
                if old_node.score >= next_node.score {
                    continue;
                }
            }

            left_to_explore.push(next_node.clone());
            dists.insert(next_node);
        }
        explored_actions.insert(node);
    }

    panic!()
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j16.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j16_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(1651, _p1(include_str!("j16_test.txt")));
        assert_eq!(2119, _p1(include_str!("j16.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(1707, _p2(include_str!("j16_test.txt")));
        assert_eq!(2615, _p2(include_str!("j16.txt")));
    }
}