use core::marker::PhantomData;

pub struct Marker<K, const N: usize>(PhantomData<K>);

pub struct BaseQuote;
pub type BaseV2 = Marker<BaseQuote, 0>;
pub type QuoteV2 = Marker<BaseQuote, 1>;
