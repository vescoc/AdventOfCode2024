use core::{marker, mem};

#[derive(Copy, Clone)]
pub struct BitSet<T, K: Fn(&T) -> usize, const SIZE: usize> {
    data: [u128; SIZE],
    key: K,
    _marker: marker::PhantomData<T>,
}

impl<T, K: Fn(&T) -> usize, const SIZE: usize> BitSet<T, K, SIZE> {
    pub const fn new(key: K) -> Self {
        Self {
            data: [0; SIZE],
            key,
            _marker: marker::PhantomData,
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn insert(&mut self, idx: T) -> bool {
        let idx = (self.key)(&idx);
        let (i, b) = (idx / mem::size_of::<u128>(), idx % mem::size_of::<u128>());

        let data = &mut self.data[i];

        let mask = 1 << b;

        let result = *data & mask != 0;

        *data |= mask;

        result
    }

    pub fn contains(&self, idx: &T) -> bool {
        let idx = (self.key)(idx);
        let (i, b) = (idx / mem::size_of::<u128>(), idx % mem::size_of::<u128>());

        self.data[i] & (1 << b) != 0
    }

    pub fn remove(&mut self, idx: &T) {
        let idx = (self.key)(idx);
        let (i, b) = (idx / mem::size_of::<u128>(), idx % mem::size_of::<u128>());

        self.data[i] &= !(1 << b);
    }

    pub fn len(&self) -> usize {
        self.data
            .iter()
            .map(|value| value.count_ones() as usize)
            .sum()
    }

    pub fn is_empty(&self) -> bool {
        self.data.iter().all(|&value| value == 0)
    }
}

impl<T, K, const SIZE: usize> BitSet<T, K, SIZE>
where
    K: Fn(&T) -> usize + Copy,
{
    pub fn key(&self) -> K {
        self.key
    }
}
