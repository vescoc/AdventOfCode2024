#![no_std]

#![allow(clippy::must_use_candidate)]

use core::mem;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

struct BitSet<const SIZE: usize, T, K: Fn(&T) -> usize> {
    data: [usize; SIZE],
    key: K,
    _marker: core::marker::PhantomData<T>,
}

impl<const SIZE: usize, T, K: Fn(&T) -> usize> BitSet<SIZE, T, K> {
    const fn new(key: K) -> Self {
        Self {
            data: [0; SIZE],
            key,
            _marker: core::marker::PhantomData,
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    fn insert(&mut self, idx: T) {
        let idx = (self.key)(&idx);
        let (i, b) = (idx / mem::size_of::<usize>(), idx % mem::size_of::<usize>());

        self.data[i] |= 1 << b;
    }

    fn contains(&self, idx: &T) -> bool {
        let idx = (self.key)(idx);
        let (i, b) = (idx / mem::size_of::<usize>(), idx % mem::size_of::<usize>());

        self.data[i] & (1 << b) != 0
    }

    fn remove(&mut self, idx: &T) {
        let idx = (self.key)(idx);
        let (i, b) = (idx / mem::size_of::<usize>(), idx % mem::size_of::<usize>());

        self.data[i] &= !(1 << b);
    }

    fn len(&self) -> usize {
        self.data
            .iter()
            .map(|value| value.count_ones() as usize)
            .sum()
    }
}

impl<const SIZE: usize, T, K> BitSet<SIZE, T, K>
where
    K: Fn(&T) -> usize + Copy,
{
    fn key(&self) -> K {
        self.key
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

        for (next_facing, (dr, dc)) in [(-1, 0), (0, 1), (1, 0), (0, -1)]
            .into_iter()
            .cycle()
            .enumerate()
            .skip(facing)
            .take(4)
        {
            match (
                current_position.0.checked_add_signed(dr),
                current_position.1.checked_add_signed(dc),
            ) {
                (Some(r), Some(c)) if r < height && c < width => {
                    if map[r * (width + 1) + c] == b'#' {
                        continue;
                    }
                    current_position = (r, c);
                    facing = next_facing;
                    break;
                }
                _ => return visited.len(),
            }
        }
    }
}

fn is_cycle<const SIZE: usize, K>(
    map: &[u8],
    (height, width): (usize, usize),
    current_visited: &BitSet<SIZE, ((usize, usize), usize), K>,
    (mut r, mut c): (usize, usize),
    mut facing: usize,
    obstruction: (usize, usize),
) -> bool
where
    K: Fn(&((usize, usize), usize)) -> usize + Copy,
{
    let mut visited = BitSet::<SIZE, ((usize, usize), usize), K>::new(current_visited.key());
    'outher: loop {
        if visited.contains(&((r, c), facing)) || current_visited.contains(&((r, c), facing)) {
            return true;
        }

        visited.insert(((r, c), facing));

        for (next_facing, (dr, dc)) in [(-1, 0), (0, 1), (1, 0), (0, -1)]
            .into_iter()
            .enumerate()
            .cycle()
            .skip(facing)
            .take(4)
        {
            match (r.checked_add_signed(dr), c.checked_add_signed(dc)) {
                (Some(nr), Some(nc)) if nr < height && nc < width => {
                    if map[nr * (width + 1) + nc] == b'#' || (nr, nc) == obstruction {
                        continue;
                    }

                    (r, c) = (nr, nc);

                    facing = next_facing;

                    continue 'outher;
                }
                _ => return false,
            }
        }

        unreachable!();
    }
}

/// # Panics
pub fn solve_2(input: &str) -> usize {
    let map = input.as_bytes();
    let width = map.iter().position(|&c| c == b'\n').unwrap();
    let height = (map.len() + 1) / (width + 1);

    let current_position = map.iter().position(|&c| c == b'^').unwrap();
    let mut current_position = (
        current_position / (width + 1),
        current_position % (width + 1),
    );

    let invalid_position = (current_position.0 - 1, current_position.1);

    let mut facing = 0;

    let mut visited =
        BitSet::<{ 256 * 256 / mem::size_of::<usize>() }, _, _>::new(|(r, c)| r * width + c);
    let mut visited_pd =
        BitSet::<{ 256 * 256 * 5 / mem::size_of::<usize>() }, _, _>::new(|((r, c), f)| {
            r * width + c + f * width * height
        });
    let mut obstructions =
        BitSet::<{ 256 * 256 / mem::size_of::<usize>() }, _, _>::new(|(r, c)| r * width + c);
    
    'outher: loop {
        visited.insert(current_position);
        visited_pd.insert((current_position, facing));

        for (next_facing, (dr, dc)) in [(-1, 0), (0, 1), (1, 0), (0, -1)]
            .into_iter()
            .enumerate()
            .cycle()
            .skip(facing)
            .take(4)
        {
            match (
                current_position.0.checked_add_signed(dr),
                current_position.1.checked_add_signed(dc),
            ) {
                (Some(r), Some(c)) if r < height && c < width => {
                    if map[r * (width + 1) + c] == b'#' {
                        continue;
                    }

                    if !obstructions.contains(&(r, c))
                        && !visited.contains(&(r, c))
                        && is_cycle(
                            map,
                            (height, width),
                            &visited_pd,
                            current_position,
                            (next_facing + 1) % 4,
                            (r, c),
                        )
                    {
                        obstructions.insert((r, c));
                    }

                    current_position = (r, c);

                    facing = next_facing;

                    continue 'outher;
                }
                _ => {
                    obstructions.remove(&invalid_position);

                    return obstructions.len();
                }
            }
        }

        unreachable!();
    }
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
    fn same_results_2() {
        assert_eq!(solve_2(&INPUT), 6);
    }
}
