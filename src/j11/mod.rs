use std::str::FromStr;

use smallvec::{smallvec, SmallVec};

#[derive(Clone)]
enum Operation {
    Plus(u64),
    Times(u64),
    Squared(),
}

#[derive(Clone)]
struct Monkey {
    items: SmallVec<[u64; 30]>,
    operation: Operation,
    test: u64,
    send_true: usize,
    send_false: usize,
    inspections: usize,
}

impl Default for Monkey {
    fn default() -> Self {
        Monkey {
            items: smallvec![],
            operation: Operation::Plus(0),
            test: 0,
            send_true: 0,
            send_false: 0,
            inspections: 0,
        }
    }
}

fn monkey_business(s: &str, divide_by_tree: bool, rounds: usize) -> usize {
    let mut lines = s.lines();
    let mut monkeys: SmallVec<[Monkey; 8]> = smallvec![];
    loop {
        let mut monkey = Monkey::default();
        lines.next().unwrap();
        for number in lines.next().unwrap().split(": ").nth(1).unwrap().split(", ").map(|word| u64::from_str(word).unwrap()) {
            monkey.items.push(number);
        };
        let mut raw_operation = lines.next().unwrap().split("old ").nth(1).unwrap().split(' ');
        monkey.operation = match (raw_operation.next().unwrap(), u64::from_str(raw_operation.next().unwrap())) {
            ("*", Ok(num)) => Operation::Times(num),
            ("+", Ok(num)) => Operation::Plus(num),
            ("*", _) => Operation::Squared(),
            _ => panic!()
        };
        monkey.test = u64::from_str(lines.next().unwrap().split("by ").nth(1).unwrap()).unwrap();
        monkey.send_true = usize::from_str(lines.next().unwrap().split("monkey ").nth(1).unwrap()).unwrap();
        monkey.send_false = usize::from_str(lines.next().unwrap().split("monkey ").nth(1).unwrap()).unwrap();

        monkeys.push(monkey);
        match lines.next() {
            None => { break; }
            Some(_) => {}
        }
    }

    let monkey_mod: u64 = monkeys.iter().map(|m| m.test).product();

    for _ in 1..=rounds {
        let mut next_monkeys = monkeys.clone();
        for monkey_idx in 0..monkeys.len() {
            monkeys[monkey_idx] = next_monkeys[monkey_idx].clone();
            let monkey = &monkeys[monkey_idx];
            for item_idx in monkey.items.iter() {
                next_monkeys[monkey_idx].inspections += 1;
                let mut worry = *item_idx;
                worry = match &monkey.operation {
                    Operation::Plus(num) => {
                        worry + num
                    }
                    Operation::Times(num) => {
                        (worry) * (num)
                    }
                    Operation::Squared() => {
                        worry * worry
                    }
                };

                if divide_by_tree {
                    worry /= 3;
                } else {
                    worry %= monkey_mod;
                }

                if worry % monkey.test == 0 {
                    next_monkeys[monkey.send_true].items.push(worry);
                } else {
                    next_monkeys[monkey.send_false].items.push(worry);
                }
            }
            next_monkeys[monkey_idx].items.clear();
        }
        std::mem::swap(&mut monkeys, &mut next_monkeys);
    }

    let (first, second) = monkeys.into_iter().map(|monkey| monkey.inspections).fold((0, 0),
                                                                                    |(first, second), elt| {
                                                                                        if elt < second {
                                                                                            (first, second)
                                                                                        } else if elt < first {
                                                                                            (first, elt)
                                                                                        } else {
                                                                                            (elt, first)
                                                                                        }
                                                                                    });
    first * second
}

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    monkey_business(s, true, 20)
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j11.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    monkey_business(s, false, 10000)
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j11.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j11_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(10605, _p1(include_str!("j11_test.txt")));
        assert_eq!(108240, _p1(include_str!("j11.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(2713310158, _p2(include_str!("j11_test.txt")));
        assert_eq!(25712998901, _p2(include_str!("j11.txt")));
    }
}