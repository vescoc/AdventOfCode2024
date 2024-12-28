#![no_std]
#![allow(clippy::must_use_candidate)]

use heapless::Vec as HLVec;

type Position = (usize, usize);

type Vec<T> = HLVec<T, { 64 * 128 }>;

#[cfg(feature = "input")]
pub const INPUT: &str = include_str!("../../input");

#[cfg(not(feature = "input"))]
pub const INPUT: &str = "";

fn gps_score<const TARGET: u8>(map: &[u8], &(height, width): &Position) -> usize {
    map.chunks(width + 1)
        .take(height)
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .take(width)
                .enumerate()
                .filter_map(move |(c, tile)| {
                    if *tile == TARGET {
                        Some(r * 100 + c)
                    } else {
                        None
                    }
                })
        })
        .sum()
}

fn push_up_or_left<const DR: usize, const DC: usize>(
    map: &mut [u8],
    &(_, width): &Position,
    &(r, c): &Position,
) -> Position {
    let (mut er, mut ec) = (r, c);
    while er > 0 && ec > 0 {
        (er, ec) = (er - DR, ec - DC);
        if matches!(map[er * (width + 1) + ec], b'#' | b'.') {
            break;
        }
    }

    if map[er * (width + 1) + ec] == b'#' {
        (r, c)
    } else {
        if DC == 1 {
            for i in ec..c {
                map[r * (width + 1) + i] = map[r * (width + 1) + (i + 1)];
            }
        } else {
            for i in er..r {
                map[i * (width + 1) + c] = map[(i + 1) * (width + 1) + c];
            }
        }

        (r - DR, c - DC)
    }
}

fn push_down_or_right<const DR: usize, const DC: usize>(
    map: &mut [u8],
    &(height, width): &Position,
    &(r, c): &Position,
) -> Position {
    let (mut er, mut ec) = (r, c);
    while er < height - 1 && ec < width - 1 {
        (er, ec) = (er + DR, ec + DC);
        if matches!(map[er * (width + 1) + ec], b'#' | b'.') {
            break;
        }
    }

    if map[er * (width + 1) + ec] == b'#' {
        (r, c)
    } else {
        if DC == 1 {
            for i in (c..ec).rev() {
                map[r * (width + 1) + i + 1] = map[r * (width + 1) + i];
            }
        } else {
            for i in (r..er).rev() {
                map[(i + 1) * (width + 1) + c] = map[i * (width + 1) + c];
            }
        }

        (r + DR, c + DC)
    }
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
fn large_push_up_down_rec<const DIRECTION: isize>(
    map: &mut [u8],
    &(height, width): &Position,
    &(r, c): &Position,
) -> Position {
    fn can_push_box<const DIRECTION: isize>(
        map: &[u8],
        dim: &Position,
        &(r, c): &Position,
    ) -> bool {
        let rn = (r as isize + DIRECTION) as usize;

        let (_, width) = dim;

        // [][]
        // .[].
        match (map[rn * (width + 1) + c], map[rn * (width + 1) + c + 1]) {
            (b'.', b'.') => true,
            (b'[', b']') => can_push_box::<DIRECTION>(map, dim, &(rn, c)),
            (b']', b'[') => {
                can_push_box::<DIRECTION>(map, dim, &(rn, c - 1))
                    && can_push_box::<DIRECTION>(map, dim, &(rn, c + 1))
            }
            (b']', b'.') => can_push_box::<DIRECTION>(map, dim, &(rn, c - 1)),
            (b'.', b'[') => can_push_box::<DIRECTION>(map, dim, &(rn, c + 1)),
            _ => false,
        }
    }

    fn push_box<const DIRECTION: isize>(
        map: &mut [u8],
        &(height, width): &Position,
        &(r, c): &Position,
    ) {
        let rn = (r as isize + DIRECTION) as usize;

        // [][]
        // .[].
        match (map[rn * (width + 1) + c], map[rn * (width + 1) + c + 1]) {
            (b'[', b']') => push_box::<DIRECTION>(map, &(height, width), &(rn, c)),
            (b']', b'[') => {
                push_box::<DIRECTION>(map, &(height, width), &(rn, c - 1));
                push_box::<DIRECTION>(map, &(height, width), &(rn, c + 1));
            }
            (b']', b'.') => push_box::<DIRECTION>(map, &(height, width), &(rn, c - 1)),
            (b'.', b'[') => push_box::<DIRECTION>(map, &(height, width), &(rn, c + 1)),
            _ => {}
        }

        (map[rn * (width + 1) + c], map[rn * (width + 1) + c + 1]) = (b'[', b']');

        (map[r * (width + 1) + c], map[r * (width + 1) + c + 1]) = (b'.', b'.');
    }

    let rn = (r as isize + DIRECTION) as usize;

    // last row are walls
    match map[rn * (width + 1) + c] {
        b'.' => (rn, c),
        b']' if can_push_box::<DIRECTION>(map, &(height, width), &(rn, c - 1)) => {
            push_box::<DIRECTION>(map, &(height, width), &(rn, c - 1));
            (rn, c)
        }
        b'[' if can_push_box::<DIRECTION>(map, &(height, width), &(rn, c)) => {
            push_box::<DIRECTION>(map, &(height, width), &(rn, c));
            (rn, c)
        }
        _ => (r, c),
    }
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
fn large_push_up_down_bfs<const DIRECTION: isize>(
    map: &mut [u8],
    &(_, width): &Position,
    &(r, c): &Position,
) -> Position {
    use heapless::Deque as HLDeque;

    type Queue<T> = HLDeque<T, 64>;

    let mut list = [0_u128; 64];

    let bfs = |map: &[u8], list: &mut [u128], &(r, c): &Position| {
        let mut max_level = usize::MIN;

        let mut queue = Queue::new();
        queue.push_back(((r, c), 0)).unwrap();

        while let Some(((r, c), level)) = queue.pop_back() {
            max_level = max_level.max(level);

            list[level] |= 1 << c;

            let rn = (r as isize + DIRECTION) as usize;

            // [][]
            // .[].
            match (map[rn * (width + 1) + c], map[rn * (width + 1) + c + 1]) {
                (b'.', b'.') => continue,
                (b'[', b']') => {
                    if list[level + 1] & 1 << c == 0 {
                        list[level + 1] |= 1 << c;
                        queue.push_back(((rn, c), level + 1)).unwrap();
                    }
                }
                (b']', b'[') => {
                    if list[level + 1] & 1 << (c - 1) == 0 {
                        list[level + 1] |= 1 << (c - 1);
                        queue.push_back(((rn, c - 1), level + 1)).unwrap();
                    }
                    if list[level + 1] & 1 << (c + 1) == 0 {
                        list[level + 1] |= 1 << (c + 1);
                        queue.push_back(((rn, c + 1), level + 1)).unwrap();
                    }
                }
                (b']', b'.') => {
                    if list[level + 1] & 1 << (c - 1) == 0 {
                        list[level + 1] |= 1 << (c - 1);
                        queue.push_back(((rn, c - 1), level + 1)).unwrap();
                    }
                }
                (b'.', b'[') => {
                    if list[level + 1] & 1 << (c + 1) == 0 {
                        list[level + 1] |= 1 << (c + 1);
                        queue.push_back(((rn, c + 1), level + 1)).unwrap();
                    }
                }
                _ => return None,
            }
        }

        Some(max_level + 1)
    };

    let push = |map: &mut [u8], list: &[u128], r| {
        for (i, set) in list.iter().enumerate().rev() {
            for c in 0..width {
                if set & (1 << c) != 0 {
                    let rd = (r as isize + DIRECTION * (i as isize + 1)) as usize;
                    let rs = (r as isize + DIRECTION * i as isize) as usize;

                    (map[rd * (width + 1) + c], map[rd * (width + 1) + c + 1]) = (b'[', b']');

                    (map[rs * (width + 1) + c], map[rs * (width + 1) + c + 1]) = (b'.', b'.');
                }
            }
        }
    };

    let rn = (r as isize + DIRECTION) as usize;

    // last row are walls
    match map[rn * (width + 1) + c] {
        b'.' => (rn, c),
        b']' => {
            if let Some(level) = bfs(map, &mut list, &(rn, c - 1)) {
                push(map, &list[0..level], rn);

                (rn, c)
            } else {
                (r, c)
            }
        }
        b'[' => {
            if let Some(level) = bfs(map, &mut list, &(rn, c)) {
                push(map, &list[0..level], rn);

                (rn, c)
            } else {
                (r, c)
            }
        }
        _ => (r, c),
    }
}

/// # Panics
pub fn solve_1(input: &str) -> usize {
    solve::<b'O'>(
        input,
        |data| data.iter().copied().collect::<Vec<_>>(),
        push_up_or_left::<1, 0>,
        push_down_or_right::<1, 0>,
    )
}

/// # Panics
pub fn solve_2_rec(input: &str) -> usize {
    solve::<b'['>(
        input,
        |data| {
            data.iter()
                .flat_map(|&tile| match tile {
                    b'#' => [b'#', b'#'].as_slice(),
                    b'O' => [b'[', b']'].as_slice(),
                    b'.' => [b'.', b'.'].as_slice(),
                    b'@' => [b'@', b'.'].as_slice(),
                    b'\n' => [b'\n'].as_slice(),
                    _ => panic!("invalid char: {tile}"),
                })
                .copied()
                .collect::<Vec<_>>()
        },
        large_push_up_down_rec::<-1>,
        large_push_up_down_rec::<1>,
    )
}

/// # Panics
pub fn solve_2_bfs(input: &str) -> usize {
    solve::<b'['>(
        input,
        |data| {
            data.iter()
                .flat_map(|&tile| match tile {
                    b'#' => [b'#', b'#'].as_slice(),
                    b'O' => [b'[', b']'].as_slice(),
                    b'.' => [b'.', b'.'].as_slice(),
                    b'@' => [b'@', b'.'].as_slice(),
                    b'\n' => [b'\n'].as_slice(),
                    _ => panic!("invalid char: {tile}"),
                })
                .copied()
                .collect::<Vec<_>>()
        },
        large_push_up_down_bfs::<-1>,
        large_push_up_down_bfs::<1>,
    )
}

pub use solve_2_rec as solve_2;

/// # Panics
fn solve<const TARGET: u8>(
    input: &str,
    make_map: impl Fn(&[u8]) -> Vec<u8>,
    push_up: impl Fn(&mut [u8], &Position, &Position) -> Position,
    push_down: impl Fn(&mut [u8], &Position, &Position) -> Position,
) -> usize {
    let mut parts = input.split("\n\n");

    let mut map = make_map(parts.next().unwrap().as_bytes());

    let width = map.iter().position(|&c| c == b'\n').unwrap();
    let height = (map.len() + 1) / (width + 1);

    let (mut r, mut c) = map
        .chunks(width + 1)
        .take(height)
        .enumerate()
        .find_map(|(r, row)| {
            row.iter().take(width).enumerate().find_map(|(c, tile)| {
                if *tile == b'@' {
                    Some((r, c))
                } else {
                    None
                }
            })
        })
        .unwrap();

    map[r * (width + 1) + c] = b'.';

    for m in parts.next().unwrap().lines().flat_map(|line| line.chars()) {
        match m {
            '^' => (r, c) = push_up(&mut map, &(height, width), &(r, c)),
            '<' => (r, c) = push_up_or_left::<0, 1>(&mut map, &(height, width), &(r, c)),
            '>' => (r, c) = push_down_or_right::<0, 1>(&mut map, &(height, width), &(r, c)),
            'v' => (r, c) = push_down(&mut map, &(height, width), &(r, c)),
            _ => panic!("invalid move {m}"),
        }
    }

    gps_score::<TARGET>(&map, &(height, width))
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

    const INPUT_1: &str = r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
    const INPUT_2: &str = include_str!("../../example1");

    #[test]
    fn test_gps_score() {
        let input = br"##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########";

        assert_eq!(gps_score::<b'O'>(input.as_slice(), &(10, 10)), 10092);
    }

    #[test]
    fn test_up() {
        let target = br"########
#..O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########";

        let mut input = target.to_vec();

        assert_eq!(
            push_up_or_left::<1, 0>(&mut input, &(8, 8), &(2, 2)),
            (1, 2)
        );
        assert_eq!(
            target.as_slice(),
            &input,
            "\n{}",
            core::str::from_utf8(&input).unwrap()
        );
    }

    #[test]
    fn test_down() {
        let target = br"########
#..O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########";

        let mut input = target.to_vec();

        assert_eq!(
            push_down_or_right::<1, 0>(&mut input, &(8, 8), &(2, 2)),
            (3, 2)
        );
        assert_eq!(
            target.as_slice(),
            &input,
            "\n{}",
            core::str::from_utf8(&input).unwrap()
        );
    }

    #[test]
    fn test_left() {
        let target = br"########
#..O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########";

        let mut input = target.to_vec();

        assert_eq!(
            push_up_or_left::<0, 1>(&mut input, &(8, 8), &(2, 2)),
            (2, 2)
        );
        assert_eq!(
            target.as_slice(),
            &input,
            "\n{}",
            core::str::from_utf8(&input).unwrap()
        );
    }

    #[test]
    fn test_right() {
        let target = br"########
#..O.O.#
##.OO..#
#...O..#
#.#.O..#
#...O..#
#......#
########";

        let mut input = target.to_vec();

        let target = br"########
#..O.O.#
##..OO.#
#...O..#
#.#.O..#
#...O..#
#......#
########";

        assert_eq!(
            push_down_or_right::<0, 1>(&mut input, &(8, 8), &(2, 2)),
            (2, 3)
        );
        assert_eq!(
            target.as_slice(),
            &input,
            "\n{}",
            core::str::from_utf8(&input).unwrap()
        );
    }

    #[test]
    fn same_results_1_1() {
        assert_eq!(solve_1(INPUT_1), 2028);
    }

    #[test]
    fn same_results_1_2() {
        assert_eq!(solve_1(INPUT_2), 10092);
    }

    #[test]
    fn same_results_2_rec() {
        assert_eq!(solve_2_rec(INPUT_2), 9021);
    }

    #[test]
    fn same_results_2_bfs() {
        assert_eq!(solve_2_bfs(INPUT_2), 9021);
    }

    #[cfg(feature = "input")]
    #[test]
    fn same_results_2() {
        assert_eq!(solve_2_rec(INPUT), solve_2_bfs(INPUT));
    }
}
