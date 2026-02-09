use core::marker::PhantomData;

/// Wrapper struct with const N: usize, used for generating
/// sub variants of each `axis`
#[derive(Default, Clone, Copy, PartialEq)]
pub struct Marker<K, const N: usize>(PhantomData<K>);
