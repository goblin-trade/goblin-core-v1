use core::marker::PhantomData;

use crate::quantities::{OuterBitmapIndex, SafePosition};

pub struct Pos0;

impl SafePosition<Pos0> {
    pub fn new(outer_bitmap_index: OuterBitmapIndex) -> Self {
        Self {
            inner: outer_bitmap_index.into(),
            _marker: PhantomData,
        }
    }
}
