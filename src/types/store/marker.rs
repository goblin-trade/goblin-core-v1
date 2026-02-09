use core::marker::PhantomData;

use crate::types::{Leg, Market, StoreReader, Token, Tuple};

/// Wrapper struct with const N: usize, used for generating
/// sub variants of each `axis`
#[derive(Default, Clone, Copy, PartialEq)]
pub struct Marker<K, const N: usize>(PhantomData<K>);

// The side or leg of a trade
pub type Base = Marker<Leg, 0>;
pub type Quote = Marker<Leg, 1>;

// Market variants
pub type Hardcoded = Marker<Market, 0>;
pub type Dynamic = Marker<Market, 1>;

// Token variants
pub type ETH = Marker<Token, 0>;
pub type HardcodedERC20 = Marker<Token, 1>;
pub type CustomERC20 = Marker<Token, 2>;

fn test_getter_generic<M>()
where
    M: StoreReader<Tuple<u8, u8, Leg>>,
{
    let pair: Tuple<u8, u8, Leg> = Tuple::new(0, 1);
    M::get(&pair);
}

#[test]
fn test_getter() {
    let pair: Tuple<u8, u8, Leg> = Tuple::new(0, 1);

    // Direct usage
    let _base_size = Base::get(&pair);

    // Usage as generic param
    test_getter_generic::<Base>();
}
