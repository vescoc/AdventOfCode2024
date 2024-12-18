#![allow(clippy::must_use_candidate)]

use heapless::Deque as HLDeque;

use bitset::BitSet;

#[cfg(feature = "input")]
use lazy_static::lazy_static;

#[cfg(feature = "input")]
lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

#[cfg(not(feature = "input"))]
pub const INPUT: &str = "";

const PUZZLE_WIDTH: usize = 71;
const PUZZLE_HEIGHT: usize = 71;
const BITSET_SIZE: usize = BitSet::with_capacity(PUZZLE_WIDTH * PUZZLE_HEIGHT);

type Deque<T> = HLDeque<T, { PUZZLE_WIDTH * 2 }>;

/// # Panics
fn solve_1_gen<const WIDTH: usize, const HEIGHT: usize, const TAKE: usize, const SIZE: usize>(
    input: &str,
) -> usize {
    let mut map = [[b'.'; WIDTH]; HEIGHT];
    for (x, y) in input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .take(TAKE)
    {
        map[y][x] = b'#';
    }

    let mut visited = BitSet::<_, _, SIZE>::new(|(x, y)| y * WIDTH + x);
    visited.insert((0, 0)).unwrap();

    let mut queue = Deque::new();
    queue.push_back(((0, 0), 0)).unwrap();

    while let Some(((x, y), steps)) = queue.pop_front() {
        if x == WIDTH - 1 && y == HEIGHT - 1 {
            return steps;
        }

        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            match (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
                (Some(x), Some(y))
                    if matches!(map.get(y).and_then(|row| row.get(x)), Some(b'.')) =>
                {
                    if !visited.insert((x, y)).unwrap() {
                        queue.push_back(((x, y), steps + 1)).unwrap();
                    }
                }
                _ => {}
            }
        }
    }

    unreachable!()
}

/// # Panics
fn solve_2_gen<const WIDTH: usize, const HEIGHT: usize, const SIZE: usize>(input: &str) -> String {
    use core::fmt;

    struct Info<T, K, const S: usize>((usize, usize), usize, Box<BitSet<T, K, S>>);

    impl<T, K, const S: usize> fmt::Debug for Info<T, K, S> {
        fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            write!(f, "Info({:?}, {}, [{}]", self.0, self.1, self.2.len())
        }
    }

    let key = |(x, y): &(usize, usize)| y * WIDTH + x;

    let mut last_found_path: Option<Box<BitSet<(usize, usize), _, SIZE>>> = None;

    let mut map = [[b'.'; WIDTH]; HEIGHT];
    for (x, y) in input.lines().map(|line| {
        let (x, y) = line.split_once(',').unwrap();
        (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
    }) {
        map[y][x] = b'#';

        let mut found = false;
        if last_found_path
            .as_ref()
            .map_or(true, |set| set.contains(&(x, y)).unwrap())
        {
            let mut path = BitSet::<_, _, SIZE>::new(key);
            path.insert((0, 0)).unwrap();

            let mut visited = BitSet::<_, _, SIZE>::new(key);
            visited.insert((0, 0)).unwrap();

            let mut queue = Deque::new();
            queue.push_back(Info((0, 0), 0, Box::new(path))).unwrap();

            while let Some(Info((x, y), steps, path)) = queue.pop_front() {
                for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    match (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
                        (Some(x), Some(y))
                            if matches!(map.get(y).and_then(|row| row.get(x)), Some(b'.')) =>
                        {
                            if !visited.insert((x, y)).unwrap() {
                                let mut path = path.clone();
                                path.insert((x, y)).unwrap();

                                if x == WIDTH - 1 && y == HEIGHT - 1 {
                                    last_found_path = Some(path);
                                    found = true;
                                    break;
                                }

                                queue.push_back(Info((x, y), steps + 1, path)).unwrap();
                            }
                        }
                        _ => {}
                    }
                }
            }
        } else {
            found = true;
        }

        if !found {
            return format!("{x},{y}");
        }
    }

    unreachable!()
}

pub fn solve_1(input: &str) -> usize {
    solve_1_gen::<PUZZLE_WIDTH, PUZZLE_HEIGHT, 1024, BITSET_SIZE>(input)
}

pub fn solve_2(input: &str) -> String {
    solve_2_gen::<PUZZLE_WIDTH, PUZZLE_HEIGHT, BITSET_SIZE>(input)
}

#[cfg(feature = "input")]
pub fn part_1() -> usize {
    solve_1(&INPUT)
}

#[cfg(feature = "input")]
pub fn part_2() -> String {
    solve_2(&INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    use lazy_static::lazy_static;

    lazy_static! {
        static ref INPUT: &'static str = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#;
    }

    #[test]
    fn same_results_1() {
        assert_eq!(
            solve_1_gen::<7, 7, 12, { BitSet::with_capacity(7 * 7) }>(&INPUT),
            22
        );
    }

    #[test]
    fn same_results_2() {
        assert_eq!(
            solve_2_gen::<7, 7, { BitSet::with_capacity(7 * 7) }>(&INPUT),
            "6,1".to_string()
        );
    }
}
