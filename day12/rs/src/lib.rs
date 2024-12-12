#![no_std]
#![allow(clippy::must_use_candidate)]

use bitset::BitSet;
use heapless::{Deque, Entry, FnvIndexMap};

type VecDeque<T> = Deque<T, 1024>;
type Set<T, K> = BitSet<T, K, { BitSet::with_capacity(141 * 141) }>;
type Map<K, V> = FnvIndexMap<K, V, 512>;

#[cfg(feature = "input")]
use lazy_static::lazy_static;

#[cfg(feature = "input")]
lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

#[cfg(not(feature = "input"))]
pub static INPUT: &'static str = &"";

fn solve<ACC, A, F>(input: &str, accumulate: A, finalize: F) -> usize
where
    ACC: Default,
    A: Fn(&mut ACC, &(usize, usize), &(isize, isize)),
    F: Fn(ACC) -> usize,
{
    let map = input.as_bytes();
    let width = map.iter().position(|&c| c == b'\n').unwrap();
    let height = (map.len() + 1) / (width + 1);

    let mut total = 0;

    let mut visited = Set::new(|(r, c)| r * (width + 1) + c);
    for (r, row) in map.chunks(width + 1).enumerate().take(height) {
        for (c, &plant) in row.iter().take(width).enumerate() {
            if visited.contains(&(r, c)).unwrap() {
                continue;
            }

            let mut accumulator = ACC::default();

            let mut region = Set::new(|(r, c)| r * (width + 1) + c);
            region.insert((r, c)).unwrap();

            let mut queue = VecDeque::new();
            queue.push_back((r, c)).unwrap();

            while let Some((r, c)) = queue.pop_front() {
                for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    match (r.checked_add_signed(dr), c.checked_add_signed(dc)) {
                        (Some(r), Some(c))
                            if r < height && c < width && map[r * (width + 1) + c] == plant =>
                        {
                            if !region.insert((r, c)).unwrap() {
                                queue.push_back((r, c)).unwrap();
                                visited.insert((r, c)).unwrap();
                            }
                        }
                        _ => accumulate(&mut accumulator, &(r, c), &(dr, dc)),
                    }
                }
            }

            let multiplier = finalize(accumulator);

            total += region.len() * multiplier;
        }
    }

    total
}

/// # Panics
pub fn solve_1(input: &str) -> usize {
    solve::<usize, _, _>(
        input,
        |perimeter, _, _| *perimeter += 1,
        |perimeter| perimeter,
    )
}

/// # Panics
#[allow(clippy::cast_possible_wrap)]
pub fn solve_2(input: &str) -> usize {
    fn key(g: (isize, isize)) -> usize {
        match g {
            (0, 1) => 0,
            (0, -1) => 1,
            (1, 0) => 2,
            (-1, 0) => 3,
            _ => panic!("invalid gradient {g:?}"),
        }
    }

    solve::<Map<(isize, isize), u8>, _, _>(
        input,
        |vertex, &(r, c), &(dr, dc)| {
            let (r, c) = ((r as isize) * 2, (c as isize) * 2);
            for (vr, vc) in [(r + dr - dc, c + dc + dr), (r + dr + dc, c + dc - dr)] {
                match vertex.entry((vr, vc)) {
                    Entry::Occupied(mut v) => {
                        *v.get_mut() |= 1 << key((dr, dc));
                    }
                    Entry::Vacant(v) => {
                        v.insert(1 << key((dr, dc))).unwrap();
                    }
                }
            }
        },
        |vertex| {
            vertex
                .values()
                .map(|v| match v.count_ones() {
                    0 | 1 => 0,
                    2 => 1,
                    4 => 2,
                    _ => panic!("{v:08b}"),
                })
                .sum::<usize>()
        },
    )
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
        static ref INPUT_1: &'static str = r#"AAAA
BBCD
BBCC
EEEC"#;
        static ref INPUT_2: &'static str = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#;
        static ref INPUT_3: &'static str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;
        static ref INPUT_4: &'static str = r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#;
        static ref INPUT_5: &'static str = r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#;
    }

    #[test]
    fn same_results_1_1() {
        assert_eq!(solve_1(&INPUT_1), 140);
    }

    #[test]
    fn same_results_1_2() {
        assert_eq!(solve_1(&INPUT_2), 772);
    }

    #[test]
    fn same_results_1_3() {
        assert_eq!(solve_1(&INPUT_3), 1930);
    }

    #[test]
    fn same_results_2_1() {
        assert_eq!(solve_2(&INPUT_1), 80);
    }

    #[test]
    fn same_results_2_2() {
        assert_eq!(solve_2(&INPUT_2), 436);
    }

    #[test]
    fn same_results_2_3() {
        assert_eq!(solve_2(&INPUT_4), 236);
    }

    #[test]
    fn same_results_2_4() {
        assert_eq!(solve_2(&INPUT_5), 368);
    }

    #[test]
    fn same_results_2_5() {
        assert_eq!(solve_2(&INPUT_3), 1206);
    }
}
