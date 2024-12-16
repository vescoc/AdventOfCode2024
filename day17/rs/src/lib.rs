#![allow(clippy::must_use_candidate)]

#[cfg(feature = "input")]
use lazy_static::lazy_static;

#[cfg(feature = "input")]
lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

#[cfg(not(feature = "input"))]
pub const INPUT: &str = "";

/// # Panics
pub fn solve_1(_input: &str) -> u32 {
    todo!()
}

/// # Panics
pub fn solve_2(_input: &str) -> u32 {
    todo!()
}

#[cfg(feature = "input")]
pub fn part_1() -> u32 {
    solve_1(&INPUT)
}

#[cfg(feature = "input")]
pub fn part_2() -> u32 {
    solve_2(&INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref INPUT_1: &'static str = r#""#;
        static ref INPUT_2: &'static str = r#""#;
    }

    #[test]
    fn same_results_1() {
        assert_eq!(solve_1(&INPUT_1), 42);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&INPUT_2), 42);
    }
}
