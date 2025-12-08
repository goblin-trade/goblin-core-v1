use crate::token::ERC20Marker;
use core::marker::PhantomData;

/// Index of a token used by the matching engine
///
/// We primarily deal with token indices to save memory.
/// Token address is lazily derived at the time of hostio operations.
///
/// # Namespacing
///
/// * Token indices are namespaced as HardcodedIndex and CustomIndex\
///
/// * Additionally, we define DynamicIndex as an enum of HardcodedIndex and CustomIndex.
/// This is a runtime enum used in custom markets. This way custom markets can use
/// both hardcoded and custom token indices.
///
#[derive(Clone, Copy, PartialEq)]
pub struct TokenIndex<T: ERC20Marker> {
    pub inner: u8,
    _marker: PhantomData<T>,
}

impl<T: ERC20Marker> TokenIndex<T> {
    pub const fn new(inner: u8) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
}
