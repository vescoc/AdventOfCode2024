#![no_std]
#![allow(clippy::must_use_candidate)]

use heapless::{Entry, FnvIndexMap};

type Map<K, T> = FnvIndexMap<K, T, 4096>;

#[cfg(feature = "input")]
pub const INPUT: &str = include_str!("../../input");

#[cfg(not(feature = "input"))]
pub const INPUT: &str = &"";

struct Stones(Map<u64, u64>);

impl Stones {
    const fn new() -> Self {
        Self(Map::new())
    }

    fn clear(&mut self) {
        self.0.clear();
    }

    fn add(&mut self, stone: u64, count: u64) {
        match self.0.entry(stone) {
            Entry::Vacant(v) => {
                v.insert(count).unwrap();
            }
            Entry::Occupied(mut v) => {
                *v.get_mut() += count;
            }
        }
    }

    fn values(&self) -> impl Iterator<Item = &u64> {
        self.0.values()
    }

    fn iter(&self) -> impl Iterator<Item = (&u64, &u64)> {
        self.0.iter()
    }
}

fn split_if_even(stone: u64) -> Option<(u64, u64)> {
    stone.checked_ilog10().and_then(|digits| {
        if digits % 2 == 1 {
            let div = 10_u64.pow((digits + 1) / 2);
            Some((stone / div, stone % div))
        } else {
            None
        }
    })
}

/// # Panics
fn solve(input: &str, mut blinks: usize) -> u64 {
    let mut stones = const {
        [
            Stones::new(),
            Stones::new(),
        ]
    };

    for (k, v) in input
        .split_whitespace()
        .map(|stone| (stone.parse::<u64>().unwrap(), 1))
    {
        stones[0].add(k, v);
    }
    
    let mut i = 0;
    while blinks > 0 {
        i = (i + 1) % 2;
        let (new_stones, old_stones) = {
            let (a, b) = stones.split_at_mut(1);
            if i == 0 {
                (&mut a[0], &b[0])
            } else {
                (&mut b[0], &a[0])
            }
        };

        new_stones.clear();
        for (&stone, &n) in old_stones.iter() {
            if stone == 0 {
                new_stones.add(1, n);
            } else if let Some((a, b)) = split_if_even(stone) {
                new_stones.add(a, n);
                new_stones.add(b, n);
            } else {
                new_stones.add(stone * 2024, n);
            }
        }

        blinks -= 1;
    }

    stones[i].values().sum()
}

/// # Panics
pub fn solve_1(input: &str) -> u64 {
    solve(input, 25)
}

/// # Panics
pub fn solve_2(input: &str) -> u64 {
    solve(input, 75)
}

#[cfg(feature = "input")]
pub fn part_1() -> u64 {
    solve_1(INPUT)
}

#[cfg(feature = "input")]
pub fn part_2() -> u64 {
    solve_2(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"125 17";

    #[test]
    fn test_split_if_even() {
        assert_eq!(split_if_even(1234), Some((12, 34)));
        assert_eq!(split_if_even(12), Some((1, 2)));
    }

    #[test]
    fn same_results_1() {
        assert_eq!(solve_1(INPUT), 55312);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(INPUT, 25), 55312);
    }
}
