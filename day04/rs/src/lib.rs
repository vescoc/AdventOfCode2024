#![no_std]
#![allow(clippy::must_use_candidate)]

#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[cfg(feature = "input")]
pub const INPUT: &str = include_str!("../../input");

#[cfg(not(feature = "input"))]
pub const INPUT: &str = "";

/// # Panics
pub fn solve_1(input: &str) -> usize {
    const XMAS: &[u8] = b"XMAS";

    let data = input.as_bytes();

    let width = data.iter().position(|&c| c == b'\n').unwrap();

    #[cfg(feature = "parallel")]
    let rows = data.par_chunks(width + 1);

    #[cfg(not(feature = "parallel"))]
    let rows = data.chunks(width + 1);

    rows.enumerate()
        .map(|(r, row)| {
            let mut result = 0;
            for (c, _) in row.iter().enumerate() {
                // horizontal
                if matches!(row.get(c..c + 4), Some(XMAS)) {
                    result += 1;
                }

                // horizontal backward
                if let Some(d) = row.get(c..c + 4) {
                    if d.iter().rev().zip(XMAS).all(|(a, b)| a == b) {
                        result += 1;
                    }
                }

                // vertical
                if XMAS.iter().enumerate().all(|(i, tile)| matches!(data.get((r + i) * (width + 1) + c), Some(target) if target == tile)) {
                    result += 1;
                }

                // vertical backward
                if XMAS.iter().rev().enumerate().all(|(i, tile)| matches!(data.get((r + i) * (width + 1) + c), Some(target) if target == tile)) {
                    result += 1;
                }

                // diagonal right
                if XMAS.iter().enumerate().all(|(i, tile)| matches!(data.get((r + i) * (width + 1) + c + i), Some(target) if target == tile)) {
                    result += 1;
                }

                // diagonal right backward
                if XMAS.iter().rev().enumerate().all(|(i, tile)| matches!(data.get((r + i) * (width + 1) + c + i), Some(target) if target == tile)) {
                    result += 1;
                }

                // diagonal left
                if XMAS.iter().enumerate().all(|(i, tile)| c >= i && matches!(data.get((r + i) * (width + 1) + c - i), Some(target) if target == tile)) {
                    result += 1;
                }

                // diagonal left backward
                if XMAS.iter().rev().enumerate().all(|(i, tile)| c >= i && matches!(data.get((r + i) * (width + 1) + c - i), Some(target) if target == tile)) {
                    result += 1;
                }
            }

            result
        })
        .sum()
}

/// # Panics
pub fn solve_2(input: &str) -> usize {
    let data = input.as_bytes();

    let width = data.iter().position(|&c| c == b'\n').unwrap();
    let height = (data.len() + 1) / width;

    let check_ms = |a, b| a != b && (a == b'M' || a == b'S') && (b == b'M' || b == b'S');

    #[cfg(feature = "parallel")]
    let rows = data.par_chunks(width + 1);

    #[cfg(not(feature = "parallel"))]
    let rows = data.chunks(width + 1);

    rows.enumerate()
        .take(height - 1)
        .skip(1)
        .map(|(r, row)| {
            row.iter()
                .enumerate()
                .take(width - 1)
                .skip(1)
                .filter(move |(c, &tile)| {
                    tile == b'A'
                        && matches!(
                            (
                                data.get((r - 1) * (width + 1) + c - 1),
                                data.get((r + 1) * (width + 1) + c + 1),
                                data.get((r - 1) * (width + 1) + c + 1),
                                data.get((r + 1) * (width + 1) + c - 1),
                            ),
                            (Some(&ul), Some(&br), Some(&ur), Some(&bl)) if check_ms(ul, br) && check_ms(ur, bl))
                })
                .count()
        })
        .sum()
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

    const INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    #[test]
    fn same_results_1() {
        assert_eq!(solve_1(&INPUT), 18);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&INPUT), 9);
    }
}
