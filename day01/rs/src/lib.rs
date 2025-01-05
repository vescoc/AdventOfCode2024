#![no_std]
#![allow(clippy::must_use_candidate)]

use heapless::{Entry, FnvIndexMap, Vec as HLVec};

type HashMap<K, V> = FnvIndexMap<K, V, 1024>;
type Vec<T> = HLVec<T, 1024>;

#[cfg(feature = "input")]
pub const INPUT: &str = include_str!("../../input");
    
#[cfg(not(feature = "input"))]
pub const INPUT: &str = "";

struct CountHashMap<K, T>(HashMap<K, T>);

impl<K, V> CountHashMap<K, V> {
    const fn new() -> Self {
        Self(HashMap::new())
    }
}

impl<K> CountHashMap<K, u32>
where K: Eq + core::hash::Hash,
{
    fn add(&mut self, value: K) -> Result<(), u32> {
        match self.0.entry(value) {
            Entry::Occupied(mut v) => {
                *v.get_mut() += 1;
                Ok(())
            }
            Entry::Vacant(v) => {
                v.insert(1)?;
                Ok(())
            }
        }
    }
}

impl<K, T> core::ops::Deref for CountHashMap<K, T> {
    type Target = HashMap<K, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// # Panics
#[cfg_attr(target_os = "none", inline(never))]
pub fn solve_1(input: &str) -> u32 {
    let mut line1 = Vec::new();
    let mut line2 = Vec::new();
    for (v1, v2) in input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();

            (
                parts.next().unwrap().parse::<u32>().unwrap(),
                parts.next().unwrap().parse::<u32>().unwrap(),
            )
        })
    {
        line1.push(v1).unwrap();
        line2.push(v2).unwrap();
    }

    line1.sort_unstable();
    line2.sort_unstable();

    line1
        .iter()
        .zip(&line2)
        .map(|(a, &b)| a.abs_diff(b))
        .sum()
}

/// # Panics
#[cfg_attr(target_os = "none", inline(never))]
pub fn solve_2(input: &str) -> u32 {
    let mut line1 = Vec::new();
    let mut line2 = CountHashMap::new();
    for (v1, v2) in input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();

            (
                parts.next().unwrap().parse::<u32>().unwrap(),
                parts.next().unwrap().parse::<u32>().unwrap(),
            )
        })
    {
        line1.push(v1).unwrap();
        line2.add(v2).unwrap();
    }

    line1
        .iter()
        .map(|id| id * line2.get(id).copied().unwrap_or_default())
        .sum()
}

#[cfg(feature = "input")]
pub fn part_1() -> u32 {
    solve_1(INPUT)
}

#[cfg(feature = "input")]
pub fn part_2() -> u32 {
    solve_2(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = r"3   4
4   3
2   5
1   3
3   9
3   3";
    const INPUT_2: &str = r"3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn same_results_1() {
        assert_eq!(solve_1(INPUT_1), 11);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(INPUT_2), 31);
    }
}
