#![allow(clippy::must_use_candidate)]

use core::cmp;

use std::collections::{BinaryHeap, HashMap};

use bitset::BitSet;

#[cfg(feature = "input")]
use lazy_static::lazy_static;

#[cfg(feature = "input")]
lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

#[cfg(not(feature = "input"))]
pub const INPUT: &str = "";

/// # Panics
pub fn solve_1(input: &str) -> u32 {
    #[derive(Eq, PartialEq)]
    struct Node {
        position: (usize, usize),
        direction: (isize, isize),
        cost: u32,
    }

    impl Ord for Node {
        fn cmp(&self, other: &Self) -> cmp::Ordering {
            other.cost.cmp(&self.cost)
        }
    }

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    let maze = input.as_bytes();
    let width = maze.iter().position(|&c| c == b'\n').unwrap();
    let height = (maze.len() + 1) / (width + 1);

    let (r, c) = maze
        .chunks(width + 1)
        .take(height)
        .enumerate()
        .find_map(|(r, row)| {
            row.iter()
                .take(width)
                .position(|&tile| tile == b'S')
                .map(|c| (r, c))
        })
        .expect("invalid maze: cannot find `S`");

    let mut costs = HashMap::with_capacity(140 * 140);
    costs.insert(((r, c), (0, 1)), 0);

    let mut queue = BinaryHeap::with_capacity(4096);
    queue.push(Node {
        position: (r, c),
        direction: (0, 1),
        cost: 0,
    });

    while let Some(Node {
        position: (r, c),
        direction: (dr, dc),
        cost,
    }) = queue.pop()
    {
        if cost > costs.get(&((r, c), (dr, dc))).copied().unwrap_or(u32::MAX) {
            continue;
        }

        if maze[r * (width + 1) + c] == b'E' {
            return cost;
        }

        for (ndr, ndc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            match (r.checked_add_signed(ndr), c.checked_add_signed(ndc)) {
                (Some(r), Some(c))
                    if r < height && c < width && maze[r * (width + 1) + c] != b'#' =>
                {
                    let neighbor_cost = if (ndr, ndc) == (dr, dc) {
                        0
                    } else if ndr == -dr || ndc == -dc {
                        2000
                    } else {
                        1000
                    } + cost
                        + 1;

                    let e = costs.entry(((r, c), (ndr, ndc))).or_insert(u32::MAX);
                    if *e > neighbor_cost {
                        // I need only the first best
                        *e = neighbor_cost;
                        queue.push(Node {
                            position: (r, c),
                            direction: (ndr, ndc),
                            cost: neighbor_cost,
                        });
                    }
                }
                _ => {}
            }
        }
    }

    unreachable!()
}

/// # Panics
pub fn solve_2(input: &str) -> usize {
    type Set<T, K> = BitSet<T, K, { BitSet::with_capacity(141 * 141) }>;

    struct Node<T, K> {
        position: (usize, usize),
        direction: (isize, isize),
        cost: u32,
        path: Box<Set<T, K>>,
    }

    impl<T, K> PartialEq for Node<T, K> {
        fn eq(&self, other: &Self) -> bool {
            self.position == other.position && self.direction == other.direction
        }
    }

    impl<T, K> Eq for Node<T, K> {}

    impl<T, K> Ord for Node<T, K> {
        fn cmp(&self, other: &Self) -> cmp::Ordering {
            other.cost.cmp(&self.cost)
        }
    }

    impl<T, K> PartialOrd for Node<T, K> {
        fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    let maze = input.as_bytes();
    let width = maze.iter().position(|&c| c == b'\n').unwrap();
    let height = (maze.len() + 1) / (width + 1);

    let (r, c) = maze
        .chunks(width + 1)
        .take(height)
        .enumerate()
        .find_map(|(r, row)| {
            row.iter()
                .take(width)
                .position(|&tile| tile == b'S')
                .map(|c| (r, c))
        })
        .expect("invalid maze: cannot find `S`");

    let key = |(r, c): &(usize, usize)| r * width + c;

    let mut best_tiles = Set::new(key);
    best_tiles.insert((r, c)).unwrap();

    let mut best_cost = u32::MAX;

    let mut costs = HashMap::with_capacity(140 * 140);
    costs.insert(((r, c), (0, 1)), 0);

    let mut path = Set::new(key);
    path.insert((r, c)).unwrap();

    let mut queue = BinaryHeap::with_capacity(4096);
    queue.push(Node {
        position: (r, c),
        direction: (0, 1),
        cost: 0,
        path: Box::new(path),
    });

    while let Some(Node {
        position: (r, c),
        direction: (dr, dc),
        cost,
        path,
    }) = queue.pop()
    {
        if cost > best_cost || cost > costs.get(&((r, c), (dr, dc))).copied().unwrap_or(u32::MAX) {
            continue;
        }

        if maze[r * (width + 1) + c] == b'E' && cost <= best_cost {
            best_cost = cost;

            best_tiles |= *path;

            continue;
        }

        for (ndr, ndc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            match (r.checked_add_signed(ndr), c.checked_add_signed(ndc)) {
                (Some(r), Some(c))
                    if r < height && c < width && maze[r * (width + 1) + c] != b'#' =>
                {
                    let neighbor_cost = if (ndr, ndc) == (dr, dc) {
                        0
                    } else if ndr == -dr || ndc == -dc {
                        2000
                    } else {
                        1000
                    } + cost
                        + 1;

                    let e = costs.entry(((r, c), (ndr, ndc))).or_insert(u32::MAX);
                    if *e >= neighbor_cost {
                        *e = neighbor_cost;

                        let mut path = path.clone();
                        path.insert((r, c)).unwrap();

                        queue.push(Node {
                            position: (r, c),
                            direction: (ndr, ndc),
                            cost: neighbor_cost,
                            path,
                        });
                    }
                }
                _ => {}
            }
        }
    }

    best_tiles.len()
}

#[cfg(feature = "input")]
pub fn part_1() -> u32 {
    solve_1(&INPUT)
}

#[cfg(feature = "input")]
pub fn part_2() -> usize {
    solve_2(&INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref INPUT_1: &'static str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;
        static ref INPUT_2: &'static str = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;
    }

    #[test]
    fn same_results_1_1() {
        assert_eq!(solve_1(&INPUT_1), 7036);
    }

    #[test]
    fn same_results_1_2() {
        assert_eq!(solve_1(&INPUT_2), 11048);
    }

    #[test]
    fn same_results_2_1() {
        assert_eq!(solve_2(&INPUT_1), 45);
    }

    #[test]
    fn same_results_2_2() {
        assert_eq!(solve_2(&INPUT_2), 64);
    }
}
