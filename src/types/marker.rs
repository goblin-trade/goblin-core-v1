use core::marker::PhantomData;

use crate::types::{GenericMap, GenericMapAccessor};

/// Marker type for any pair variant
#[derive(Clone, Copy, Default, PartialEq)]
pub struct Marker<const N: usize, K>(PhantomData<K>);

impl<T0, T1, K> GenericMapAccessor<T0, T1, K> for Marker<0, K>
where
    T0: Clone + Copy + PartialEq,
    T1: Clone + Copy + PartialEq,
{
    type Result = T0;

    fn get(map: &GenericMap<T0, T1, K>) -> Self::Result {
        map.0
    }

    fn get_leg(map: &GenericMap<T0, T1, K>) -> &Self::Result {
        &map.0
    }

    fn get_leg_mut(map: &mut GenericMap<T0, T1, K>) -> &mut Self::Result {
        &mut map.0
    }
}

impl<T0, T1, K> GenericMapAccessor<T0, T1, K> for Marker<1, K>
where
    T0: Clone + Copy + PartialEq,
    T1: Clone + Copy + PartialEq,
{
    type Result = T1;

    fn get(map: &GenericMap<T0, T1, K>) -> Self::Result {
        map.1
    }

    fn get_leg(map: &GenericMap<T0, T1, K>) -> &Self::Result {
        &map.1
    }

    fn get_leg_mut(map: &mut GenericMap<T0, T1, K>) -> &mut Self::Result {
        &mut map.1
    }
}

pub struct ETHERC20Pair;
pub struct BaseQuotePair;
pub struct HardcodedCustomPair;

// pub type ETH = Marker<0, ETHERC20Pair>;
// pub type ERC20 = Marker<1, ETHERC20Pair>;

// type Base = Marker<0, BaseQuotePair>;
// type Quote = Marker<1, BaseQuotePair>;

// type HardcodedMarker = Marker<0, BaseQuotePair>;
// type CustomMarker = Marker<1, BaseQuotePair>;
