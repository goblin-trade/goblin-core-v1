use core::marker::PhantomData;

pub struct TupleV2<T0, T1, K>(T0, T1, PhantomData<K>);
pub struct TripleV2<T0, T1, T2, K>(T0, T1, T2, PhantomData<K>);

pub struct Marker<K, const N: usize>(PhantomData<K>);

pub struct BaseQuote;
pub type BaseV2 = Marker<BaseQuote, 0>;
pub type QuoteV2 = Marker<BaseQuote, 1>;

pub trait TupleReaderV2<T0, T1, K> {
    type Result: Clone + Copy;

    fn get(store: &TupleV2<T0, T1, K>) -> Self::Result;
    fn get_leg(store: &TupleV2<T0, T1, K>) -> &Self::Result;
    fn get_leg_mut(store: &mut TupleV2<T0, T1, K>) -> &mut Self::Result;
}

impl<T0, T1, K> TupleReaderV2<T0, T1, K> for Marker<K, 0>
where
    T0: Clone + Copy,
    T1: Clone + Copy,
{
    type Result = T0;

    fn get(store: &TupleV2<T0, T1, K>) -> Self::Result {
        store.0
    }

    fn get_leg(store: &TupleV2<T0, T1, K>) -> &Self::Result {
        &store.0
    }

    fn get_leg_mut(store: &mut TupleV2<T0, T1, K>) -> &mut Self::Result {
        &mut store.0
    }
}

impl<T0, T1, K> TupleReaderV2<T0, T1, K> for Marker<K, 1>
where
    T0: Clone + Copy,
    T1: Clone + Copy,
{
    type Result = T1;

    fn get(store: &TupleV2<T0, T1, K>) -> Self::Result {
        store.1
    }

    fn get_leg(store: &TupleV2<T0, T1, K>) -> &Self::Result {
        &store.1
    }

    fn get_leg_mut(store: &mut TupleV2<T0, T1, K>) -> &mut Self::Result {
        &mut store.1
    }
}
