use std::hash::{Hash, Hasher};
use std::str::FromStr;

use itertools::Itertools;
use smallvec::{SmallVec, smallvec};

use crate::j19::Action::{BuildRobot, DoNothing};

const MAX_MINUTES: usize = 24;

#[derive(Clone, Debug)]
struct Node {
    time_left: usize,
    robots: [usize; 4],
    resources: [usize; 4],
    bp: BluePrint,
    priority: [usize; 4],
}

#[derive(Debug, Clone)]
enum Action {
    BuildRobot(usize),
    DoNothing,
}

impl Node {
    pub fn available_actions(&self) -> SmallVec<[Action; 5]> {
        let mut can_build = smallvec![];
        'robot_costs: for idx in 0..4 {
            for resource_idx in 0..3 {
                if self.bp.costs[self.priority[idx]][resource_idx] > self.resources[resource_idx] {
                    continue 'robot_costs;
                }
            }
            can_build.push(BuildRobot(self.priority[idx]));
            return can_build;
        }
        can_build.push(DoNothing);
        can_build
    }

    pub fn is_game_over(&self) -> bool {
        self.time_left == 0
    }

    pub fn act_with_action_id(&mut self, action: &Action) {
        for (robot, resource) in self.robots.iter().zip(self.resources.iter_mut()) {
            *resource += *robot;
        }
        match action {
            BuildRobot(idx) => {
                for (cost, resource) in self.bp.costs[*idx].iter().zip(self.resources.iter_mut()) {
                    *resource -= *cost;
                }
                self.robots[*idx] += 1;
            }
            DoNothing => {}
        }
        self.time_left -= 1;
    }

    fn depth_first_max(&self) -> usize {
        if self.is_game_over() {
            return self.resources[3];
        }

        self.available_actions().iter().map(|action| {
            let mut clone = self.clone();
            clone.act_with_action_id(action);
            clone.depth_first_max()
        }).max().unwrap()
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.robots.hash(state);
        self.resources.hash(state);
    }
}

#[derive(Clone, Debug)]
struct BluePrint {
    costs: [[usize; 3]; 4],
}

impl BluePrint {
    pub fn from_str(line: &str) -> Self {
        let mut splits = line.split('.');
        let mut bp = BluePrint {
            costs: [[0; 3]; 4],
        };
        let mut first_robot = splits.next().unwrap();
        let ore_cost = usize::from_str(first_robot.split("costs ").skip(1)
            .next().unwrap().split(' ').next().unwrap()).unwrap();
        bp.costs[0][0] = ore_cost;

        let mut second_robot = splits.next().unwrap();
        let ore_cost = usize::from_str(second_robot.split("costs ").skip(1)
            .next().unwrap().split(' ').next().unwrap()).unwrap();
        bp.costs[1][0] = ore_cost;

        let mut third_robot = splits.next().unwrap();
        let mut resources = third_robot.split("costs ").skip(1)
            .next().unwrap().split(' ');
        let ore_cost = usize::from_str(resources.next().unwrap()).unwrap();
        let clay_cost = usize::from_str(resources.skip(2).next().unwrap()).unwrap();
        bp.costs[2][0] = ore_cost;
        bp.costs[2][1] = clay_cost;

        let mut fouth_robot = splits.next().unwrap();
        let mut resources = fouth_robot.split("costs ").skip(1)
            .next().unwrap().split(' ');
        let ore_cost = usize::from_str(resources.next().unwrap()).unwrap();
        let obsidian_cost = usize::from_str(resources.skip(2).next().unwrap()).unwrap();
        bp.costs[3][0] = ore_cost;
        bp.costs[3][2] = obsidian_cost;
        bp
    }

    pub fn evaluate(&self) -> usize {
        let mut permutations: SmallVec<[[usize; 4]; 24]> = smallvec![];
        for one in 0..4 {
            for two in 0..4 {
                if one == two {
                    continue
                }
                for three in 0..4 {
                    if one == three || two == three {
                        continue
                    }
                    for four in 0..4 {
                        if one == four || two == four || three == four {
                            continue
                        }
                        permutations.push([one, two, three, four])
                    }
                }
            }
        }
        let mut max = 0;
        for perm in permutations {
            let mut root_node = Node {
                time_left: MAX_MINUTES,
                robots: Default::default(),
                resources: Default::default(),
                bp: self.clone(),
                priority: perm,
            };
            root_node.robots[0] = 1;
            dbg!(root_node.depth_first_max());
            max = max.max(root_node.depth_first_max());
        }
        max
    }
}

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    for line in s.lines() {
        let bp = BluePrint::from_str(line);
        dbg!(bp.evaluate());
    }
    61
}


#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j19.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    for line in s.lines() {}
    42
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j19.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j19_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(42, _p1(include_str!("j19_test.txt")));
        assert_eq!(42, _p1(include_str!("j19.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(42, _p2(include_str!("j19_test.txt")));
        assert_eq!(42, _p2(include_str!("j19.txt")));
    }
}