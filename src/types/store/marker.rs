use core::marker::PhantomData;

use crate::types::{Leg, StoreReader, TupleV2};

/// Wrapper struct with const N: usize, used for generating
/// sub variants of each `axis`
pub struct Marker<K, const N: usize>(PhantomData<K>);

/// The side or leg of a trade

pub type BaseV2 = Marker<Leg, 0>;
pub type QuoteV2 = Marker<Leg, 1>;

pub struct MarketVariantMarker;
pub type HardcodedV2 = Marker<MarketVariantMarker, 0>;
pub type DynamicV2 = Marker<MarketVariantMarker, 1>;

fn test_getter_generic<M>()
where
    M: StoreReader<TupleV2<u8, u8, Leg>>,
{
    let pair: TupleV2<u8, u8, Leg> = TupleV2::new(0, 1);
    M::get(&pair);
}

#[test]
fn test_getter() {
    let pair: TupleV2<u8, u8, Leg> = TupleV2::new(0, 1);

    // Direct usage
    let _base_size = BaseV2::get(&pair);

    // Usage as generic param
    test_getter_generic::<BaseV2>();
}
