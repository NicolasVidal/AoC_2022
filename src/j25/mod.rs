use std::fmt::{Display, Formatter, Write};

use rand::{Rng, SeedableRng};
use rand::prelude::SliceRandom;
use rand_xoshiro::Xoshiro256PlusPlus;
use smallvec::SmallVec;

use crate::j25::SnafuChar::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum SnafuChar {
    Two,
    One,
    Zero,
    Minus,
    DoubleMinus,
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Snafu(SmallVec<[SnafuChar; 20]>);

impl Display for Snafu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for c in self.0.iter().rev() {
            f.write_char(match c {
                Two => { '2' }
                One => { '1' }
                Zero => { '0' }
                Minus => { '-' }
                DoubleMinus => { '=' }
            })?;
        }
        Ok(())
    }
}

impl Snafu {
    #[allow(unused)]
    pub fn from_str(s: &str) -> Snafu {
        let mut snafu: Snafu = Default::default();
        for c in s.chars() {
            snafu.0.push(match c {
                '2' => Two,
                '1' => One,
                '0' => Zero,
                '-' => Minus,
                '=' => DoubleMinus,
                _ => panic!()
            });
        }
        snafu.0.reverse();
        snafu
    }

    #[allow(unused)]
    pub fn to_decimal(&self) -> isize {
        let mut result = 0;
        for (power, num) in self.0.iter().enumerate() {
            result += 5isize.pow(power as u32) * match num {
                Two => { 2 }
                One => { 1 }
                Zero => { 0 }
                Minus => { -1 }
                DoubleMinus => { -2 }
            };
        }

        result
    }

    pub fn from_random<R: Rng + ?Sized>(rng: &mut R) -> Snafu {
        let mut snafu = Snafu::default();
        for _ in 0..snafu.0.capacity() {
            snafu.0.push(*[DoubleMinus, Minus, Zero, One, Two].choose(rng).unwrap());
        }
        snafu
    }

    pub fn trim(&mut self) {
        while let Some(elt) = self.0.last() {
            if *elt != Zero {
                break;
            }
            self.0.pop();
        }
    }


    pub fn trim_self(mut self) -> Self {
        self.trim();
        self
    }
}

pub fn genetic_algorithm(total: isize) -> Snafu {
    const POP_SIZE: usize = 2000;
    const MUTATION_RATE: f32 = 0.4f32;
    const BREEDERS_PERCENTAGE: f32 = 0.25f32;
    const FORCED_DIVERSITY_PERCENTAGE: f32 = 0.3f32;
    const BREEDERS_COUNT: usize = (POP_SIZE as f32 * BREEDERS_PERCENTAGE) as usize;
    const FORCED_DIVERSITY_COUNT: usize = (POP_SIZE as f32 * FORCED_DIVERSITY_PERCENTAGE) as usize;
    const ELITISM_PERCENTAGE: f32 = 0.20f32;
    const ELITISM_COUNT: usize = (POP_SIZE as f32 * ELITISM_PERCENTAGE) as usize;
    let mut rng = Xoshiro256PlusPlus::from_entropy();

    let mut population = [(); POP_SIZE]
        .map(|()| Snafu::from_random(&mut rng));
    let mut scores = [0f32; POP_SIZE];

    let mut breeders = [(); BREEDERS_COUNT].map(|()| Snafu::from_random(&mut rng));

    let mut population_idx = [0usize; POP_SIZE];
    for (idx, index) in population_idx.iter_mut().enumerate().take(POP_SIZE) {
        *index = idx;
    }

    let mut best_score = f32::MAX;
    loop {
        for i in 0..POP_SIZE {
            scores[i] = (total - population[i].to_decimal()).abs() as f32;
            if scores[i] == 0f32 {
                return population[i].clone();
            }
            if scores[i] < best_score {
                best_score = scores[i];
            }
        }

        population_idx.sort_unstable_by_key(|elt| scores[*elt] as usize);

        // Elitism
        for (i, index) in population_idx.iter().enumerate().take(POP_SIZE) {
            population[i] = population[*index].clone();
        }

        // Forced Diversity
        for solution in population.iter_mut().take(FORCED_DIVERSITY_COUNT).skip(ELITISM_COUNT) {
            *solution = Snafu::from_random(&mut rng);
        }

        let total_scores = scores.iter().sum::<f32>();

        for score in scores.iter_mut().take(POP_SIZE) {
            *score = 1.0f32 - *score / total_scores;
        }

        for i in 0..BREEDERS_COUNT {
            // This should be better but would cause Heap Allocations, sigh
            // breeders[i] = population[*population_idx.choose_weighted(&mut rng, |elt| scores[*elt]).unwrap()].clone();
            breeders[i] = population[population_idx[i]].clone();
        }

        // Crossover
        for i in (FORCED_DIVERSITY_COUNT + ELITISM_COUNT)..POP_SIZE {
            let parents = [population.choose(&mut rng).unwrap().clone(),
                population.choose(&mut rng).unwrap().clone()];
            for (c_idx, c) in population[i].0.iter_mut().enumerate() {
                *c = parents[rng.gen_range(0..2)].0[c_idx];
            }
        }

        // Mutation
        for solution in population.iter_mut().take(POP_SIZE).skip(FORCED_DIVERSITY_COUNT + ELITISM_COUNT) {
            if rng.gen_range(0f32..1f32) < MUTATION_RATE {
                *solution.0.choose_mut(&mut rng).unwrap() = *[
                    DoubleMinus,
                    Minus,
                    Zero,
                    One,
                    Two,
                ].choose(&mut rng).unwrap();
            }
        }
    }
}

#[allow(unused)]
pub fn _p1(s: &str) -> Snafu {
    let mut total = 0isize;
    for line in s.lines() {
        total += Snafu::from_str(line).to_decimal();
    }
    genetic_algorithm(total).trim_self()
}

#[allow(unused)]
pub fn p1() -> Snafu {
    _p1(include_str!("j25.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j25_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(Snafu::from_str("2=-1=0"), _p1(include_str!("j25_test.txt")));
        assert_eq!(Snafu::from_str("2-0-01==0-1=2212=100"), _p1(include_str!("j25.txt")));
    }
}