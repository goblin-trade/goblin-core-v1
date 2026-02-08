use core::marker::PhantomData;

use crate::types::{StoreReader, TupleV2};

pub struct Marker<K, const N: usize>(PhantomData<K>);

pub struct BaseQuote;
pub type BaseV2 = Marker<BaseQuote, 0>;
pub type QuoteV2 = Marker<BaseQuote, 1>;

fn test_getter_generic<M>()
where
    M: StoreReader<TupleV2<u8, u8, BaseQuote>>,
{
    let pair: TupleV2<u8, u8, BaseQuote> = TupleV2::new(0, 1);

    let gg = M::get(&pair);
}

#[test]
fn test_getter() {
    let pair: TupleV2<u8, u8, BaseQuote> = TupleV2::new(0, 1);

    // Direct usage
    let _base_size = BaseV2::get(&pair);

    // Usage as generic param
    test_getter_generic::<BaseV2>();
}
