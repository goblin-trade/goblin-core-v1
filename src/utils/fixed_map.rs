/// A fixed-capacity map with O(N) lookups, no removal, no alloc.
/// - Uses `MaybeUninit` so keys/values are never zero-initialized.
/// - Only supports insert/update (no delete).
pub struct FixedMap<K, V, const N: usize> {
    pub entries: [(K, V); N],
    pub len: usize,
    // _marker: PhantomData<(K, V)>,
}

// impl<K, V, const N: usize> Default for FixedMap<K, V, N> {
//     fn default() -> Self {
//         Self {
//             entries: [const { MaybeUninit::uninit() }; N],
//             len: 0,
//             _marker: PhantomData,
//         }
//     }
// }

impl<K, V, const N: usize> FixedMap<K, V, N> {
    /// Inserts a key-value pair into the map, overwriting the existing value if the key is already present.
    ///
    /// # Returns
    ///
    /// Some(()) if insertion is successful, None if array is full
    pub fn insert(&mut self, key: K, new_value: V) -> Option<()>
    where
        K: PartialEq,
    {
        // Check if the key already exists.
        for i in 0..self.len {
            let (k, v) = &mut self.entries[i];
            if *k == key {
                // Key found, just update the value and return.
                *v = new_value;
                return Some(());
            }
        }

        // If the loop finishes, the key is new. Check for capacity.
        if self.len >= N {
            return None;
        }

        // There is space. Write the new entry.
        // SAFETY: `self.len < N`, so this index is in bounds
        self.entries[self.len] = (key, new_value);

        self.len += 1;

        Some(())
    }

    pub fn get_or_insert_mut(&mut self, key: K) -> Option<&mut V>
    where
        K: PartialEq + Clone + Copy,
        V: Default,
    {
        for i in 0..self.len {
            if self.entries[i].0 == key {
                let (_, value_mut) = &mut self.entries[i];
                return Some(value_mut);
            }
        }

        if self.len >= N {
            return None;
        }

        // SAFETY: self.len < N, slot is uninitialized
        let entry = &mut self.entries[self.len];
        self.len += 1;
        *entry = (key, V::default());

        let (_, value_mut) = entry;
        Some(value_mut)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn iter(&self) -> impl Iterator<Item = &(K, V)> {
        self.entries[..self.len].iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut (K, V)> {
        self.entries[..self.len].iter_mut()
    }
}
