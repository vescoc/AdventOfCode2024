#![no_std]
#![allow(clippy::must_use_candidate)]

use bitset::BitSet;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

const VISITED_SIZE: usize = BitSet::with_capacity(131 * 131);
const VISITED_STATE_SIZE: usize = BitSet::with_capacity(131 * 131 * 5);

#[cfg(feature = "input")]
use lazy_static::lazy_static;

#[cfg(feature = "input")]
lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

#[cfg(not(feature = "input"))]
pub const INPUT: &str = "";

/// # Panics
pub fn solve_1(input: &str) -> usize {
    let map = input.as_bytes();
    let width = map.iter().position(|&c| c == b'\n').unwrap();
    let height = (map.len() + 1) / (width + 1);

    let current_position = map.iter().position(|&c| c == b'^').unwrap();
    let mut current_position = (
        current_position / (width + 1),
        current_position % (width + 1),
    );

    let mut visited = BitSet::<_, _, VISITED_SIZE>::new(|(r, c)| r * width + c);

    let mut facing = 0;
    loop {
        visited.insert(current_position).unwrap();
        if current_position.0 == 0
            || current_position.1 == 0
            || current_position.0 == width - 1
            || current_position.1 == height - 1
        {
            return visited.len();
        }

        (facing, current_position) = [(-1, 0), (0, 1), (1, 0), (0, -1)]
            .into_iter()
            .enumerate()
            .cycle()
            .skip(facing)
            .take(4)
            .find_map(|(facing, (dr, dc))| {
                let (r, c) = (
                    current_position.0.checked_add_signed(dr).unwrap(),
                    current_position.1.checked_add_signed(dc).unwrap(),
                );

                if map[r * (width + 1) + c] == b'#' {
                    None
                } else {
                    Some((facing, (r, c)))
                }
            })
            .unwrap();
    }
}

fn is_cycle<const SIZE: usize, K>(
    map: &[u8],
    (height, width): (usize, usize),
    guard_visited: &BitSet<((usize, usize), usize), K, SIZE>,
    (mut r, mut c): (usize, usize),
    mut facing: usize,
    obstruction: (usize, usize),
) -> bool
where
    K: Fn(&((usize, usize), usize)) -> usize + Copy,
{
    let mut visited = BitSet::<_, _, SIZE>::new(guard_visited.key());
    loop {
        if visited.insert(((r, c), facing)).unwrap() || guard_visited.contains(&((r, c), facing)).unwrap() {
            return true;
        }

        if r == 0 || c == 0 || r == width - 1 || c == height - 1 {
            return false;
        }

        (facing, (r, c)) = [(-1, 0), (0, 1), (1, 0), (0, -1)]
            .into_iter()
            .enumerate()
            .cycle()
            .skip(facing)
            .take(4)
            .find_map(|(facing, (dr, dc))| {
                let (r, c) = (
                    r.checked_add_signed(dr).unwrap(),
                    c.checked_add_signed(dc).unwrap(),
                );

                if map[r * (width + 1) + c] == b'#' || (r, c) == obstruction {
                    None
                } else {
                    Some((facing, (r, c)))
                }
            })
            .unwrap();
    }
}

/// # Panics
#[cfg(feature = "parallel")]
pub fn solve_2_par(input: &str) -> usize {
    let map = input.as_bytes();
    let width = map.iter().position(|&c| c == b'\n').unwrap();
    let height = (map.len() + 1) / (width + 1);

    let current_position = map.iter().position(|&c| c == b'^').unwrap();
    let mut current_position = (
        current_position / (width + 1),
        current_position % (width + 1),
    );

    let mut visited = BitSet::<_, _, VISITED_SIZE>::new(|(r, c)| r * width + c);
    let mut visited_pd =
        BitSet::<_, _, VISITED_STATE_SIZE>::new(|((r, c), f)| r * width + c + f * width * height);

    let mut facing = 0;
    core::iter::from_fn(|| loop {
        if current_position.0 == 0
            || current_position.1 == 0
            || current_position.0 == width - 1
            || current_position.1 == height - 1
        {
            return None;
        }

        visited.insert(current_position).unwrap();
        visited_pd.insert((current_position, facing)).unwrap();

        let (next_facing, next_position) = [(-1, 0), (0, 1), (1, 0), (0, -1)]
            .into_iter()
            .enumerate()
            .cycle()
            .skip(facing)
            .take(4)
            .find_map(|(facing, (dr, dc))| {
                let (r, c) = (
                    current_position.0.checked_add_signed(dr).unwrap(),
                    current_position.1.checked_add_signed(dc).unwrap(),
                );

                if map[r * (width + 1) + c] == b'#' {
                    None
                } else {
                    Some((facing, (r, c)))
                }
            })
            .unwrap();

        let result = if visited.contains(&next_position).unwrap() {
            None
        } else {
            Some((
                visited_pd.clone(),
                current_position,
                (next_facing + 1) % 4,
                next_position,
            ))
        };

        (facing, current_position) = (next_facing, next_position);

        if result.is_some() {
            return result;
        }
    })
    .fuse()
    .par_bridge()
    .filter(|(visited, position, facing, obstruction)| {
        is_cycle(
            map,
            (height, width),
            visited,
            *position,
            *facing,
            *obstruction,
        )
    })
    .count()
}

/// # Panics
pub fn solve_2_sync(input: &str) -> usize {
    let map = input.as_bytes();
    let width = map.iter().position(|&c| c == b'\n').unwrap();
    let height = (map.len() + 1) / (width + 1);

    let current_position = map.iter().position(|&c| c == b'^').unwrap();
    let mut current_position = (
        current_position / (width + 1),
        current_position % (width + 1),
    );

    let mut visited = BitSet::<_, _, VISITED_SIZE>::new(|(r, c)| r * width + c);
    let mut visited_pd =
        BitSet::<_, _, VISITED_STATE_SIZE>::new(|((r, c), f)| r * width + c + f * width * height);

    let mut count = 0;
    let mut facing = 0;
    loop {
        if current_position.0 == 0
            || current_position.1 == 0
            || current_position.0 == width - 1
            || current_position.1 == height - 1
        {
            return count;
        }

        visited.insert(current_position).unwrap();
        visited_pd.insert((current_position, facing)).unwrap();

        let (next_facing, next_position) = [(-1, 0), (0, 1), (1, 0), (0, -1)]
            .into_iter()
            .enumerate()
            .cycle()
            .skip(facing)
            .take(4)
            .find_map(|(facing, (dr, dc))| {
                let (r, c) = (
                    current_position.0.checked_add_signed(dr).unwrap(),
                    current_position.1.checked_add_signed(dc).unwrap(),
                );

                if map[r * (width + 1) + c] == b'#' {
                    None
                } else {
                    Some((facing, (r, c)))
                }
            })
            .unwrap();

        if !visited.contains(&next_position).unwrap()
            && is_cycle(
                map,
                (height, width),
                &visited_pd,
                current_position,
                (next_facing + 1) % 4,
                next_position,
            )
        {
            count += 1;
        }

        (facing, current_position) = (next_facing, next_position);
    }
}

#[cfg(not(feature = "parallel"))]
pub use solve_2_sync as solve_2;

#[cfg(feature = "parallel")]
pub use solve_2_par as solve_2;

#[cfg(feature = "input")]
pub fn part_1() -> usize {
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
        static ref INPUT: &'static str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
    }

    #[test]
    fn same_results_1() {
        assert_eq!(solve_1(&INPUT), 41);
    }

    #[test]
    fn same_results_2_sync() {
        assert_eq!(solve_2_sync(&INPUT), 6);
    }

    #[test]
    fn same_results_2_par() {
        assert_eq!(solve_2_par(&INPUT), 6);
    }
}
