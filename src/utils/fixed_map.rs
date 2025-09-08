use core::{marker::PhantomData, mem::MaybeUninit};

/// A fixed-capacity map with O(N) lookups, no removal, no alloc.
/// - Uses `MaybeUninit` so keys/values are never zero-initialized.
/// - Only supports insert/update (no delete).
pub struct FixedMap<K, V, const N: usize> {
    entries: [MaybeUninit<(K, V)>; N],
    len: usize,
    _marker: PhantomData<(K, V)>,
}

impl<K, V, const N: usize> Default for FixedMap<K, V, N> {
    fn default() -> Self {
        Self {
            entries: [const { MaybeUninit::uninit() }; N],
            len: 0,
            _marker: PhantomData,
        }
    }
}

impl<K, V, const N: usize> FixedMap<K, V, N> {
    pub fn new_unchecked(entries: [MaybeUninit<(K, V)>; N], len: usize) -> Self {
        Self {
            entries,
            len,
            _marker: PhantomData,
        }
    }

    pub fn get_or_insert_mut(&mut self, key: K) -> Option<&mut V>
    where
        K: PartialEq + Clone + Copy,
        V: Default,
    {
        for i in 0..self.len {
            // SAFETY: slots [0..len) are initialized
            let (k, v) = unsafe { &mut *self.entries[i].as_mut_ptr() };
            if *k == key {
                return Some(v);
            }
        }

        if self.len >= N {
            return None;
        }

        // SAFETY: self.len < N, slot is uninitialized
        let entry = &mut self.entries[self.len];
        self.len += 1;
        entry.write((key, V::default()));

        let (_, v) = unsafe { &mut *entry.as_mut_ptr() };
        Some(v)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn iter(&self) -> impl Iterator<Item = &(K, V)> {
        self.entries[..self.len]
            .iter()
            .map(|maybe_uninit| unsafe { maybe_uninit.assume_init_ref() })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut (K, V)> {
        self.entries[..self.len]
            .iter_mut()
            .map(|maybe_uninit| unsafe { maybe_uninit.assume_init_mut() })
    }
}
