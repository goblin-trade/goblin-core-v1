use core::marker::PhantomData;

use crate::{axis::token::token_marker::TokenMarker, settlement::ConstZero};

#[derive(Clone, Copy, PartialEq)]
pub struct TokenIndex<T: TokenMarker> {
    pub inner: usize,
    _marker: PhantomData<T>,
}

impl<T: TokenMarker> From<usize> for TokenIndex<T> {
    fn from(value: usize) -> Self {
        Self {
            inner: value,
            _marker: PhantomData,
        }
    }
}

impl<T: TokenMarker> ConstZero for TokenIndex<T> {
    const ZEROED: Self = Self {
        inner: 0,
        _marker: PhantomData,
    };
}
