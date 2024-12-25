#![no_std]
#![allow(clippy::must_use_candidate)]

use heapless::{String as HLString, Vec as HLVec};

#[cfg(feature = "input")]
pub const INPUT: &str = include_str!("../../input");

#[cfg(not(feature = "input"))]
pub const INPUT: &str = "";

type Vec<T> = HLVec<T, 1024>;
type String = HLString<1>;

/// # Panics
pub fn solve_1(input: &str) -> u32 {
    let (mut keys, mut locks) = (Vec::new(), Vec::new());
    for part in input.split("\n\n") {
        let list = if part.starts_with("#####") {
            &mut locks
        } else {
            &mut keys
        };

        let mut acc = [0; 5];
        part.as_bytes().chunks(6).skip(1).take(5).for_each(|row| {
            for (v, &c) in acc.iter_mut().zip(row.iter().take(5)) {
                *v += u8::from(c == b'#');
            }
        });
        list.push(acc).unwrap();
    }

    let mut total = 0;
    for key in keys {
        for lock in &locks {
            total += u32::from(key.iter().zip(lock.iter()).all(|(a, b)| a + b <= 5));
        }
    }

    total
}

/// # Panics
pub fn solve_2(_input: &str) -> String {
    let mut result = String::new();
    result.push('*').unwrap();
    result
}

#[cfg(feature = "input")]
pub fn part_1() -> u32 {
    solve_1(INPUT)
}

#[cfg(feature = "input")]
pub fn part_2() -> String {
    solve_2(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn same_results_1() {
        assert_eq!(solve_1(INPUT), 3);
    }
}
