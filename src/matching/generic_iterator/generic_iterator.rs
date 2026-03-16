use core::marker::PhantomData;

pub struct GenericIterator<I, L, N>
where
    I: Iterator,
    I::Item: InnerItem<L::Item, N>,
    L: Iterator,
{
    pub inner_iterator: I,
    pub inner_item: I::Item,
    pub linear_iterator: L,
    pub limit: L::Item,
    _marker: PhantomData<N>,
}

pub trait InnerItem<P, N> {
    fn next_item(&self, pos: P) -> N;
}

impl<I, L, N> Iterator for GenericIterator<I, L, N>
where
    I: Iterator,
    I::Item: InnerItem<L::Item, N>,
    L: Iterator,
{
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
