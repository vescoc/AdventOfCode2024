#![no_std]

#![allow(clippy::must_use_candidate)]

use core::mem;

#[cfg(not(target_family = "wasm"))]
use rayon::prelude::*;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

#[derive(Copy, Clone)]
pub struct BitSet<const SIZE: usize, T, K: Fn(&T) -> usize> {
    data: [usize; SIZE],
    key: K,
    _marker: core::marker::PhantomData<T>,
}

impl<const SIZE: usize, T, K: Fn(&T) -> usize> BitSet<SIZE, T, K> {
    pub const fn new(key: K) -> Self {
        Self {
            data: [0; SIZE],
            key,
            _marker: core::marker::PhantomData,
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn insert(&mut self, idx: T) -> bool {
        let idx = (self.key)(&idx);
        let (i, b) = (idx / mem::size_of::<u128>(), idx % mem::size_of::<u128>());

        let data = &mut self.data[i];

        let mask = 1 << b;

        let result = *data & mask != 0;

        *data |= mask;

        result
    }

    pub fn contains(&self, idx: &T) -> bool {
        let idx = (self.key)(idx);
        let (i, b) = (idx / mem::size_of::<u128>(), idx % mem::size_of::<u128>());

        self.data[i] & (1 << b) != 0
    }

    pub fn remove(&mut self, idx: &T) {
        let idx = (self.key)(idx);
        let (i, b) = (idx / mem::size_of::<u128>(), idx % mem::size_of::<u128>());

        self.data[i] &= !(1 << b);
    }

    pub fn len(&self) -> usize {
        self.data
            .iter()
            .map(|value| value.count_ones() as usize)
            .sum()
    }

    pub fn is_empty(&self) -> bool {
        self.data.iter().all(|&value| value == 0)
    }
}

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

    let mut visited =
        BitSet::<{ 256 * 256 / mem::size_of::<u128>() }, _, _>::new(|(r, c)| r * width + c);

    let mut facing = 0;
    loop {
        visited.insert(current_position);
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
    mut visited: BitSet<SIZE, ((usize, usize), usize), K>,
    (mut r, mut c): (usize, usize),
    mut facing: usize,
    obstruction: (usize, usize),
) -> bool
where
    K: Fn(&((usize, usize), usize)) -> usize,
{
    loop {
        if visited.insert(((r, c), facing)) {
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
#[cfg(not(target_family = "wasm"))]
pub fn solve_2_par(input: &str) -> usize {
    let map = input.as_bytes();
    let width = map.iter().position(|&c| c == b'\n').unwrap();
    let height = (map.len() + 1) / (width + 1);

    let current_position = map.iter().position(|&c| c == b'^').unwrap();
    let mut current_position = (
        current_position / (width + 1),
        current_position % (width + 1),
    );

    let mut visited =
        BitSet::<{ 256 * 256 / mem::size_of::<u128>() }, _, _>::new(|(r, c)| r * width + c);
    let mut visited_pd =
        BitSet::<{ 256 * 256 * 5 / mem::size_of::<u128>() }, _, _>::new(|((r, c), f)| {
            r * width + c + f * width * height
        });

    let mut facing = 0;
    core::iter::from_fn(|| loop {
        if current_position.0 == 0
            || current_position.1 == 0
            || current_position.0 == width - 1
            || current_position.1 == height - 1
        {
            return None;
        }

        visited.insert(current_position);
        visited_pd.insert((current_position, facing));

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

        let result = if visited.contains(&next_position) {
            None
        } else {
            Some((
                visited_pd,
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
            *visited,
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

    let mut visited =
        BitSet::<{ 256 * 256 / mem::size_of::<u128>() }, _, _>::new(|(r, c)| r * width + c);
    let mut visited_pd =
        BitSet::<{ 256 * 256 * 5 / mem::size_of::<u128>() }, _, _>::new(|((r, c), f)| {
            r * width + c + f * width * height
        });

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

        visited.insert(current_position);
        visited_pd.insert((current_position, facing));

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

        if !visited.contains(&next_position)
            && is_cycle(
                map,
                (height, width),
                visited_pd,
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

pub use solve_2_sync as solve_2;

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
