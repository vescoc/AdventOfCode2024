#![no_std]
#![allow(clippy::must_use_candidate)]

#[cfg(not(target_family = "wasm"))]
use rayon::prelude::*;

use heapless::Deque;

use bitset::BitSet as VBitSet;

use lazy_static::lazy_static;

type VecDeque<T> = Deque<T, 32>;
type BitSet<T, K> = VBitSet<T, K, { VBitSet::with_capacity(64 * 64) }>;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

/// (rows x columns)
type Point = (usize, usize);

/// # Panics
fn solve<F>(input: &str, find_paths: F) -> usize
where
    F: Fn(&Point, &[u8], Point) -> usize + Copy + Sync + Send,
{
    let map = input.as_bytes();
    let width = map.iter().position(|&c| c == b'\n').unwrap();
    let height = (map.len() + 1) / (width + 1);

    #[cfg(not(target_family = "wasm"))]
    let rows = map.par_chunks(width + 1).take(height);

    #[cfg(target_family = "wasm")]
    let rows = map.chunks(width + 1).take(height);

    rows.enumerate()
        .map(|(r, row)| {
            row.iter()
                .take(width)
                .enumerate()
                .map(move |(c, &tile)| {
                    if tile == b'0' {
                        find_paths(&(height, width), map, (r, c))
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

/// # Panics
pub fn solve_1(input: &str) -> usize {
    solve(input, |&(height, width), map, (r, c)| {
        let mut count = 0;
        let mut visited = BitSet::new(|(r, c)| r * (width + 1) + c);
        visited.insert((r, c)).unwrap();

        let mut queue = VecDeque::new();
        queue.push_back((r, c)).unwrap();
        while let Some((r, c)) = queue.pop_front() {
            let tile = map[r * (width + 1) + c];
            if tile == b'9' {
                count += 1;
            } else {
                for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    match (r.checked_add_signed(dr), c.checked_add_signed(dc)) {
                        (Some(r), Some(c))
                            if r < width && c < height && map[r * (width + 1) + c] == tile + 1 =>
                        {
                            if !visited.insert((r, c)).unwrap() {
                                queue.push_back((r, c)).unwrap();
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        count
    })
}

/// # Panics
pub fn solve_2(input: &str) -> usize {
    solve(input, |&(height, width), map, (r, c)| {
        let mut count = 0;
        let mut queue = VecDeque::new();
        queue.push_back((r, c)).unwrap();
        while let Some((r, c)) = queue.pop_front() {
            let tile = map[r * (width + 1) + c];
            if tile == b'9' {
                count += 1;
            } else {
                for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    match (r.checked_add_signed(dr), c.checked_add_signed(dc)) {
                        (Some(r), Some(c))
                            if r < width && c < height && map[r * (width + 1) + c] == tile + 1 =>
                        {
                            queue.push_back((r, c)).unwrap();
                        }
                        _ => {}
                    }
                }
            }
        }
        count
    })
}

pub fn part_1() -> usize {
    solve_1(&INPUT)
}

pub fn part_2() -> usize {
    solve_2(&INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref INPUT_1: &'static str = r#"0123
1234
8765
9876"#;
        static ref INPUT_2: &'static str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;
    }

    #[test]
    fn same_results_1_1() {
        assert_eq!(solve_1(&INPUT_1), 1);
    }

    #[test]
    fn same_results_1_2() {
        assert_eq!(solve_1(&INPUT_2), 36);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&INPUT_2), 81);
    }
}
