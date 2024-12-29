#![no_std]
#![allow(clippy::must_use_candidate)]

use core::cmp;

use heapless::{binary_heap, BinaryHeap as HLBinaryHeap};

use bitset::BitSet;

#[cfg(feature = "input")]
pub const INPUT: &str = include_str!("../../input");

#[cfg(not(feature = "input"))]
pub const INPUT: &str = "";

const WIDTH: usize = 140;
const HEIGHT: usize = 140;
const DIMENSION: usize = 5;

type Queue<T> = HLBinaryHeap<T, binary_heap::Max, { 1024 * 4 }>;

#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
#[inline]
fn costs_key((r, c): (u8, u8), (dr, dc): (i8, i8)) -> usize {
    let d = match (dr, dc) {
        (-1, 0) => 0,
        (1, 0) => 1,
        (0, 1) => 2,
        (0, -1) => 3,
        _ => unreachable!(),
    };

    ((((r - 1) as isize * WIDTH as isize + c as isize - 1) << 2) + d) as usize
}

/// # Panics
#[allow(clippy::cast_possible_truncation)]
fn dijkstra(
    costs: &mut [u32],
    maze: &[u8],
    &(height, width): &(usize, usize),
    (start_r, start_c): (u8, u8),
) -> Option<(u32, (u8, u8))> {
    #[derive(Debug, Eq, PartialEq)]
    struct Node {
        cost: u32,
        position: (u8, u8),
        direction: (i8, i8),
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

    let mut queue = Queue::new();
    queue
        .push(Node {
            position: (start_r, start_c),
            direction: (0, 1),
            cost: 0,
        })
        .unwrap();

    while let Some(Node {
        position: (r, c),
        direction: (dr, dc),
        cost,
    }) = queue.pop()
    {
        if cost > costs[costs_key((r, c), (dr, dc))] {
            continue;
        }

        if maze[r as usize * (width + 1) + c as usize] == b'E' {
            return Some((cost, (r, c)));
        }

        for (ndr, ndc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            match (r.checked_add_signed(ndr), c.checked_add_signed(ndc)) {
                (Some(r), Some(c))
                    if (r as usize) < height
                        && (c as usize) < width
                        && maze[r as usize * (width + 1) + c as usize] != b'#' =>
                {
                    let neighbor_cost = if (ndr, ndc) == (dr, dc) {
                        0
                    } else if ndr == -dr || ndc == -dc {
                        continue;
                    } else {
                        1000
                    } + cost
                        + 1;

                    let e = &mut costs[costs_key((r, c), (ndr, ndc))];
                    if *e > neighbor_cost {
                        // I need only the first best
                        *e = neighbor_cost;
                        queue
                            .push(Node {
                                position: (r, c),
                                direction: (ndr, ndc),
                                cost: neighbor_cost,
                            })
                            .unwrap();
                    }
                }
                _ => {}
            }
        }
    }

    None
}

/// # Panics
#[allow(clippy::cast_possible_truncation)]
pub fn solve_1(input: &str) -> u32 {
    let maze = input.as_bytes();
    let width = maze.iter().position(|&c| c == b'\n').unwrap();
    let height = (maze.len() + 1) / (width + 1);

    let (start_r, start_c) = maze
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

    let mut costs = [u32::MAX; { WIDTH * HEIGHT * DIMENSION }];
    costs[costs_key((start_r as u8, start_c as u8), (0i8, 1i8))] = 0u32;

    let (cost, _) = dijkstra(
        &mut costs,
        maze,
        &(height, width),
        (start_r as u8, start_c as u8),
    )
    .unwrap();

    cost
}

/// # Panics
#[allow(clippy::cast_possible_truncation)]
pub fn solve_2(input: &str) -> usize {
    struct Visit<'a, T, K, const N: usize> {
        set: &'a mut BitSet<T, K, N>,
        visited: &'a mut [u32],
        maze: &'a [u8],
        dim: (usize, usize),
        end: (u8, u8),
        best_cost: u32,
    }

    impl<K, const N: usize> Visit<'_, (u8, u8), K, N>
    where
        K: Fn(&(u8, u8)) -> usize,
    {
        fn visit(&mut self, (r, c): (u8, u8), (dr, dc): (i8, i8), cost: u32) -> bool {
            if (r, c) == self.end {
                return true;
            }

            let (height, width) = self.dim;

            let mut found = false;
            for (ndr, ndc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                match (r.checked_add_signed(ndr), c.checked_add_signed(ndc)) {
                    (Some(r), Some(c))
                        if (r as usize) < height
                            && (c as usize) < width
                            && self.maze[r as usize * (width + 1) + c as usize] != b'#' =>
                    {
                        let neighbor_cost = if (ndr, ndc) == (dr, dc) {
                            0
                        } else if ndr == -dr || ndc == -dc {
                            continue;
                        } else {
                            1000
                        } + cost
                            + 1;

                        if neighbor_cost > self.best_cost {
                            continue;
                        }

                        let e = &mut self.visited[costs_key((r, c), (ndr, ndc))];
                        if *e >= neighbor_cost {
                            *e = neighbor_cost;
                            if self.visit((r, c), (ndr, ndc), neighbor_cost) {
                                self.set.insert((r, c)).unwrap();
                                found = true;
                            }
                        }
                    }
                    _ => {}
                }
            }

            found
        }
    }

    let maze = input.as_bytes();
    let width = maze.iter().position(|&c| c == b'\n').unwrap();
    let height = (maze.len() + 1) / (width + 1);

    let (start_r, start_c) = maze
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

    let mut costs = [u32::MAX; { WIDTH * HEIGHT * DIMENSION }];
    costs[costs_key((start_r as u8, start_c as u8), (0i8, 1i8))] = 0u32;

    let (best_cost, (end_r, end_c)) = dijkstra(
        &mut costs,
        maze,
        &(height, width),
        (start_r as u8, start_c as u8),
    )
    .unwrap();

    let mut set =
        BitSet::<(u8, u8), _, { BitSet::with_capacity(WIDTH * HEIGHT) }>::new(|&(r, c)| {
            (r - 1) as usize * width + (c - 1) as usize
        });

    let mut visit = Visit {
        set: &mut set,
        visited: &mut costs,
        maze,
        dim: (height, width),
        end: (end_r, end_c),
        best_cost,
    };

    if visit.visit((start_r as u8, start_c as u8), (0, 1), 0) {
        set.insert((start_r as u8, start_c as u8)).unwrap();
    }

    set.len()
}

#[cfg(feature = "input")]
pub fn part_1() -> u32 {
    solve_1(INPUT)
}

#[cfg(feature = "input")]
pub fn part_2() -> usize {
    solve_2(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = r"###############
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
###############";
    const INPUT_2: &str = r"#################
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
#################";

    #[test]
    fn same_results_1_1() {
        assert_eq!(solve_1(INPUT_1), 7036);
    }

    #[test]
    fn same_results_1_2() {
        assert_eq!(solve_1(INPUT_2), 11048);
    }

    #[test]
    fn same_results_2_1() {
        assert_eq!(solve_2(INPUT_1), 45);
    }

    #[test]
    fn same_results_2_2() {
        assert_eq!(solve_2(INPUT_2), 64);
    }
}
