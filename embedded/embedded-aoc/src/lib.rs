#![no_std]

use core::fmt::Write as _;

use core::fmt;
use core::ops;

use embedded_io::{Read, Write};

use fugit::{Duration, Instant};

use heapless::{String as HLString, Vec as HLVec};

#[cfg(feature = "defmt")]
use defmt::{info, warn};

#[cfg(feature = "log")]
use log::{info, warn};

type Vec<T> = HLVec<T, { 1024 * 32 }>;
type PartResult = HLString<64>;

const START_INPUT_TAG: &str = "START INPUT DAY: ";
const END_INPUT_TAG: &str = "END INPUT";

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone)]
pub enum Day {
    #[cfg(feature = "day01")]
    Day01,
    #[cfg(feature = "day02")]
    Day02,
    #[cfg(feature = "day03")]
    Day03,
    #[cfg(feature = "day04")]
    Day04,
    #[cfg(feature = "day05")]
    Day05,
    #[cfg(feature = "day06")]
    Day06,
    #[cfg(feature = "day07")]
    Day07,
    #[cfg(feature = "day08")]
    Day08,
    #[cfg(feature = "day09")]
    Day09,
    #[cfg(feature = "day10")]
    Day10,
    #[cfg(feature = "day11")]
    Day11,
    #[cfg(feature = "day12")]
    Day12,
    #[cfg(feature = "day13")]
    Day13,
    #[cfg(feature = "day14")]
    Day14,
    #[cfg(feature = "day15")]
    Day15,
    #[cfg(feature = "day16")]
    Day16,
    #[cfg(feature = "day17")]
    Day17,
    #[cfg(feature = "day18")]
    Day18,
    #[cfg(feature = "day19")]
    Day19,
    #[cfg(feature = "day20")]
    Day20,
    #[cfg(feature = "day21")]
    Day21,
    #[cfg(feature = "day22")]
    Day22,
    #[cfg(feature = "day23")]
    Day23,
    #[cfg(feature = "day24")]
    Day24,
    #[cfg(feature = "day25")]
    Day25,
}

impl Day {
    fn to_string(result: &mut PartResult, value: impl fmt::Display) -> Result<(), fmt::Error> {
        write!(result, "{value}")
    }

    fn solve_1(self, result: &mut PartResult, input: &str) -> Result<(), fmt::Error> {
        match self {
            #[cfg(feature = "day01")]
            Day::Day01 => Self::to_string(result, day01::solve_1(input)),
            #[cfg(feature = "day02")]
            Day::Day02 => Self::to_string(result, day02::solve_1(input)),
            #[cfg(feature = "day03")]
            Day::Day03 => Self::to_string(result, day03::solve_1(input)),
            #[cfg(feature = "day04")]
            Day::Day04 => Self::to_string(result, day04::solve_1(input)),
            #[cfg(feature = "day05")]
            Day::Day05 => Self::to_string(result, day05::solve_1(input)),
            #[cfg(feature = "day06")]
            Day::Day06 => Self::to_string(result, day06::solve_1(input)),
            #[cfg(feature = "day07")]
            Day::Day07 => Self::to_string(result, day07::solve_1(input)),
            #[cfg(feature = "day08")]
            Day::Day08 => Self::to_string(result, day08::solve_1(input)),
            #[cfg(feature = "day09")]
            Day::Day09 => Self::to_string(result, day09::solve_1(input)),
            #[cfg(feature = "day10")]
            Day::Day10 => Self::to_string(result, day10::solve_1(input)),
            #[cfg(feature = "day11")]
            Day::Day11 => Self::to_string(result, day11::solve_1(input)),
            #[cfg(feature = "day12")]
            Day::Day12 => Self::to_string(result, day12::solve_1(input)),
            #[cfg(feature = "day13")]
            Day::Day13 => Self::to_string(result, day13::solve_1(input)),
            #[cfg(feature = "day14")]
            Day::Day14 => Self::to_string(
                result,
                day14::solve_1::<{ day14::WIDTH }, { day14::HEIGHT }>(input),
            ),
            #[cfg(feature = "day15")]
            Day::Day15 => Self::to_string(result, day15::solve_1(input)),
            #[cfg(feature = "day16")]
            Day::Day16 => Self::to_string(result, day16::solve_1(input)),
            #[cfg(feature = "day17")]
            Day::Day17 => Self::to_string(result, day17::solve_1(input)),
            #[cfg(feature = "day18")]
            Day::Day18 => Self::to_string(result, day18::solve_1(input)),
            #[cfg(feature = "day19")]
            Day::Day19 => Self::to_string(result, day19::solve_1(input)),
            #[cfg(feature = "day20")]
            Day::Day20 => Self::to_string(result, day20::solve_1(input)),
            #[cfg(feature = "day21")]
            Day::Day21 => Self::to_string(result, day21::solve_1(input)),
            #[cfg(feature = "day22")]
            Day::Day22 => Self::to_string(result, day22::solve_1(input)),
            #[cfg(feature = "day23")]
            Day::Day23 => Self::to_string(result, day23::solve_1(input)),
            #[cfg(feature = "day24")]
            Day::Day24 => Self::to_string(result, day24::solve_1(input)),
            #[cfg(feature = "day25")]
            Day::Day25 => Self::to_string(result, day25::solve_1(input)),
        }
    }

    fn solve_2(self, result: &mut PartResult, input: &str) -> Result<(), fmt::Error> {
        match self {
            #[cfg(feature = "day01")]
            Day::Day01 => Self::to_string(result, day01::solve_2(input)),
            #[cfg(feature = "day02")]
            Day::Day02 => Self::to_string(result, day02::solve_2(input)),
            #[cfg(feature = "day03")]
            Day::Day03 => Self::to_string(result, day03::solve_2(input)),
            #[cfg(feature = "day04")]
            Day::Day04 => Self::to_string(result, day04::solve_2(input)),
            #[cfg(feature = "day05")]
            Day::Day05 => Self::to_string(result, day05::solve_2(input)),
            #[cfg(feature = "day06")]
            Day::Day06 => Self::to_string(result, day06::solve_2(input)),
            #[cfg(feature = "day07")]
            Day::Day07 => Self::to_string(result, day07::solve_2(input)),
            #[cfg(feature = "day08")]
            Day::Day08 => Self::to_string(result, day08::solve_2(input)),
            #[cfg(feature = "day09")]
            Day::Day09 => Self::to_string(result, day09::solve_2(input)),
            #[cfg(feature = "day10")]
            Day::Day10 => Self::to_string(result, day10::solve_2(input)),
            #[cfg(feature = "day11")]
            Day::Day11 => Self::to_string(result, day11::solve_2(input)),
            #[cfg(feature = "day12")]
            Day::Day12 => Self::to_string(result, day12::solve_2(input)),
            #[cfg(feature = "day13")]
            Day::Day13 => Self::to_string(result, day13::solve_2(input)),
            #[cfg(feature = "day14")]
            Day::Day14 => Self::to_string(result, day14::solve_2(input)),
            #[cfg(feature = "day15")]
            Day::Day15 => Self::to_string(result, day15::solve_2(input)),
            #[cfg(feature = "day16")]
            Day::Day16 => Self::to_string(result, day16::solve_2(input)),
            #[cfg(feature = "day17")]
            Day::Day17 => Self::to_string(result, day17::solve_2(input)),
            #[cfg(feature = "day18")]
            Day::Day18 => Self::to_string(result, day18::solve_2(input)),
            #[cfg(feature = "day19")]
            Day::Day19 => Self::to_string(result, day19::solve_2(input)),
            #[cfg(feature = "day20")]
            Day::Day20 => Self::to_string(result, day20::solve_2(input)),
            #[cfg(feature = "day21")]
            Day::Day21 => Self::to_string(result, day21::solve_2(input)),
            #[cfg(feature = "day22")]
            Day::Day22 => Self::to_string(result, day22::solve_2(input)),
            #[cfg(feature = "day23")]
            Day::Day23 => Self::to_string(result, day23::solve_2(input)),
            #[cfg(feature = "day24")]
            Day::Day24 => Self::to_string(result, day24::solve_2(input)),
            #[cfg(feature = "day25")]
            Day::Day25 => Self::to_string(result, day25::solve_2(input)),
        }
    }
}

impl core::str::FromStr for Day {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.chars().take(2).try_fold(0, |acc, digit| match digit {
            '0'..='9' => Some(acc * 10 + digit as u32 - '0' as u32),
            _ => None,
        }) {
            #[cfg(feature = "day01")]
            Some(1) => Ok(Day::Day01),
            #[cfg(feature = "day02")]
            Some(2) => Ok(Day::Day02),
            #[cfg(feature = "day03")]
            Some(3) => Ok(Day::Day03),
            #[cfg(feature = "day04")]
            Some(4) => Ok(Day::Day04),
            #[cfg(feature = "day05")]
            Some(5) => Ok(Day::Day05),
            #[cfg(feature = "day06")]
            Some(6) => Ok(Day::Day06),
            #[cfg(feature = "day07")]
            Some(7) => Ok(Day::Day07),
            #[cfg(feature = "day08")]
            Some(8) => Ok(Day::Day08),
            #[cfg(feature = "day09")]
            Some(9) => Ok(Day::Day09),
            #[cfg(feature = "day10")]
            Some(10) => Ok(Day::Day10),
            #[cfg(feature = "day11")]
            Some(11) => Ok(Day::Day11),
            #[cfg(feature = "day12")]
            Some(12) => Ok(Day::Day12),
            #[cfg(feature = "day13")]
            Some(13) => Ok(Day::Day13),
            #[cfg(feature = "day14")]
            Some(14) => Ok(Day::Day14),
            #[cfg(feature = "day15")]
            Some(15) => Ok(Day::Day15),
            #[cfg(feature = "day16")]
            Some(16) => Ok(Day::Day16),
            #[cfg(feature = "day17")]
            Some(17) => Ok(Day::Day17),
            #[cfg(feature = "day18")]
            Some(18) => Ok(Day::Day18),
            #[cfg(feature = "day19")]
            Some(19) => Ok(Day::Day19),
            #[cfg(feature = "day20")]
            Some(20) => Ok(Day::Day20),
            #[cfg(feature = "day21")]
            Some(21) => Ok(Day::Day21),
            #[cfg(feature = "day22")]
            Some(22) => Ok(Day::Day22),
            #[cfg(feature = "day23")]
            Some(23) => Ok(Day::Day23),
            #[cfg(feature = "day24")]
            Some(24) => Ok(Day::Day24),
            #[cfg(feature = "day25")]
            Some(25) => Ok(Day::Day25),
            Some(_) => Err("invalid day"),
            None => Err("invalid number"),
        }
    }
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let day = match self {
            #[cfg(feature = "day01")]
            Day::Day01 => 1,
            #[cfg(feature = "day02")]
            Day::Day02 => 2,
            #[cfg(feature = "day03")]
            Day::Day03 => 3,
            #[cfg(feature = "day04")]
            Day::Day04 => 4,
            #[cfg(feature = "day05")]
            Day::Day05 => 5,
            #[cfg(feature = "day06")]
            Day::Day06 => 6,
            #[cfg(feature = "day07")]
            Day::Day07 => 7,
            #[cfg(feature = "day08")]
            Day::Day08 => 8,
            #[cfg(feature = "day09")]
            Day::Day09 => 9,
            #[cfg(feature = "day10")]
            Day::Day10 => 10,
            #[cfg(feature = "day11")]
            Day::Day11 => 11,
            #[cfg(feature = "day12")]
            Day::Day12 => 12,
            #[cfg(feature = "day13")]
            Day::Day13 => 13,
            #[cfg(feature = "day14")]
            Day::Day14 => 14,
            #[cfg(feature = "day15")]
            Day::Day15 => 15,
            #[cfg(feature = "day16")]
            Day::Day16 => 16,
            #[cfg(feature = "day17")]
            Day::Day17 => 17,
            #[cfg(feature = "day18")]
            Day::Day18 => 18,
            #[cfg(feature = "day19")]
            Day::Day19 => 19,
            #[cfg(feature = "day20")]
            Day::Day20 => 20,
            #[cfg(feature = "day21")]
            Day::Day21 => 21,
            #[cfg(feature = "day22")]
            Day::Day22 => 22,
            #[cfg(feature = "day23")]
            Day::Day23 => 23,
            #[cfg(feature = "day24")]
            Day::Day24 => 24,
            #[cfg(feature = "day25")]
            Day::Day25 => 25,
        };

        write!(f, "{day:02}")
    }
}

pub trait Timer<T, const NOM: u32, const DENOM: u32> {
    fn now(&self) -> Instant<T, NOM, DENOM>;
}

pub trait Handler<T, const NOM: u32, const DENOM: u32> {
    fn started(&mut self, _day: Day, _timestamp: Instant<T, NOM, DENOM>) {}
    fn ended(
        &mut self,
        _day: Day,
        _elapsed: Duration<T, NOM, DENOM>,
        _part_1: &str,
        _part_2: &str,
    ) {
    }
    fn unsupported_day(&mut self) {}
    fn invalid_input(&mut self) {}
}

#[derive(Default)]
pub struct DummyHandler<T, const NOM: u32, const DENOM: u32> {
    _t: core::marker::PhantomData<T>,
}

impl<T, const NOM: u32, const DENOM: u32> Handler<T, NOM, DENOM> for DummyHandler<T, NOM, DENOM> {}

/// # Panics
pub fn run<const NOM: u32, const DENOM: u32>(
    (mut rx, mut tx): (impl Read, impl Write),
    timer: &impl Timer<u64, NOM, DENOM>,
    mut handler: impl Handler<u64, NOM, DENOM>,
) -> !
where
    Instant<u64, NOM, DENOM>: ops::Sub<Output = Duration<u64, NOM, DENOM>>,
{
    let mut buf = [0u8; 64];
    let mut buffer = Vec::new();
    loop {
        buffer.clear();
        loop {
            match rx.read(&mut buf) {
                Err(_) | Ok(0) => {}
                Ok(count) => {
                    if buffer.extend_from_slice(&buf[..count]).is_err() {
                        warn!("buffer overflow");
                        break;
                    }

                    if let Ok(input) = core::str::from_utf8(&buffer) {
                        match (input.find(START_INPUT_TAG), input.find(END_INPUT_TAG)) {
                            (Some(start_position), Some(end_position)) => {
                                let Ok(day) =
                                    input[start_position + START_INPUT_TAG.len()..].parse::<Day>()
                                else {
                                    warn!("unsupported day");

                                    handler.unsupported_day();

                                    write!(&mut tx, "unsupported day\r\n").unwrap();

                                    break;
                                };

                                let input = input
                                    [start_position + START_INPUT_TAG.len() + 2..end_position]
                                    .trim();

                                info!("[{}] start working on {}", day, day);

                                let mut part_1 = PartResult::new();
                                let mut part_2 = PartResult::new();

                                let start = timer.now();

                                handler.started(day, start);

                                if day.solve_1(&mut part_1, input).is_err() {
                                    warn!("part_1: buffer overflow");
                                    break;
                                }

                                if day.solve_2(&mut part_2, input).is_err() {
                                    warn!("part_2: buffer overflow");
                                    break;
                                }

                                let elapsed = timer.now() - start;

                                handler.ended(day, elapsed, part_1.as_str(), part_2.as_str());

                                info!("[{}] part 1: {}", day, part_1.as_str());
                                write!(&mut tx, "[{day}] part 1: {part_1}\r\n").unwrap();

                                info!("[{}] part 2: {}", day, part_2.as_str());
                                write!(&mut tx, "[{day}] part 2: {part_2}\r\n").unwrap();

                                info!(
                                    "[{}] elapsed: {}ms ({}us)",
                                    day,
                                    elapsed.to_millis(),
                                    elapsed.to_micros()
                                );
                                write!(
                                    &mut tx,
                                    "[{day}] elapsed: {}ms ({}us)\r\n",
                                    elapsed.to_millis(),
                                    elapsed.to_micros()
                                )
                                .unwrap();

                                break;
                            }
                            (None, Some(_)) => {
                                warn!("invalid input");

                                handler.invalid_input();

                                write!(&mut tx, "invalid input\r\n").unwrap();

                                break;
                            }
                            _ => {}
                        }
                    } else {
                        warn!("invalid utf8 data");
                        break;
                    }
                }
            }
        }
    }
}
