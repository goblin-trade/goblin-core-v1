use core::marker::PhantomData;

pub struct GenericIterator<A, B, C, D, E>
where
    A: InnerItem<C, E>,
    B: Iterator<Item = A>,
    D: Iterator<Item = C>,
    Self: Iterator<Item = E>,
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
