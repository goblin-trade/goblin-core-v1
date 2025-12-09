use core::marker::PhantomData;

use crate::token::{ERC20, ETH};

pub struct GenericMap<T0, T1, K>(T0, T1, PhantomData<K>);

impl<T0, T1, K> GenericMap<T0, T1, K> {
    pub fn new(t0: T0, t1: T1) -> Self {
        Self(t0, t1, PhantomData)
    }
}

pub trait GenericMapAccessor<T0, T1, K> {
    type Result;

    fn get_leg(map: &GenericMap<T0, T1, K>) -> &Self::Result;
}

// ETH and ERC20 implement trait TokenMarker
type TokenMap<T0, T1> = GenericMap<T0, T1, (ETH, ERC20)>;

pub struct Marker<const N: usize, K>(PhantomData<K>);

impl<T0, T1, K> GenericMapAccessor<T0, T1, K> for Marker<0, K> {
    type Result = T0;

    fn get_leg(map: &GenericMap<T0, T1, K>) -> &Self::Result {
        &map.0
    }
}

impl<T0, T1, K> GenericMapAccessor<T0, T1, K> for Marker<1, K> {
    type Result = T1;

    fn get_leg(map: &GenericMap<T0, T1, K>) -> &Self::Result {
        &map.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn process_leg<T0, T1, M>(map: &TokenMap<T0, T1>) -> &M::Result
    where
        M: GenericMapAccessor<T0, T1, (ETH, ERC20)>,
    {
        M::get_leg(map)
    }

    fn process_leg_typed<M>(map: &TokenMap<u8, u16>) -> &M::Result
    where
        M: GenericMapAccessor<u8, u16, (ETH, ERC20)>,
    {
        M::get_leg(map)
    }

    #[test]
    fn test_read_from_token_map() {
        let token_map = TokenMap::<u8, u16>::new(0, 1);

        let eth_amount = Marker::<0, (ETH, ERC20)>::get_leg(&token_map);

        let eth_amount_v2 = process_leg::<u8, u16, Marker<0, (ETH, ERC20)>>(&token_map);

        let eth_amount_v3 = process_leg_typed::<Marker<0, (ETH, ERC20)>>(&token_map);

        // Alt design where (ETH, ERC20) impl MarkerPair
        // (ETH, ERC20)::Markers.0::get_leg()
    }
}
