#![allow(clippy::must_use_candidate)]

use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[cfg(feature = "input")]
pub const INPUT: &str = include_str!("../../input");

#[cfg(not(feature = "input"))]
pub const INPUT: &str = "";

/// # Panics
pub fn solve_1(input: &str) -> u64 {
    #[derive(Copy, Clone)]
    enum Block {
        Free,
        Occupied(usize),
    }

    let mut disk = input
        .trim()
        .chars()
        .enumerate()
        .flat_map(|(i, size)| {
            core::iter::repeat(if i % 2 == 0 {
                Block::Occupied(i / 2)
            } else {
                Block::Free
            })
            .take((size as u32 - '0' as u32) as usize)
        })
        .collect::<Vec<_>>();

    let (mut current, mut last) = (
        disk.iter()
            .position(|block| matches!(block, Block::Free))
            .unwrap(),
        disk.iter()
            .rposition(|block| matches!(block, Block::Occupied(_)))
            .unwrap(),
    );
    while current < last {
        disk[current] = disk[last];
        disk[last] = Block::Free;

        while matches!(disk[current], Block::Occupied(_)) {
            current += 1;
        }

        while matches!(disk[last], Block::Free) {
            last -= 1;
        }
    }

    disk.iter()
        .enumerate()
        .filter_map(|(i, block)| match block {
            Block::Occupied(id) => Some(i as u64 * *id as u64),
            Block::Free => None,
        })
        .sum()
}

/// # Panics
#[allow(clippy::too_many_lines)]
pub fn solve_2(input: &str) -> u64 {
    #[derive(Debug)]
    struct Info {
        id: usize,
        idx: usize,
        size: usize,
    }

    let mut free_space_heaps = [const { BinaryHeap::new() }; 9];
    let mut occupied_heap = Vec::new();

    let mut idx = 0;
    for (i, size) in input.trim().chars().enumerate() {
        let size = (size as u32 - '0' as u32) as usize;
        if size == 0 {
            continue;
        }

        if i % 2 == 0 {
            occupied_heap.push(Info {
                id: i / 2,
                idx,
                size,
            });
        } else {
            free_space_heaps[size - 1].push(Reverse(idx));
        }

        idx += size;
    }

    let find_blocks = |free_space_heaps: &[BinaryHeap<Reverse<usize>>], occupied_heap: &[Info]| {
        let (mut target_occupied_idx, mut target_free_block_idx, mut target_free_block_size) =
            (usize::MAX, usize::MAX, usize::MAX);

        for (
            candidate_occupied_idx,
            &Info {
                idx: block_idx,
                size: block_size,
                ..
            },
        ) in occupied_heap.iter().enumerate().rev()
        {
            if target_free_block_idx < block_idx {
                break;
            }

            if let Some((free_block_idx, free_block_size)) = free_space_heaps
                .iter()
                .enumerate()
                .skip(block_size - 1)
                .filter_map(|(free_block_size, heap)| {
                    heap.peek().and_then(|&Reverse(v)| {
                        if v < block_idx {
                            Some((v, free_block_size + 1))
                        } else {
                            None
                        }
                    })
                })
                .min_by_key(|(k, _)| *k)
            {
                (
                    target_occupied_idx,
                    target_free_block_idx,
                    target_free_block_size,
                ) = (candidate_occupied_idx, free_block_idx, free_block_size);
            }
        }

        if target_occupied_idx != usize::MAX {
            return Some((
                target_occupied_idx,
                target_free_block_idx,
                target_free_block_size,
            ));
        }

        None
    };

    let mut moved_blocks = Vec::new();
    while let Some((block_idx, free_block_idx, free_block_size)) =
        find_blocks(&free_space_heaps, &occupied_heap)
    {
        let Info {
            id,
            size: moved_size,
            ..
        } = occupied_heap.remove(block_idx);

        moved_blocks.push(Info {
            id,
            idx: free_block_idx,
            size: moved_size,
        });

        free_space_heaps[free_block_size - 1].pop();
        if free_block_size > moved_size {
            free_space_heaps[free_block_size - moved_size - 1]
                .push(Reverse(free_block_idx + moved_size));
        }
    }

    moved_blocks
        .into_iter()
        .chain(occupied_heap)
        .flat_map(|Info { id, idx, size }| (idx..idx + size).map(move |i| id as u64 * i as u64))
        .sum()
}

#[cfg(feature = "input")]
pub fn part_1() -> u64 {
    solve_1(&INPUT)
}

#[cfg(feature = "input")]
pub fn part_2() -> u64 {
    solve_2(&INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"2333133121414131402"#;

    #[test]
    fn same_results_1() {
        assert_eq!(solve_1(&INPUT), 1928);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&INPUT), 2858);
    }
}
