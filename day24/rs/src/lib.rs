#![no_std]
#![allow(clippy::must_use_candidate)]

use core::ops;

use heapless::{FnvIndexMap, String as HLString, Vec as HLVec};

type Values<K, V> = FnvIndexMap<K, V, 1024>;
type Instructions<'a> = HLVec<Instruction<'a>, 512>;
type Z<'a> = HLVec<&'a str, 64>;
type Bads<T> = HLVec<T, 8>;
type String = HLString<64>;

#[cfg(feature = "input")]
pub const INPUT: &str = include_str!("../../input");

#[cfg(not(feature = "input"))]
pub const INPUT: &str = "";

#[derive(Copy, Clone, Debug)]
enum Gate {
    And,
    Or,
    Xor,
}

impl Gate {
    fn evaluate<T, O>(self, a: T, b: T) -> O
    where
        T: ops::BitAnd<Output = O>,
        T: ops::BitOr<Output = O>,
        T: ops::BitXor<Output = O>,
    {
        match self {
            Gate::And => a & b,
            Gate::Or => a | b,
            Gate::Xor => a ^ b,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Instruction<'a> {
    left: &'a str,
    right: &'a str,
    output: &'a str,
    gate: Gate,
}

fn parse_initial_values<'a>(input: &'a str, initial_values: &mut Values<&'a str, bool>) {
    for line in input.lines() {
        let (line, value) = line.split_once(": ").unwrap();
        let value = match value {
            "1" => true,
            "0" => false,
            _ => unreachable!("invalid intial values input"),
        };

        initial_values.insert(line, value).unwrap();
    }
}

fn parse_logic<'a>(input: &'a str, instructions: &mut Instructions<'a>, z: &mut Z<'a>) {
    for line in input.lines() {
        let (gate, output) = line.split_once(" -> ").unwrap();

        let mut gate_parts = gate.split(' ');

        let left = gate_parts.next().unwrap();
        let gate = match gate_parts.next() {
            Some("AND") => Gate::And,
            Some("OR") => Gate::Or,
            Some("XOR") => Gate::Xor,
            _ => unreachable!("invalid logic input"),
        };
        let right = gate_parts.next().unwrap();

        if output.starts_with('z') {
            z.push(output).unwrap();
        }

        instructions
            .push(Instruction {
                left,
                right,
                output,
                gate,
            })
            .unwrap();
    }

    z.sort_unstable();
}

/// # Panics
pub fn solve_1(input: &str) -> u64 {
    fn get_z(z: &[&str], values: &Values<&str, bool>) -> Option<u64> {
        z.iter().enumerate().try_fold(0, |acc, (bit, z)| {
            values
                .get(z)
                .map(|&value| if value { acc | 1 << bit } else { acc })
        })
    }

    let (initial_values, logic) = input.split_once("\n\n").unwrap();

    let mut values = Values::new();
    parse_initial_values(initial_values, &mut values);

    let mut instructions = Instructions::new();
    let mut z = Z::new();
    parse_logic(logic, &mut instructions, &mut z);

    loop {
        if let Some(value) = get_z(&z, &values) {
            return value;
        }

        for Instruction {
            left,
            right,
            output,
            gate,
        } in &instructions
        {
            if let (Some(&a), Some(&b)) = (values.get(left), values.get(right)) {
                values.insert(output, gate.evaluate(a, b)).unwrap();
            }
        }
    }
}

/// # Panics
pub fn solve_2(input: &str) -> String {
    #[allow(dead_code)]
    #[derive(Debug)]
    enum Bad<'a> {
        Xor(Instruction<'a>, &'a str),
        And(Instruction<'a>, &'a str),
        Or(Instruction<'a>, &'a str),
        Carry(Instruction<'a>, &'a str),
        S(Instruction<'a>, &'a str),
    }

    impl Bad<'_> {
        fn output(&self) -> &str {
            match self {
                Bad::Xor(_, output)
                | Bad::And(_, output)
                | Bad::Or(_, output)
                | Bad::Carry(_, output)
                | Bad::S(_, output) => output,
            }
        }
    }

    fn is_inner(pin: &str) -> bool {
        !is_input(pin) && !is_output(pin)
    }

    fn is_input(pin: &str) -> bool {
        pin.starts_with('x') || pin.starts_with('y')
    }

    fn is_output(pin: &str) -> bool {
        pin.starts_with('z')
    }

    fn is_carry_input<'a>(instructions: &[Instruction<'a>], pin: &'a str) -> bool {
        instructions
            .iter()
            .any(|instruction| matches!(instruction, Instruction { left, right, gate: Gate::Or, .. } if pin == *left || pin == *right))
    }

    fn is_s_input<'a>(instructions: &[Instruction<'a>], pin: &'a str) -> bool {
        !instructions
            .iter()
            .any(|instruction| matches!(instruction, Instruction { left, right, gate: Gate::Or, .. } if pin == *left || pin == *right))
    }

    let (_, logic) = input.split_once("\n\n").unwrap();

    let mut instructions = Instructions::new();
    let mut z = Z::new();
    parse_logic(logic, &mut instructions, &mut z);

    let mut bads = Bads::new();
    for instruction in &instructions {
        match instruction {
            Instruction {
                left,
                right,
                output,
                gate: Gate::Xor,
            } if is_inner(left) && is_inner(right) && is_inner(output) => {
                bads.push(Bad::Xor(*instruction, output)).unwrap();
            }
            Instruction {
                output,
                gate: Gate::And,
                ..
            } if is_output(output) => {
                bads.push(Bad::And(*instruction, output)).unwrap();
            }
            Instruction {
                output,
                gate: Gate::Or,
                ..
            } if is_output(output) && output != z.iter().last().unwrap() => {
                bads.push(Bad::Or(*instruction, output)).unwrap();
            }
            Instruction {
                left,
                right,
                output,
                gate: Gate::And,
                ..
            } if !is_carry_input(&instructions, output)
                && (!left.ends_with("00") || !right.ends_with("00")) =>
            {
                bads.push(Bad::Carry(*instruction, output)).unwrap();
            }
            Instruction {
                output,
                gate: Gate::Xor,
                ..
            } if is_inner(output) && !is_s_input(&instructions, output) => {
                bads.push(Bad::S(*instruction, output)).unwrap();
            }
            _ => {}
        }
    }

    assert!(bads.len() == 8, "bads invalid");

    let mut v = bads.iter().map(Bad::output).collect::<Bads<_>>();
    v.sort_unstable();

    let mut result = String::new();
    for bad in v.iter().take(v.len() - 1) {
        result.push_str(bad).unwrap();
        result.push(',').unwrap();
    }
    result.push_str(v.iter().last().unwrap()).unwrap();

    result
}

#[cfg(feature = "input")]
pub fn part_1() -> u64 {
    solve_1(INPUT)
}

#[cfg(feature = "input")]
pub fn part_2() -> String {
    solve_2(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = r"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

    const INPUT_2: &str = r"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[test]
    fn same_results_1_1() {
        assert_eq!(solve_1(INPUT_1), 4);
    }

    #[test]
    fn same_results_1_2() {
        assert_eq!(solve_1(INPUT_2), 2024);
    }
}
