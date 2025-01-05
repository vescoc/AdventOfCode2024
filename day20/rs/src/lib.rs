#![no_std]
#![allow(clippy::must_use_candidate)]

use bitset::BitSet;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[cfg(feature = "input")]
pub const INPUT: &str = include_str!("../../input");

#[cfg(not(feature = "input"))]
pub const INPUT: &str = "";

use heapless::Vec as HLVec;

type Vec<T> = HLVec<T, { 4096 * 4 }>;

/// # Panics
#[allow(clippy::cast_possible_truncation)]
pub fn solve_v<const MIN_SAVING: usize, const CHEAT_LEN: usize>(input: &str) -> usize {
    let map = input.as_bytes();
    let width = map.iter().position(|&tile| tile == b'\n').unwrap();
    let height = (map.len() + 1) / (width + 1);

    let (start_r, start_c) = map
        .chunks(width + 1)
        .take(height)
        .enumerate()
        .find_map(|(r, row)| {
            row.iter()
                .take(width)
                .position(|&tile| tile == b'S')
                .map(|c| (r, c))
        })
        .unwrap();

    let mut visited = BitSet::<_, _, { BitSet::with_capacity(141 * 142) }>::new(|(r, c)| {
        *r as usize * (width + 1) + *c as usize
    });
    visited.insert((start_r as u8, start_c as u8)).unwrap();

    let path_iter = (0u16..).scan(
        ((start_r as u8, start_c as u8), false),
        |((r, c), done), i| {
            if *done {
                return None;
            }

            if map[*r as usize * (width + 1) + *c as usize] == b'E' {
                *done = true;
                Some(((*r, *c), i))
            } else {
                let res = ((*r, *c), i);

                (*r, *c) = [(0, 1), (0, -1), (1, 0), (-1, 0)]
                    .into_iter()
                    .find_map(|(dr, dc)| {
                        match (r.checked_add_signed(dr), c.checked_add_signed(dc)) {
                            (Some(nr), Some(nc))
                                if (nr as usize) < height
                                    && (nc as usize) < width
                                    && map[nr as usize * (width + 1) + nc as usize] != b'#' =>
                            {
                                if visited.insert((nr, nc)).unwrap() {
                                    None
                                } else {
                                    Some((nr, nc))
                                }
                            }
                            _ => None,
                        }
                    })
                    .unwrap();

                Some(res)
            }
        },
    );

    let mut main_path = Vec::new();
    for v in path_iter {
        main_path.push(v).unwrap();
    }

    let path_len = main_path.len();

    #[cfg(feature = "parallel")]
    let steps = main_path.par_iter();

    #[cfg(not(feature = "parallel"))]
    let steps = main_path.iter();

    steps
        .map(|((r, c), i)| {
            main_path
                .iter()
                .skip(*i as usize + MIN_SAVING + 1)
                .filter_map(|((tr, tc), ti)| {
                    let distance = r.abs_diff(*tr) as usize + c.abs_diff(*tc) as usize;
                    if distance <= CHEAT_LEN {
                        let cheat_path_len = path_len - *ti as usize + *i as usize + distance;
                        let saving = path_len.saturating_sub(cheat_path_len);
                        if saving >= MIN_SAVING {
                            Some(())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .count()
        })
        .sum()
}

/// # Panics
#[allow(clippy::cast_possible_truncation)]
pub fn solve_m<const MIN_SAVING: usize, const CHEAT_LEN: usize>(input: &str) -> usize {
    let map = input.as_bytes();
    let width = map.iter().position(|&tile| tile == b'\n').unwrap();
    let height = (map.len() + 1) / (width + 1);

    let (start_r, start_c) = map
        .chunks(width + 1)
        .take(height)
        .enumerate()
        .find_map(|(r, row)| {
            row.iter()
                .take(width)
                .position(|&tile| tile == b'S')
                .map(|c| (r, c))
        })
        .unwrap();

    let mut visited = BitSet::<_, _, { BitSet::with_capacity(141 * 142) }>::new(|(r, c)| {
        *r as usize * (width + 1) + *c as usize
    });
    visited.insert((start_r as u8, start_c as u8)).unwrap();

    let path_iter = (0u16..).scan(
        ((start_r as u8, start_c as u8), false),
        |((r, c), done), i| {
            if *done {
                return None;
            }

            if map[*r as usize * (width + 1) + *c as usize] == b'E' {
                *done = true;
                Some(((*r, *c), i))
            } else {
                let res = ((*r, *c), i);

                (*r, *c) = [(0, 1), (0, -1), (1, 0), (-1, 0)]
                    .into_iter()
                    .find_map(|(dr, dc)| {
                        match (r.checked_add_signed(dr), c.checked_add_signed(dc)) {
                            (Some(nr), Some(nc))
                                if (nr as usize) < height
                                    && (nc as usize) < width
                                    && map[nr as usize * (width + 1) + nc as usize] != b'#' =>
                            {
                                if visited.insert((nr, nc)).unwrap() {
                                    None
                                } else {
                                    Some((nr, nc))
                                }
                            }
                            _ => None,
                        }
                    })
                    .unwrap();

                Some(res)
            }
        },
    );

    let mut path = Vec::new();
    let mut main_path = [[0u16; 141]; 142];
    for ((r, c), v) in path_iter {
        main_path[r as usize][c as usize] = v;
        path.push(((r, c), v)).unwrap();
    }

    let path_len = path.len();

    #[cfg(feature = "parallel")]
    let steps = path.par_iter();

    #[cfg(not(feature = "parallel"))]
    let steps = path.iter();

    let main_path = &main_path; // borrow checker :PPP
    steps
        .map(|((r, c), i)| {
            ((*r as usize).saturating_sub(CHEAT_LEN)..(*r as usize + CHEAT_LEN + 1).min(height))
                .flat_map(move |tr| {
                    ((*c as usize).saturating_sub(CHEAT_LEN)
                        ..(*c as usize + CHEAT_LEN + 1).min(width))
                        .filter_map(move |tc| {
                            if (*r, *c) == (tr as u8, tc as u8) {
                                return None;
                            }

                            let distance = (*r as usize).abs_diff(tr) + (*c as usize).abs_diff(tc);
                            if distance <= CHEAT_LEN && map[tr * (width + 1) + tc] != b'#' {
                                let cheat_path_len =
                                    path_len - main_path[tr][tc] as usize + *i as usize + distance;
                                let saving = path_len.saturating_sub(cheat_path_len);
                                if saving >= MIN_SAVING {
                                    return Some((cheat_path_len, saving));
                                }
                            }

                            None
                        })
                })
                .count()
        })
        .sum()
}

/// # Panics
#[cfg_attr(target_os = "none", inline(never))]
pub fn solve_1(input: &str) -> usize {
    solve_m::<100, 2>(input)
}

/// # Panics
#[cfg_attr(target_os = "none", inline(never))]
pub fn solve_2(input: &str) -> usize {
    solve_v::<100, 20>(input)
}

#[cfg(feature = "input")]
pub fn part_1() -> usize {
    solve_1(INPUT)
}

#[cfg(feature = "input")]
pub fn part_2() -> usize {
    solve_2(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn same_results_1_m() {
        assert_eq!(solve_m::<12, 2>(INPUT), 3 + 1 + 1 + 1 + 1 + 1);
    }

    #[test]
    fn same_results_2_m() {
        assert_eq!(solve_m::<50, 20>(INPUT), 285);
    }

    #[test]
    fn same_results_1_v() {
        assert_eq!(solve_v::<12, 2>(INPUT), 3 + 1 + 1 + 1 + 1 + 1);
    }

    #[test]
    fn same_results_2_v() {
        assert_eq!(solve_v::<50, 20>(INPUT), 285);
    }

    #[cfg(feature = "input")]
    #[test]
    fn same_results_1_v_vs_m() {
        assert_eq!(
            solve_v::<100, 2>(super::INPUT),
            solve_m::<100, 2>(super::INPUT)
        );
    }

    #[cfg(feature = "input")]
    #[test]
    fn same_results_2_v_vs_m() {
        assert_eq!(
            solve_v::<100, 20>(super::INPUT),
            solve_m::<100, 20>(super::INPUT)
        );
    }
}
