use crate::{settlement::ConstDefault, types::FixedMap};

impl<
        'a,
        K: PartialEq + Clone + Copy + ConstDefault,
        V: Default + Clone + Copy + ConstDefault,
        const N: usize,
    > IntoIterator for &'a FixedMap<K, V, N>
{
    type Item = &'a (K, V);
    type IntoIter = core::slice::Iter<'a, (K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries[..self.len].iter()
    }
}

impl<
        'a,
        K: PartialEq + Clone + Copy + ConstDefault,
        V: Default + Clone + Copy + ConstDefault,
        const N: usize,
    > IntoIterator for &'a mut FixedMap<K, V, N>
{
    type Item = &'a mut (K, V);
    type IntoIter = core::slice::IterMut<'a, (K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries[..self.len].iter_mut()
    }
}
