#![no_std]
#![allow(clippy::must_use_candidate)]

use bitset::BitSet;

use heapless::{Entry, FnvIndexMap, Vec as HLVec};

const MAP_SIZE: usize = 64;
const SET_SIZE: usize = BitSet::with_capacity(MAP_SIZE * MAP_SIZE);

type HashMap<K, V> = FnvIndexMap<K, V, MAP_SIZE>;
type Vec<T> = HLVec<T, 16>;

#[cfg(feature = "input")]
pub const INPUT: &str = include_str!("../../input");

#[cfg(not(feature = "input"))]
pub const INPUT: &str = "";

type Point = (usize, usize);

struct Antennas(HashMap<u8, Vec<Point>>);

impl Antennas {
    const fn new() -> Self {
        Self(HashMap::new())
    }
    
    fn add(&mut self, key: u8, value: &Point) -> Result<(), Point> {
        match self.0.entry(key) {
            Entry::Occupied(mut v) => {
                v.get_mut().push(*value)?;
            }
            Entry::Vacant(v) => {
                v.insert(Vec::from_slice(&[*value]).map_err(|()| *value)?).map_err(|_| *value)?;
            }
        }

        Ok(())
    }
}

impl core::ops::Deref for Antennas {
    type Target = HashMap<u8, Vec<Point>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[allow(clippy::cast_possible_wrap)]
fn calculate_antinode((height, width): Point, a: &Point, b: &Point) -> impl Iterator<Item = Point> + use<> {
    let (dr, dc) = (b.0 as isize - a.0 as isize, b.1 as isize - a.1 as isize);
    let r = match (a.0.checked_add_signed(-dr), a.1.checked_add_signed(-dc)) {
        (Some(r), Some(c)) if r < height && c < width => Some((r, c)),
        _ => None,
    };
    r.into_iter()
}

#[allow(clippy::cast_possible_wrap)]
fn calculate_antinodes(
    (height, width): Point,
    a: &Point,
    b: &Point,
) -> impl Iterator<Item = Point> + use<> {
    let (a, b) = (*a, *b);

    let (dr, dc) = (b.0 as isize - a.0 as isize, b.1 as isize - a.1 as isize);

    let mut index = 0;
    core::iter::from_fn(move || {
        let r = match (
            a.0.checked_add_signed(-dr * index),
            a.1.checked_add_signed(-dc * index),
        ) {
            (Some(r), Some(c)) if r < height && c < width => Some((r, c)),
            _ => None,
        };
        index += 1;
        r
    })
    .fuse()
}

/// # Panics
pub fn solve<F, I>(input: &str, calculate_antinodes: F) -> usize
where
    for<'a> F: Fn(Point, &'a Point, &'a Point) -> I + 'a,
    I: Iterator<Item = Point>,
{
    let map = input.as_bytes();
    let width = map.iter().position(|&c| c == b'\n').unwrap();
    let height = (map.len() + 1) / (width + 1);
    
    let mut antennas = Antennas::new();
    for (tile, point) in map
        .chunks(width + 1)
        .enumerate()
        .flat_map(|(r, row)| {
            // taking also the last char ot the row: it is a newline (if exists) :P
            row.iter().enumerate().filter_map(move |(c, &tile)| {
                if tile.is_ascii_alphanumeric() {
                    Some((tile, (r, c)))
                } else {
                    None
                }
            })
        })
    {
        antennas.add(tile, &point).unwrap();
    }

    let mut antinodes =
        BitSet::<Point, _, SET_SIZE>::new(|(r, c)| r * width + c);
    for nodes in antennas.values() {
        for (i, a) in nodes.iter().enumerate().take(nodes.len() - 1) {
            for b in nodes.iter().skip(i + 1) {
                for antinode in calculate_antinodes((height, width), a, b) {
                    antinodes.insert(antinode).unwrap();
                }
                for antinode in calculate_antinodes((height, width), b, a) {
                    antinodes.insert(antinode).unwrap();
                }
            }
        }
    }

    antinodes.len()
}

/// # Panics
#[cfg_attr(target_os = "none", inline(never))]
pub fn solve_1(input: &str) -> usize {
    solve(input, calculate_antinode)
}

/// # Panics
#[cfg_attr(target_os = "none", inline(never))]
pub fn solve_2(input: &str) -> usize {
    solve(input, calculate_antinodes)
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

    const INPUT_1: &str = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    const INPUT_2: &str = r"T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
..........";

    #[test]
    fn same_results_1() {
        assert_eq!(solve_1(INPUT_1), 14);
    }

    #[test]
    fn same_results_2_1() {
        assert_eq!(solve_2(INPUT_2), 9);
    }

    #[test]
    fn same_results_2_2() {
        assert_eq!(solve_2(INPUT_1), 34);
    }
}
