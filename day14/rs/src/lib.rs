#![no_std]
#![allow(clippy::must_use_candidate)]

use heapless::{String as HLString, Vec as HLVec};

#[cfg(feature = "input")]
use lazy_static::lazy_static;

#[cfg(feature = "input")]
lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

#[cfg(not(feature = "input"))]
pub const INPUT: &str = "";

pub const WIDTH: i32 = 101;
pub const HEIGHT: i32 = 103;

type String = HLString<8>;
type Vec<T> = HLVec<T, 512>;

pub struct Robot {
    pub position: (i32, i32),
    pub velocity: (i32, i32),
}

impl Robot {
    fn new(position: (i32, i32), velocity: (i32, i32)) -> Self {
        Self { position, velocity }
    }
}

fn parse(data: &str) -> (i32, i32) {
    let mut parts = data.split(',');

    (
        parts
            .next()
            .unwrap()
            .chars()
            .skip(2)
            .collect::<String>()
            .parse::<i32>()
            .unwrap(),
        parts.next().unwrap().parse::<i32>().unwrap(),
    )
}

/// # Panics
pub fn robots(input: &str) -> impl Iterator<Item = Robot> + use<'_> {
    input.lines().map(|line| {
        let mut parts = line.split_whitespace().map(parse);
        let (px, py) = parts.next().unwrap();
        let (vx, vy) = parts.next().unwrap();

        Robot::new((px, py), (vx, vy))
    })
}

/// # Panics
#[allow(clippy::cast_sign_loss)]
pub fn solve_1<const WIDTH: i32, const HEIGHT: i32>(input: &str) -> usize {
    robots(input)
        .fold(
            [0, 0, 0, 0],
            |mut acc,
             Robot {
                 position: (px, py),
                 velocity: (vx, vy),
             }| {
                let (px, py) = (
                    (px + vx * 100).rem_euclid(WIDTH),
                    (py + vy * 100).rem_euclid(HEIGHT),
                );

                if px != WIDTH / 2 && py != HEIGHT / 2 {
                    let (px, py) = (2 * px / WIDTH, 2 * py / HEIGHT);
                    acc[px as usize * 2 + py as usize] += 1;
                }

                acc
            },
        )
        .iter()
        .product()
}

/// # Panics
pub fn solve_2(input: &str) -> usize {
    const TARGET: &[u8] = b"**********";

    let robots = robots(input).collect::<Vec<_>>();

    (0..10000)
        .position(|i| {
            let mut map = [[b' '; WIDTH as usize]; HEIGHT as usize];
            for Robot {
                position: (px, py),
                velocity: (vx, vy),
            } in &robots
            {
                let (px, py) = (
                    (px + vx * i).rem_euclid(WIDTH) as usize,
                    (py + vy * i).rem_euclid(HEIGHT) as usize,
                );
                map[py][px] = b'*';
            }

            map.into_iter()
                .any(|row| (0..row.len() - TARGET.len()).any(|i| row[i..].starts_with(TARGET)))
        })
        .unwrap()
}

#[cfg(feature = "input")]
pub fn part_1() -> usize {
    solve_1::<WIDTH, HEIGHT>(&INPUT)
}

#[cfg(feature = "input")]
pub fn part_2() -> usize {
    solve_2(&INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref INPUT: &'static str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;
    }

    #[test]
    fn same_results_1() {
        assert_eq!(solve_1::<11, 7>(&INPUT), 12);
    }
}
