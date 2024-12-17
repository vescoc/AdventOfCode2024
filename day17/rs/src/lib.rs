#![no_std]
#![allow(clippy::must_use_candidate)]

use core::fmt::Write;

use heapless::{String as HLString, Vec as HLVec};

pub type String = HLString<32>;
type Vec<T> = HLVec<T, 32>;

#[cfg(feature = "input")]
use lazy_static::lazy_static;

#[cfg(feature = "input")]
lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

#[cfg(not(feature = "input"))]
pub const INPUT: &str = "";

type Integer = u64;

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;

#[allow(clippy::cast_possible_truncation)]
fn run_to_out(pc: &mut usize, register: &mut [Integer], istructions: &[u8]) -> Option<Integer> {
    let combo = |register: &[Integer], operand| match operand {
        0..=3 => Integer::from(operand),
        4 => register[A],
        5 => register[B],
        6 => register[C],
        _ => panic!("invalid operand"),
    };

    let literal = |operand| Integer::from(operand);

    while *pc < istructions.len() {
        match istructions[*pc] {
            0 => {
                // adv
                register[A] /= 1 << combo(register, istructions[*pc + 1]);
                *pc += 2;
            }
            1 => {
                // bxl
                register[B] ^= literal(istructions[*pc + 1]);
                *pc += 2;
            }
            2 => {
                // bst
                register[B] = combo(register, istructions[*pc + 1]) & 0b111;
                *pc += 2;
            }
            3 => {
                // jnz
                if register[A] == 0 {
                    *pc += 2;
                } else {
                    *pc = literal(istructions[*pc + 1]) as usize;
                }
            }
            4 => {
                // bxc
                register[B] ^= register[C];
                *pc += 2;
            }
            5 => {
                // out
                let result = combo(register, istructions[*pc + 1]) % 8;
                *pc += 2;

                return Some(result);
            }
            6 => {
                // bdv
                register[B] = register[A] / (1 << combo(register, istructions[*pc + 1]));
                *pc += 2;
            }
            7 => {
                // cdv
                register[C] = register[A] / (1 << combo(register, istructions[*pc + 1]));
                *pc += 2;
            }
            op => unreachable!("invalid instruction {op}"),
        }
    }

    None
}

/// # Panics
#[allow(clippy::cast_possible_truncation)]
pub fn solve_1(input: &str) -> String {
    let mut parts = input.split("\n\n");

    let mut register = [0; 3];
    for line in parts.next().unwrap().lines() {
        let line = line.as_bytes();

        register[line[b"Register ".len()] as usize - b'A' as usize] = line
            .iter()
            .skip("Register A: ".len())
            .fold(0, |acc, digit| acc * 10 + Integer::from(digit - b'0'));
    }

    let istructions = {
        let line = parts.next().unwrap();
        let line = line.as_bytes();
        line["Program: ".len()..]
            .split(|&c| c == b',')
            .map(|n| n[0] - b'0')
            .collect::<Vec<_>>()
    };

    let mut out = Vec::new();
    let mut pc = 0;
    while let Some(value) = run_to_out(&mut pc, &mut register[..], &istructions) {
        out.push(value).unwrap();
    }

    let mut result = String::new();
    for (i, value) in out.iter().enumerate() {
        result.write_char(char::from(*value as u8 + b'0')).unwrap();
        if i != out.len() - 1 {
            result.write_char(',').unwrap();
        }
    }

    result
}

/// # Panics
pub fn solve_2(input: &str) -> Integer {
    let mut parts = input.split("\n\n").skip(1);

    let istructions = {
        let line = parts.next().unwrap();
        let line = line.as_bytes();
        line["Program: ".len()..]
            .split(|&c| c == b',')
            .map(|n| n[0] - b'0')
            .collect::<Vec<_>>()
    };

    let mut buffers = [Vec::new(), Vec::new()];
    buffers[0]
        .push(Integer::from(istructions.last().copied().unwrap()))
        .unwrap();

    let buffer = istructions
        .iter()
        .rev()
        .fold(0, |current_buffer_index, &istruction| {
            let (prev, next) = {
                let (a, b) = buffers.split_at_mut(1);
                if current_buffer_index == 0 {
                    (&a[0], &mut b[0])
                } else {
                    (&b[0], &mut a[0])
                }
            };

            next.clear();
            for v in prev {
                for n in 0..8 {
                    let a = (v << 3) | n; // v * 8 + n

                    let mut pc = 0;
                    let mut register = [a, 0, 0];
                    if let Some(out) = run_to_out(&mut pc, &mut register[..], &istructions) {
                        if out == istruction.into() {
                            next.push(a).unwrap();
                        }
                    }
                }
            }

            (current_buffer_index + 1) % 2
        });

    buffers[buffer].iter().min().copied().unwrap()
}

#[cfg(feature = "input")]
pub fn part_1() -> String {
    solve_1(&INPUT)
}

#[cfg(feature = "input")]
pub fn part_2() -> Integer {
    // input
    // bst A // 2,4   - B <- A & 0x111
    // bxl 1 // 1,1   - B <- B ^ [1]
    // cdv B // 7,5   - C <- A >> B
    // bxc   // 4,4   - B <- B ^ C = B ^ (A >> B)
    // bxl 4 // 1,4   - B <- B ^ 0b100
    // adv 3 // 0,3   - A <- A >> 3
    // out B // 5,5   -   -> B & 0x111
    // jnz 0 // 3,0   - goto 0

    // example
    // adv 3 // 0,3   - A <- A >> 3
    // out B // 5,4   -   -> B & 0x111
    // jnz 0 // 3,0   - goto 0

    solve_2(&INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref INPUT_1: &'static str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;
        static ref INPUT_2: &'static str = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"#;
    }

    #[test]
    fn same_results_1() {
        assert_eq!(
            solve_1(&INPUT_1),
            String::try_from("4,6,3,5,6,3,5,2,1,0").unwrap()
        );
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&INPUT_2), 117440);
    }
}
