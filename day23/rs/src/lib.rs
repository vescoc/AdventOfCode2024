#![no_std]
#![allow(clippy::must_use_candidate)]

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use bitset::BitSet;

use heapless::{FnvIndexMap, FnvIndexSet, String as HLString, Vec as HLVec};

#[cfg(feature = "input")]
pub const INPUT: &str = include_str!("../../input");

#[cfg(not(feature = "input"))]
pub const INPUT: &str = "";

const WIDTH: usize = ('z' as u16 - 'a' as u16 + 1) as usize;

type Vec<T> = HLVec<T, 16>;
type NodeSet<T> = FnvIndexSet<T, { (WIDTH * WIDTH).next_power_of_two() }>;
type Set<T> = FnvIndexSet<T, 2048>;
type NodeMap<K, V> = FnvIndexMap<K, V, { (WIDTH * WIDTH).next_power_of_two() }>;
type String = HLString<64>;

fn id(element: &str) -> usize {
    element.as_bytes().iter().fold(0, |acc, &digit| {
        acc * WIDTH + digit as usize - b'a' as usize
    })
}

/// # Panics
pub fn solve_1(input: &str) -> usize {
    fn set(elements: &[usize]) -> u128 {
        let mut elements = Vec::try_from(elements).unwrap();

        elements.sort_unstable();

        elements.iter().fold(0, |acc, &element| {
            acc * WIDTH as u128 * WIDTH as u128 + element as u128
        })
    }

    let mut edges =
        BitSet::<_, _, { BitSet::with_capacity(WIDTH * WIDTH * WIDTH * WIDTH) }>::new(|(r, c)| {
            r * WIDTH * WIDTH + c
        });
    let mut ts = BitSet::<_, _, { BitSet::with_capacity(WIDTH * WIDTH) }>::new(|i| *i);
    for (a, b) in input.lines().map(|line| line.split_once('-').unwrap()) {
        let (id_a, id_b) = (id(a), id(b));

        edges.insert((id_a, id_b)).unwrap();
        edges.insert((id_b, id_a)).unwrap();

        if a.starts_with('t') {
            ts.insert(id_a).unwrap();
        }
        if b.starts_with('t') {
            ts.insert(id_b).unwrap();
        }
    }

    let mut result = Set::new();
    let edges = &edges; // !!! borrow checker + move :'(
    (0..WIDTH * WIDTH)
        .filter(|t_id| ts.contains(t_id).unwrap())
        .flat_map(|t_id| {
            (0..WIDTH * WIDTH).filter_map(move |a_id| {
                if edges.contains(&(t_id, a_id)).unwrap() {
                    Some((t_id, a_id))
                } else {
                    None
                }
            })
        })
        .flat_map(|(t_id, a_id)| {
            (0..WIDTH * WIDTH).skip(a_id + 1).filter_map(move |b_id| {
                if edges.contains(&(t_id, b_id)).unwrap() && edges.contains(&(a_id, b_id)).unwrap()
                {
                    Some(set(&[t_id, a_id, b_id]))
                } else {
                    None
                }
            })
        })
        .for_each(|set| {
            result.insert(set).unwrap();
        });

    result.len()
}

/// # Panics
pub fn solve_2(input: &str) -> String {
    let mut nodes = NodeSet::new();
    let mut id2node = NodeMap::new();

    let mut edges =
        BitSet::<_, _, { BitSet::with_capacity(WIDTH * WIDTH * WIDTH * WIDTH) }>::new(|(r, c)| {
            r * WIDTH * WIDTH + c
        });
    for (a, b) in input.lines().map(|line| line.split_once('-').unwrap()) {
        let (id_a, id_b) = (id(a), id(b));

        edges.insert((id_a, id_b)).unwrap();
        edges.insert((id_b, id_a)).unwrap();

        nodes.insert(id_a).unwrap();
        nodes.insert(id_b).unwrap();

        id2node.insert(id_a, a).unwrap();
        id2node.insert(id_b, b).unwrap();
    }

    #[cfg(feature = "parallel")]
    let nodes = nodes.iter().par_bridge();

    #[cfg(not(feature = "parallel"))]
    let nodes = nodes.iter();

    let best_group = nodes
        .map(|&start_id| {
            let mut group = Vec::try_from([start_id].as_slice()).unwrap();
            for candidate_id in
                (0..WIDTH * WIDTH).filter(|id| edges.contains(&(start_id, *id)).unwrap())
            {
                if group
                    .iter()
                    .all(|&id| edges.contains(&(id, candidate_id)).unwrap())
                {
                    group.push(candidate_id).unwrap();
                }
            }

            group
        })
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap();

    let mut group = Vec::new();
    for node in best_group.into_iter().map(|id| id2node[&id]) {
        group.push(node).unwrap();
    }
    group.sort_unstable();

    let mut result = String::new();
    for node in group.iter().take(group.len() - 1) {
        result.push_str(node).unwrap();
        result.push(',').unwrap();
    }
    result.push_str(group.iter().last().unwrap()).unwrap();

    result
}

#[cfg(feature = "input")]
pub fn part_1() -> usize {
    solve_1(INPUT)
}

#[cfg(feature = "input")]
pub fn part_2() -> String {
    solve_2(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn same_results_1() {
        assert_eq!(solve_1(INPUT), 7);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(&solve_2(INPUT), &"co,de,ka,ta");
    }
}
