use core::marker::PhantomData;

pub struct GenericIterator<A, B, C, D, E>
where
    A: InnerItem<C, E>,
    B: Iterator<Item = A>,
    D: Iterator<Item = C>,
{
    pub inner_item: A,
    pub inner_iterator: B,
    pub limit: C,
    pub linear_iterator: D,
    _marker: PhantomData<E>,
}

pub trait InnerItem<C, E> {
    fn next_item(&self, position: C) -> E;
}

impl<A, B, C, D, E> Iterator for GenericIterator<A, B, C, D, E>
where
    A: InnerItem<C, E>,
    B: Iterator<Item = A>,
    D: Iterator<Item = C>,
{
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
