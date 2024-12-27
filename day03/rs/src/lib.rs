#![no_std]
#![allow(clippy::must_use_candidate)]

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::u32 as parse_u32,
    combinator::map,
    error::Error,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

enum Either<L, R> {
    Left(L),
    Right(R),
}

#[cfg(feature = "input")]
pub const INPUT: &str = include_str!("../../input");

#[cfg(not(feature = "input"))]
pub const INPUT: &str = "";

const MUL_TOKEN: &[u8] = b"mul(";
const DO_TOKEN: &[u8] = b"do()";
const DONT_TOKEN: &[u8] = b"don't()";

fn mul(i: &[u8]) -> IResult<&[u8], u32> {
    map(
        delimited(
            tag(MUL_TOKEN),
            separated_pair(parse_u32, tag(","), parse_u32),
            tag(")"),
        ),
        |(a, b)| a * b,
    )(i)
}

fn parse_mul(i: &[u8]) -> IResult<&[u8], u32> {
    preceded(take_till(|c| c == MUL_TOKEN[0]), mul)(i)
}

fn parse_do(i: &[u8]) -> IResult<&[u8], &[u8]> {
    preceded(take_till(|c| c == DO_TOKEN[0]), tag(DO_TOKEN))(i)
}

fn parse_dont(i: &[u8]) -> IResult<&[u8], &[u8]> {
    preceded(take_till(|c| c == DONT_TOKEN[0]), tag(DONT_TOKEN))(i)
}

fn parse_gated_mul(i: &[u8]) -> IResult<&[u8], Either<bool, u32>> {
    alt((
        map(parse_mul, Either::Right),
        map(parse_do, |_| Either::Left(true)),
        map(parse_dont, |_| Either::Left(false)),
    ))(i)
}

pub fn solve_1_nom(input: &str) -> u32 {
    let mut input = input.as_bytes();

    let mut acc = 0;
    while !input.is_empty() {
        match parse_mul(input) {
            Ok((remainder, value)) => {
                acc += value;
                input = remainder;
            }
            Err(nom::Err::Error(Error {
                input: remaining, ..
            })) => {
                if input == remaining {
                    input = &input[1..];
                } else {
                    input = remaining;
                }
            }
            Err(_) => unreachable!(),
        }
    }

    acc
}

pub fn solve_2_nom(input: &str) -> u32 {
    let mut input = input.as_bytes();

    let mut enabled = true;
    let mut acc = 0;
    while !input.is_empty() {
        match parse_gated_mul(input) {
            Ok((remainder, Either::Right(value))) => {
                if enabled {
                    acc += value;
                }
                input = remainder;
            }
            Ok((remainder, Either::Left(value))) => {
                enabled = value;
                input = remainder;
            }
            Err(nom::Err::Error(Error {
                input: remaining, ..
            })) => {
                if input == remaining {
                    input = &input[1..];
                } else {
                    input = remaining;
                }
            }
            Err(_) => unreachable!(),
        }
    }

    acc
}

fn parse_a(input: &[u8], mut i: usize) -> Result<(usize, u32), usize> {
    let mut acc = 0;
    let mut j = 0;
    while j < 4 {
        match input.get(i) {
            Some(c @ (b'0'..=b'9')) => {
                acc = acc * 10 + u32::from(c - b'0');
                i += 1;
                j += 1;
            }
            Some(b',') => {
                if j > 0 {
                    return Ok((i + 1, acc));
                }

                return Err(i + 1);
            }
            _ => {
                return Err(i);
            }
        }
    }

    Err(i)
}

fn parse_b(input: &[u8], mut i: usize) -> Result<(usize, u32), usize> {
    let mut acc = 0;
    let mut j = 0;
    while j < 4 {
        match input.get(i) {
            Some(c @ (b'0'..=b'9')) => {
                acc = acc * 10 + u32::from(c - b'0');
                i += 1;
                j += 1;
            }
            Some(b')') => {
                if j > 0 {
                    return Ok((i + 1, acc));
                }
                return Err(i + 1);
            }
            _ => {
                return Err(i);
            }
        }
    }

    Err(i)
}

/// # Panics
pub fn solve_1_handmade(input: &str) -> u32 {
    let input = input.as_bytes();

    let mut result = 0;

    let mut i = 0;
    loop {
        match input.get(i..i + MUL_TOKEN.len()) {
            Some(MUL_TOKEN) => match parse_a(input, i + MUL_TOKEN.len()) {
                Ok((j, a)) => match parse_b(input, j) {
                    Ok((j, b)) => {
                        result += a * b;
                        i = j;
                    }
                    Err(j) => {
                        i = j;
                    }
                },
                Err(j) => {
                    i = j;
                }
            },
            Some(_) => {
                i += 1;
            }
            None => {
                break;
            }
        }
    }

    result
}

pub use solve_1_handmade as solve_1;

/// # Panics
pub fn solve_2_handmade(input: &str) -> u32 {
    let input = input.as_bytes();

    let mut result = 0;
    let mut enabled = true;

    let mut i = 0;
    loop {
        match input.get(i..) {
            Some(ss) if ss.starts_with(MUL_TOKEN) => match parse_a(input, i + MUL_TOKEN.len()) {
                Ok((j, a)) => match parse_b(input, j) {
                    Ok((j, b)) => {
                        if enabled {
                            result += a * b;
                        }
                        i = j;
                    }
                    Err(j) => {
                        i = j;
                    }
                },
                Err(j) => {
                    i = j;
                }
            },
            Some(ss) if ss.starts_with(DO_TOKEN) => {
                enabled = true;
                i += DO_TOKEN.len();
            }
            Some(ss) if ss.starts_with(DONT_TOKEN) => {
                enabled = false;
                i += DONT_TOKEN.len();
            }
            Some(_) => {
                i += 1;
            }
            None => {
                break;
            }
        }
    }

    result
}

pub use solve_2_handmade as solve_2;

#[cfg(feature = "input")]
pub fn part_1() -> u32 {
    solve_1(INPUT)
}

#[cfg(feature = "input")]
pub fn part_2() -> u32 {
    solve_2(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str =
        r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT_2: &str =
        r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn same_results_1_handmade() {
        assert_eq!(solve_1_handmade(INPUT_1), 161);
    }

    #[test]
    fn same_results_1_nom() {
        assert_eq!(solve_1_nom(INPUT_1), 161);
    }

    #[test]
    fn same_results_2_handmade() {
        assert_eq!(solve_2_handmade(INPUT_2), 48);
    }

    #[test]
    fn same_results_2_nom() {
        assert_eq!(solve_2_handmade(INPUT_2), 48);
    }
}
