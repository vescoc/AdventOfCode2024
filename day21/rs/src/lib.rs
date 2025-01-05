#![no_std]
#![allow(clippy::must_use_candidate)]

use core::iter;

#[cfg(feature = "input")]
pub const INPUT: &str = include_str!("../../input");

#[cfg(not(feature = "input"))]
pub const INPUT: &str = "";

// mod consts;

const DIRECTIONS: &[((isize, isize), usize)] =
    &[((-1, 0), U), ((0, -1), L), ((0, 1), R), ((1, 0), D)];

const PAD_POS_SYMBOLS: &[u8] = b"A<>^v";
const PAD_NUM_SYMBOLS: &[u8] = b"0123456789A";

const PAD_POS_DISPLAY: [[u8; 3]; 2] = [*b" ^A", *b"<v>"];

const PAD_NUM_DISPLAY: [[u8; 3]; 4] = [*b"789", *b"456", *b"123", *b" 0A"];

const fn symbol(symbols: &[u8], symbol: u8) -> usize {
    let mut i = 0;
    while i < symbols.len() {
        if symbols[i] == symbol {
            return i;
        }

        i += 1;
    }

    panic!("cannot find symbol");
}

const fn find_position<const WIDTH: usize, const HEIGHT: usize>(
    pad: &[[u8; WIDTH]; HEIGHT],
    symbol: u8,
) -> (usize, usize) {
    let mut r = 0;
    while r < HEIGHT {
        let mut c = 0;
        while c < WIDTH {
            if pad[r][c] == symbol {
                return (r, c);
            }

            c += 1;
        }

        r += 1;
    }

    panic!("cannot find symbol in pad");
}

const A: usize = symbol(PAD_POS_SYMBOLS, b'A');
const L: usize = symbol(PAD_POS_SYMBOLS, b'<');
const R: usize = symbol(PAD_POS_SYMBOLS, b'>');
const U: usize = symbol(PAD_POS_SYMBOLS, b'^');
const D: usize = symbol(PAD_POS_SYMBOLS, b'v');

const NUM_A: usize = symbol(PAD_NUM_SYMBOLS, b'A');

const PAD_POS: [(usize, usize); PAD_POS_SYMBOLS.len()] = {
    let mut pad = [(0, 0); PAD_POS_SYMBOLS.len()];

    let mut i = 0;
    while i < pad.len() {
        pad[i] = find_position(&PAD_POS_DISPLAY, PAD_POS_SYMBOLS[i]);

        i += 1;
    }

    pad
};

const NUM_POS: [(usize, usize); PAD_NUM_SYMBOLS.len()] = {
    let mut pad = [(0, 0); PAD_NUM_SYMBOLS.len()];

    let mut i = 0;
    while i < pad.len() {
        pad[i] = find_position(&PAD_NUM_DISPLAY, PAD_NUM_SYMBOLS[i]);

        i += 1;
    }

    pad
};

type PadPos = [[u64; PAD_POS_SYMBOLS.len()]; PAD_POS_SYMBOLS.len()];
type PadNum = [[u64; PAD_NUM_SYMBOLS.len()]; PAD_NUM_SYMBOLS.len()];

const PAD00: PadPos = [[1; PAD_POS_SYMBOLS.len()]; PAD_POS_SYMBOLS.len()];

const PAD02: PadPos = pad_n(PAD00, 2);
const PAD25: PadPos = pad_n(PAD00, 25);

const NUM02: PadNum = num(PAD02);
const NUM25: PadNum = num(PAD25);

const fn pad_n(mut costs: PadPos, n: usize) -> PadPos {
    let mut i = 0;
    while i < n {
        costs = pad(costs);

        i += 1;
    }

    costs
}

const fn pad(p: PadPos) -> PadPos {
    let mut costs = [[0; PAD_POS_SYMBOLS.len()]; PAD_POS_SYMBOLS.len()];

    let mut from = 0;
    while from < PAD_POS.len() {
        let (start_r, start_c) = PAD_POS[from];

        let mut to = 0;
        while to < PAD_POS.len() {
            let (end_r, end_c) = PAD_POS[to];

            costs[from][to] = pad_cost(
                p,
                (start_r, start_c),
                A,
                (end_r, end_c),
                key((start_r, start_c)),
            );

            to += 1;
        }

        from += 1;
    }

    costs
}

const fn num(p: PadPos) -> PadNum {
    let mut costs = [[0; PAD_NUM_SYMBOLS.len()]; PAD_NUM_SYMBOLS.len()];

    let mut from = 0;
    while from < NUM_POS.len() {
        let (start_r, start_c) = NUM_POS[from];

        let mut to = 0;
        while to < NUM_POS.len() {
            let (end_r, end_c) = NUM_POS[to];

            costs[from][to] = num_cost(
                p,
                (start_r, start_c),
                A,
                (end_r, end_c),
                key((start_r, start_c)),
            );

            to += 1;
        }

        from += 1;
    }

    costs
}

const fn key((r, c): (usize, usize)) -> u32 {
    1 << (r * 3 + c) // same width for pos and num pad
}

const fn pad_cost(
    p: PadPos,
    (start_r, start_c): (usize, usize),
    start_direction: usize,
    (end_r, end_c): (usize, usize),
    visited: u32,
) -> u64 {
    const WIDTH: usize = 3;
    const HEIGHT: usize = 2;

    if start_r == end_r && start_c == end_c {
        return p[start_direction][A];
    }

    let mut cost = u64::MAX;

    let mut i = 0;
    while i < DIRECTIONS.len() {
        let ((dr, dc), direction) = DIRECTIONS[i];
        i += 1;

        match (
            start_r.checked_add_signed(dr),
            start_c.checked_add_signed(dc),
        ) {
            (Some(r), Some(c)) if r < HEIGHT && c < WIDTH && (r != 0 || c != 0) => {
                let mask = key((r, c));
                if visited & mask != 0 {
                    continue;
                }

                let value = pad_cost(p, (r, c), direction, (end_r, end_c), visited | mask);
                if value == u64::MAX {
                    continue;
                }

                cost = min(cost, value + p[start_direction][direction]);
            }
            _ => continue,
        }
    }

    cost
}

const fn num_cost(
    p: PadPos,
    (start_r, start_c): (usize, usize),
    start_direction: usize,
    (end_r, end_c): (usize, usize),
    visited: u32,
) -> u64 {
    const WIDTH: usize = 3;
    const HEIGHT: usize = 4;

    if start_r == end_r && start_c == end_c {
        return p[start_direction][A];
    }

    let mut cost = u64::MAX;

    let mut i = 0;
    while i < DIRECTIONS.len() {
        let ((dr, dc), direction) = DIRECTIONS[i];
        i += 1;

        match (
            start_r.checked_add_signed(dr),
            start_c.checked_add_signed(dc),
        ) {
            (Some(r), Some(c)) if r < HEIGHT && c < WIDTH && (r != 3 || c != 0) => {
                let mask = key((r, c));
                if visited & mask != 0 {
                    continue;
                }

                let value = num_cost(p, (r, c), direction, (end_r, end_c), visited | mask);
                if value == u64::MAX {
                    continue;
                }

                cost = min(cost, value + p[start_direction][direction]);
            }
            _ => continue,
        }
    }

    cost
}

const fn min(a: u64, b: u64) -> u64 {
    if a <= b {
        a
    } else {
        b
    }
}

fn solve(input: &str, numpad: &PadNum) -> u64 {
    input
        .lines()
        .map(|line| {
            assert_eq!(line.chars().last(), Some('A'));

            let n = line[..3].parse::<u64>().unwrap();
            let length = iter::once(NUM_A)
                .chain(line.as_bytes().iter().take(3).map(|d| (d - b'0') as usize))
                .zip(
                    line.as_bytes()
                        .iter()
                        .take(3)
                        .map(|d| (d - b'0') as usize)
                        .chain(iter::once(NUM_A)),
                )
                .map(|(from, to)| numpad[from][to])
                .sum::<u64>();

            n * length
        })
        .sum()
}

/// # Panics
#[cfg_attr(target_os = "none", inline(never))]
pub fn solve_1(input: &str) -> u64 {
    solve(input, &NUM02)
}

/// # Panics
#[cfg_attr(target_os = "none", inline(never))]
pub fn solve_2(input: &str) -> u64 {
    solve(input, &NUM25)
}

#[cfg(feature = "input")]
pub fn part_1() -> u64 {
    solve_1(INPUT)
}

#[cfg(feature = "input")]
pub fn part_2() -> u64 {
    solve_2(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"029A
980A
179A
456A
379A";

    #[allow(clippy::unreadable_literal)]
    #[test]
    fn same_results_1() {
        assert_eq!(solve_1(INPUT), 126384);
    }
}
