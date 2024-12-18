#![no_std]
#![allow(clippy::must_use_candidate)]

use core::fmt::Write;

use heapless::{Deque as HLDeque, String as HLString, Vec as HLVec};

use bitset::BitSet;

#[cfg(feature = "input")]
use lazy_static::lazy_static;

#[cfg(feature = "input")]
lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

#[cfg(not(feature = "input"))]
pub const INPUT: &str = "";

pub const PUZZLE_WIDTH: usize = 71;
pub const PUZZLE_HEIGHT: usize = 71;
pub const BITSET_SIZE: usize = BitSet::with_capacity(PUZZLE_WIDTH * PUZZLE_HEIGHT);

type Deque<T> = HLDeque<T, { PUZZLE_WIDTH * 2 }>;
type Vec<T> = HLVec<T, { PUZZLE_WIDTH * PUZZLE_HEIGHT }>;
type String = HLString<16>;

/// # Panics
pub fn solve_1_bfs<
    const WIDTH: usize,
    const HEIGHT: usize,
    const TAKE: usize,
    const SIZE: usize,
>(
    input: &str,
) -> usize {
    let mut map = [[b'.'; WIDTH]; HEIGHT];
    for (x, y) in input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .take(TAKE)
    {
        map[y][x] = b'#';
    }

    let mut visited = BitSet::<_, _, SIZE>::new(|(x, y)| y * WIDTH + x);
    visited.insert((0, 0)).unwrap();

    let mut queue = Deque::new();
    queue.push_back(((0, 0), 0)).unwrap();

    while let Some(((x, y), steps)) = queue.pop_front() {
        if x == WIDTH - 1 && y == HEIGHT - 1 {
            return steps;
        }

        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            match (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
                (Some(x), Some(y))
                    if matches!(map.get(y).and_then(|row| row.get(x)), Some(b'.')) =>
                {
                    if !visited.insert((x, y)).unwrap() {
                        queue.push_back(((x, y), steps + 1)).unwrap();
                    }
                }
                _ => {}
            }
        }
    }

    unreachable!()
}

/// # Panics
pub fn bfs<const WIDTH: usize, const HEIGHT: usize, const SIZE: usize>(
    map: &[[u8; WIDTH]; HEIGHT],
) -> bool {
    let key = |(x, y): &(usize, usize)| y * WIDTH + x;

    let mut visited = BitSet::<_, _, SIZE>::new(key);
    visited.insert((0, 0)).unwrap();

    let mut queue = Deque::new();
    queue.push_back((0usize, 0usize)).unwrap();

    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            match (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
                (Some(x), Some(y))
                    if matches!(map.get(y).and_then(|row| row.get(x)), Some(b'.')) =>
                {
                    if x == WIDTH - 1 && y == HEIGHT - 1 {
                        return true;
                    }

                    if !visited.insert((x, y)).unwrap() {
                        queue.push_back((x, y)).unwrap();
                    }
                }
                _ => {}
            }
        }
    }

    false
}

/// # Panics
pub fn dfs<const WIDTH: usize, const HEIGHT: usize, const SIZE: usize>(
    map: &[[u8; WIDTH]; HEIGHT],
) -> bool {
    let key = |(x, y): &(usize, usize)| y * WIDTH + x;

    let mut visited = BitSet::<_, _, SIZE>::new(key);
    visited.insert((0, 0)).unwrap();

    let mut queue = Vec::new();
    queue.push((0usize, 0usize)).unwrap();

    while let Some((x, y)) = queue.pop() {
        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            match (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
                (Some(x), Some(y))
                    if matches!(map.get(y).and_then(|row| row.get(x)), Some(b'.')) =>
                {
                    if x == WIDTH - 1 && y == HEIGHT - 1 {
                        return true;
                    }

                    if !visited.insert((x, y)).unwrap() {
                        queue.push((x, y)).unwrap();
                    }
                }
                _ => {}
            }
        }
    }

    false
}

/// # Panics
pub fn solve_2_bs<const WIDTH: usize, const HEIGHT: usize, const SIZE: usize, const CUT: usize>(
    input: &str,
    search: impl Fn(&[[u8; WIDTH]; HEIGHT]) -> bool,
) -> String {
    let drops = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut min = CUT;
    let mut max = drops.len();
    let mut middle = (min + max) / 2;

    let mut grid = [[b'.'; WIDTH]; HEIGHT];
    for (x, y) in drops.iter().take(middle).copied() {
        grid[y][x] = b'#';
    }

    while min != middle {
        if search(&grid) {
            let new_middle = (middle + max) / 2;
            for (x, y) in drops.iter().skip(middle).take(new_middle - middle).copied() {
                grid[y][x] = b'#';
            }
            min = middle;
            middle = new_middle;
        } else {
            let new_middle = (min + middle) / 2;
            for (x, y) in drops
                .iter()
                .skip(new_middle)
                .take(middle - new_middle)
                .copied()
            {
                grid[y][x] = b'.';
            }
            max = middle;
            middle = new_middle;
        }
    }

    let (x, y) = drops[middle];

    let mut result = String::new();
    write!(&mut result, "{x},{y}").unwrap();

    result
}

pub fn solve_1(input: &str) -> usize {
    solve_1_bfs::<PUZZLE_WIDTH, PUZZLE_HEIGHT, 1024, BITSET_SIZE>(input)
}

pub fn solve_2(input: &str) -> String {
    solve_2_bs::<PUZZLE_WIDTH, PUZZLE_HEIGHT, BITSET_SIZE, 1024>(
        input,
        dfs::<PUZZLE_WIDTH, PUZZLE_HEIGHT, BITSET_SIZE>,
    )
}

#[cfg(feature = "input")]
pub fn part_1() -> usize {
    solve_1(&INPUT)
}

#[cfg(feature = "input")]
pub fn part_2() -> String {
    solve_2(&INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    use lazy_static::lazy_static;

    lazy_static! {
        static ref INPUT: &'static str = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#;
    }

    #[test]
    fn same_results_1() {
        assert_eq!(
            solve_1_bfs::<7, 7, 12, { BitSet::with_capacity(7 * 7) }>(&INPUT),
            22
        );
    }

    #[test]
    fn same_results_2_bs_dfs() {
        assert_eq!(
            &solve_2_bs::<7, 7, { BitSet::with_capacity(7 * 7) }, 0>(
                &INPUT,
                dfs::<7, 7, { BitSet::with_capacity(7 * 7) }>
            ),
            &"6,1"
        );
    }

    #[test]
    fn same_results_2_bs_bfs() {
        assert_eq!(
            &solve_2_bs::<7, 7, { BitSet::with_capacity(7 * 7) }, 0>(
                &INPUT,
                bfs::<7, 7, { BitSet::with_capacity(7 * 7) }>
            ),
            &"6,1"
        );
    }
}
