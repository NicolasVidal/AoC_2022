use std::hash::{Hash, Hasher};
use std::str::FromStr;

use smallvec::{SmallVec, smallvec};

use crate::j19::Action::{BuildRobot, DoNothing};

#[derive(Clone, Debug)]
struct Node {
    time_left: usize,
    robots: [usize; 4],
    resources: [usize; 4],
    bp: BluePrint,
    #[allow(unused)]
    strategy: [usize; 4],
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Action {
    BuildRobot(usize),
    DoNothing,
}


impl Node {
    #[allow(unused)]
    pub fn compute_time_steps_to_get_robot<const MAX_MINUTES: usize>(&self, idx: usize) -> usize {
        let costs = self.bp.costs[idx];
        let production = self.robots;
        let mut resources = self.resources;

        for time_step in 0..MAX_MINUTES {
            if resources.iter().zip(costs.iter()).all(|(r, c)| *r >= *c) {
                return time_step;
            }
            for idx in 0..4 {
                resources[idx] += production[idx];
            }
        }

        usize::MAX
    }

    pub fn available_actions(&self) -> SmallVec<[Action; 5]> {
        let mut can_build = smallvec![];

        'robot_costs: for robot_idx in (0..4).rev() {
            for resource_idx in 0..3 {
                if self.bp.costs[robot_idx][resource_idx] > self.resources[resource_idx] {
                    continue 'robot_costs;
                }
            }

            can_build.push(BuildRobot(robot_idx));
            if robot_idx == 3
            {
                return can_build;
            }
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

    fn depth_first_max(&self, best_score: &mut usize) -> usize {
        if self.is_game_over() {
            if self.resources[3] > *best_score {
                *best_score = self.resources[3];
            }
            return self.resources[3];
        }

        if (0..self.time_left).sum::<usize>() +
            self.time_left * self.robots[3] +
            self.resources[3] < *best_score {
            return self.resources[3];
        }

        self.available_actions().iter().map(|action| {
            let mut clone = self.clone();
            clone.act_with_action_id(action);
            clone.depth_first_max(best_score)
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
        let first_robot = splits.next().unwrap();
        let ore_cost = usize::from_str(first_robot.split("costs ").nth(1).unwrap().split(' ').next().unwrap()).unwrap();
        bp.costs[0][0] = ore_cost;

        let second_robot = splits.next().unwrap();
        let ore_cost = usize::from_str(second_robot.split("costs ").nth(1).unwrap().split(' ').next().unwrap()).unwrap();
        bp.costs[1][0] = ore_cost;

        let third_robot = splits.next().unwrap();
        let mut resources = third_robot.split("costs ").nth(1).unwrap().split(' ');
        let ore_cost = usize::from_str(resources.next().unwrap()).unwrap();
        let clay_cost = usize::from_str(resources.nth(2).unwrap()).unwrap();
        bp.costs[2][0] = ore_cost;
        bp.costs[2][1] = clay_cost;

        let fouth_robot = splits.next().unwrap();
        let mut resources = fouth_robot.split("costs ").nth(1).unwrap().split(' ');
        let ore_cost = usize::from_str(resources.next().unwrap()).unwrap();
        let obsidian_cost = usize::from_str(resources.nth(2).unwrap()).unwrap();
        bp.costs[3][0] = ore_cost;
        bp.costs[3][2] = obsidian_cost;
        bp
    }

    pub fn evaluate<const MAX_MINUTES: usize>(&self) -> usize {
        let mut root_node = Node {
            time_left: MAX_MINUTES,
            robots: Default::default(),
            resources: Default::default(),
            bp: self.clone(),
            strategy: [0, 0, 0, 0],
        };
        root_node.robots[0] = 1;
        let mut best_score = 0usize;
        root_node.depth_first_max(&mut best_score)
    }
}

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut sum = 0;
    for (idx, line) in s.lines().enumerate() {
        let bp = BluePrint::from_str(line);
        let eval = dbg!(bp.evaluate::<24>());
        sum += (idx + 1) * eval
    }
    sum
}


#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j19.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let mut product = 1;
    for (idx, line) in s.lines().enumerate().take(3) {
        let bp = BluePrint::from_str(line);
        let eval = dbg!(bp.evaluate::<32>());
        product *= eval
    }
    product
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
        assert_eq!(33, _p1(include_str!("j19_test.txt")));
        assert_eq!(1262, _p1(include_str!("j19.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(3472, _p2(include_str!("j19_test.txt")));
        assert_eq!(37191, _p2(include_str!("j19.txt")));
    }
}