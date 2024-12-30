#![no_std]
#![allow(clippy::must_use_candidate)]

use heapless::{binary_heap, BinaryHeap as HLBinaryHeap, Deque as HLDeque, Vec as HLVec};

type Free<T> = HLDeque<T, { 1024 * 16 }>;
type Disk<T> = HLDeque<T, { 1024 * 16 }>;
type Heap<T> = HLVec<T, { 1024 * 16 }>;
type BinaryHeap<T> = HLBinaryHeap<T, binary_heap::Min, { 1024 * 2 }>;

#[cfg(feature = "input")]
pub const INPUT: &str = include_str!("../../input");

#[cfg(not(feature = "input"))]
pub const INPUT: &str = "";

/// # Panics
#[allow(clippy::cast_possible_truncation)]
pub fn solve_1(input: &str) -> u64 {
    let mut occupied = Disk::new();
    let mut free = Free::new();
    let mut idx = 0u32;
    for (i, size) in input.trim().as_bytes().iter().enumerate() {
        let size = size - b'0';
        if size == 0 {
            continue;
        }

        if i % 2 == 0 {
            occupied.push_back((i as u16 / 2, idx, size)).unwrap();
        } else {
            free.push_back((idx, size)).unwrap();
        }

        idx += u32::from(size);
    }

    let (mut low_mark, mut high_mark) = (u32::MIN, u32::MAX);
    while low_mark < high_mark {
        let Some((id, occupied_idx, mut occupied_size)) = occupied.pop_back() else {
            break;
        };
        let Some((free_idx, mut free_size)) = free.pop_front() else {
            break;
        };

        if free_idx > occupied_idx {
            occupied
                .push_back((id, occupied_idx, occupied_size))
                .unwrap();
            break;
        }

        let moveable_size = free_size.min(occupied_size);
        assert!(moveable_size > 0);

        occupied_size -= moveable_size;
        free_size -= moveable_size;

        occupied.push_front((id, free_idx, moveable_size)).unwrap();

        if free_size > 0 {
            free.push_front((free_idx + u32::from(moveable_size), free_size))
                .unwrap();
        }
        if occupied_size > 0 {
            occupied
                .push_back((id, occupied_idx, occupied_size))
                .unwrap();
        }

        low_mark = free_idx + u32::from(moveable_size);
        high_mark = occupied_idx;
    }

    occupied
        .iter()
        .flat_map(|(id, idx, size)| (*idx..*idx + u32::from(*size)).map(move |i| u64::from(*id) * u64::from(i)))
        .sum()
}

/// # Panics
#[allow(clippy::too_many_lines, clippy::cast_possible_truncation)]
pub fn solve_2(input: &str) -> u64 {
    #[derive(Debug)]
    struct Info {
        id: u16,
        index: u32,
        size: u8,
    }

    let mut free_space_heaps = const { [ const { BinaryHeap::new() }; 9] };
    let mut occupied_heap = const { Heap::new() };

    let mut index = 0u32;
    for (i, size) in input.trim().chars().enumerate() {
        let size = (size as u32 - '0' as u32) as u8;
        if size == 0 {
            continue;
        }

        if i % 2 == 0 {
            occupied_heap
                .push(Info {
                    id: i as u16 / 2,
                    index,
                    size,
                })
                .unwrap();
        } else {
            free_space_heaps[size as usize - 1].push(index).unwrap();
        }

        index += u32::from(size);
    }

    let find_blocks = |free_space_heaps: &[BinaryHeap<u32>], occupied_heap: &[Info]| {
        let (mut target_occupied_index, mut target_free_block_index, mut target_free_block_size) =
            (u32::MAX, u32::MAX, u8::MAX);

        for (
            candidate_occupied_index,
            &Info {
                index: block_index,
                size: block_size,
                ..
            },
        ) in occupied_heap.iter().enumerate().rev()
        {
            if target_free_block_index < block_index {
                break;
            }

            if let Some((free_block_index, free_block_size)) = free_space_heaps
                .iter()
                .enumerate()
                .skip(block_size as usize - 1)
                .filter_map(|(free_block_size, heap)| {
                    heap.peek().and_then(|&v| {
                        if v < block_index {
                            Some((v, free_block_size as u8 + 1))
                        } else {
                            None
                        }
                    })
                })
                .min_by_key(|(k, _)| *k)
            {
                (
                    target_occupied_index,
                    target_free_block_index,
                    target_free_block_size,
                ) = (candidate_occupied_index as u32, free_block_index, free_block_size);
            }
        }

        if target_occupied_index != u32::MAX {
            return Some((
                target_occupied_index,
                target_free_block_index,
                target_free_block_size,
            ));
        }

        None
    };

    let mut moved_blocks = const { Heap::new() };
    while let Some((block_index, free_block_index, free_block_size)) =
        find_blocks(&free_space_heaps, &occupied_heap)
    {
        let Info {
            id,
            size: moved_size,
            ..
        } = occupied_heap.remove(block_index as usize);

        moved_blocks
            .push(Info {
                id,
                index: free_block_index,
                size: moved_size,
            })
            .unwrap();

        free_space_heaps[free_block_size as usize - 1].pop();
        if free_block_size > moved_size {
            free_space_heaps[free_block_size as usize - moved_size as usize - 1]
                .push(free_block_index + u32::from(moved_size))
                .unwrap();
        }
    }

    moved_blocks
        .iter()
        .chain(&occupied_heap)
        .flat_map(|Info { id, index, size }| (*index..*index + u32::from(*size)).map(move |i| u64::from(*id) * u64::from(i)))
        .sum()
}

#[cfg(feature = "input")]
pub fn part_1() -> u64 {
    solve_1(INPUT)
}

#[cfg(feature = "input")]
pub fn part_2() -> u64 {
    solve_2(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"2333133121414131402";

    #[test]
    fn same_results_1() {
        assert_eq!(solve_1(INPUT), 1928);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(INPUT), 2858);
    }
}
