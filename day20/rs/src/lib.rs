#![allow(clippy::must_use_candidate)]

use bitset::BitSet;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[cfg(feature = "input")]
use lazy_static::lazy_static;

#[cfg(feature = "input")]
lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

#[cfg(not(feature = "input"))]
pub const INPUT: &str = "";

/// # Panics
pub fn solve_v<const MIN_SAVING: usize, const CHEAT_LEN: usize>(input: &str) -> usize {
    use heapless::Vec as HLVec;

    type Vec<T> = HLVec<T, { 4096 * 4 }>;

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

    let mut visited =
        BitSet::<_, _, { BitSet::with_capacity(160 * 160) }>::new(|(r, c)| r * (width + 1) + c);
    visited.insert((start_r, start_c)).unwrap();

    let path = (0..).scan(((start_r, start_c), false), |((r, c), done), i| {
        if *done {
            return None;
        }

        if map[*r * (width + 1) + *c] == b'E' {
            *done = true;
            Some(((*r, *c), i))
        } else {
            let res = ((*r, *c), i);

            (*r, *c) = [(0, 1), (0, -1), (1, 0), (-1, 0)]
                .into_iter()
                .find_map(
                    |(dr, dc)| match (r.checked_add_signed(dr), c.checked_add_signed(dc)) {
                        (Some(nr), Some(nc))
                            if nr < height && nc < width && map[nr * (width + 1) + nc] != b'#' =>
                        {
                            if visited.insert((nr, nc)).unwrap() {
                                None
                            } else {
                                Some((nr, nc))
                            }
                        }
                        _ => None,
                    },
                )
                .unwrap();

            Some(res)
        }
    });
    let mut main_path = Vec::new();
    for v in path {
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
                .skip(i + MIN_SAVING + 1)
                .filter_map(|((tr, tc), ti)| {
                    let distance = r.abs_diff(*tr) + c.abs_diff(*tc);
                    if distance <= CHEAT_LEN {
                        let cheat_path_len = path_len - ti + i + distance;
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
pub fn solve_m<const MIN_SAVING: usize, const CHEAT_LEN: usize>(input: &str) -> usize {
    use heapless::FnvIndexMap;

    type Map<K, V> = FnvIndexMap<K, V, { 4096 * 4 }>;

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

    let mut visited =
        BitSet::<_, _, { BitSet::with_capacity(160 * 160) }>::new(|(r, c)| r * (width + 1) + c);
    visited.insert((start_r, start_c)).unwrap();

    let path = (0..).scan(((start_r, start_c), false), |((r, c), done), i| {
        if *done {
            return None;
        }

        if map[*r * (width + 1) + *c] == b'E' {
            *done = true;
            Some(((*r, *c), i))
        } else {
            let res = ((*r, *c), i);

            (*r, *c) = [(0, 1), (0, -1), (1, 0), (-1, 0)]
                .into_iter()
                .find_map(
                    |(dr, dc)| match (r.checked_add_signed(dr), c.checked_add_signed(dc)) {
                        (Some(nr), Some(nc))
                            if nr < height && nc < width && map[nr * (width + 1) + nc] != b'#' =>
                        {
                            if visited.insert((nr, nc)).unwrap() {
                                None
                            } else {
                                Some((nr, nc))
                            }
                        }
                        _ => None,
                    },
                )
                .unwrap();

            Some(res)
        }
    });
    let mut main_path = Map::new();
    for (k, v) in path {
        main_path.insert(k, v).unwrap();
    }

    let path_len = main_path.len();

    #[cfg(feature = "parallel")]
    let steps = main_path.iter().par_bridge();

    #[cfg(not(feature = "parallel"))]
    let steps = main_path.iter();

    let main_path = &main_path; // borrow checker :PPP
    steps
        .map(|(&(r, c), &i)| {
            let (r, c) = (r, c);
            (r.saturating_sub(CHEAT_LEN)..(r + CHEAT_LEN + 1).min(height))
                .flat_map(move |tr| {
                    (c.saturating_sub(CHEAT_LEN)..(c + CHEAT_LEN + 1).min(width)).filter_map(
                        move |tc| {
                            if (r, c) == (tr, tc) {
                                return None;
                            }

                            let distance = r.abs_diff(tr) + c.abs_diff(tc);
                            if distance <= CHEAT_LEN && map[tr * (width + 1) + tc] != b'#' {
                                let cheat_path_len = path_len - main_path[&(tr, tc)] + i + distance;
                                let saving = path_len.saturating_sub(cheat_path_len);
                                if saving >= MIN_SAVING {
                                    return Some((cheat_path_len, saving));
                                }
                            }

                            None
                        },
                    )
                })
                .count()
        })
        .sum()
}

/// # Panics
pub fn solve_1(input: &str) -> usize {
    solve_m::<100, 2>(input)
}

/// # Panics
pub fn solve_2(input: &str) -> usize {
    solve_v::<100, 20>(input)
}

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

    use lazy_static::lazy_static;

    lazy_static! {
        static ref INPUT: &'static str = r#"###############
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
###############"#;
    }

    #[test]
    fn same_results_1_m() {
        assert_eq!(solve_m::<12, 2>(&INPUT), 3 + 1 + 1 + 1 + 1 + 1);
    }

    #[test]
    fn same_results_2_m() {
        assert_eq!(solve_m::<50, 20>(&INPUT), 285);
    }

    #[test]
    fn same_results_1_v() {
        assert_eq!(solve_v::<12, 2>(&INPUT), 3 + 1 + 1 + 1 + 1 + 1);
    }

    #[test]
    fn same_results_2_v() {
        assert_eq!(solve_v::<50, 20>(&INPUT), 285);
    }

    #[cfg(feature = "input")]
    #[test]
    fn same_results_1_v_vs_m() {
        assert_eq!(
            solve_v::<100, 2>(&super::INPUT),
            solve_m::<100, 2>(&super::INPUT)
        );
    }

    #[cfg(feature = "input")]
    #[test]
    fn same_results_2_v_vs_m() {
        assert_eq!(
            solve_v::<100, 20>(&super::INPUT),
            solve_m::<100, 20>(&super::INPUT)
        );
    }
}
