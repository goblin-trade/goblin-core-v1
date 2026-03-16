use core::marker::PhantomData;
use core::ops::RangeInclusive;

use crate::goblin_error::GoblinError;

pub struct GenericIterator<I, L, N>
where
    I: Iterator + Clone + Copy,
    I::Item: InnerItem<L::Item, N>,
    L: Iterator + From<RangeInclusive<L::Item>>,
    L::Item: Clone + Copy,
{
    pub inner_iterator: I,
    pub inner_item: I::Item,
    pub linear_iterator: L,
    pub limit: L::Item,
    _marker: PhantomData<N>,
}

pub trait InnerItem<P, N> {
    fn next_item(&self, pos: P) -> Option<N>;
}

impl<I, L, N> GenericIterator<I, L, N>
where
    I: Iterator + Clone + Copy,
    I::Item: InnerItem<L::Item, N>,
    L: Iterator + From<RangeInclusive<L::Item>>,
    L::Item: Clone + Copy,
{
    pub fn new(
        inner_iterator: I,
        linear_range: RangeInclusive<L::Item>,
    ) -> Result<Self, GoblinError> {
        let inner_item = inner_iterator
            .clone()
            .next()
            .ok_or(GoblinError::IteratorOutOfBounds)?;

        Ok(Self {
            inner_iterator,
            inner_item,
            limit: *linear_range.end(),
            linear_iterator: linear_range.into(),
            _marker: PhantomData,
        })
    }
}

impl<I, L, N> Iterator for GenericIterator<I, L, N>
where
    I: Iterator + Clone + Copy,
    I::Item: InnerItem<L::Item, N>,
    L: Iterator + From<RangeInclusive<L::Item>>,
    L::Item: Clone + Copy,
{
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
