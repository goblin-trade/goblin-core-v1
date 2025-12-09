use core::marker::PhantomData;

use crate::token::{CustomToken, HardcodedToken, TokenMarker, ERC20, ETH};

pub struct GenericMap<T0, T1, M0, M1>(T0, T1, PhantomData<(M0, M1)>);

impl<T0, T1, M0, M1> GenericMap<T0, T1, M0, M1> {
    pub fn new(t0: T0, t1: T1) -> Self {
        Self(t0, t1, PhantomData)
    }
}

pub trait GenericMapAccessor<T0, T1, M0, M1> {
    type Result;

    fn get_leg(map: &GenericMap<T0, T1, M0, M1>) -> &Self::Result;
}

// ETH and ERC20 implement trait TokenMarker
type TokenMap<T0, T1> = GenericMap<T0, T1, ETH, ERC20>;

// We have more such pairs like (HardcodedToken and CustomToken)
// Can we have an abstraction to avoid implementing accessor trait for each pair?

// // Getters for (ETH, ERC20)
// impl<T0, T1> GenericMapAccessor<T0, T1, ETH, ERC20> for ETH {
//     type Result = T0;

//     fn get_leg(map: &TokenMap<T0, T1>) -> &Self::Result {
//         &map.0
//     }
// }

// impl<T0, T1> GenericMapAccessor<T0, T1, ETH, ERC20> for ERC20 {
//     type Result = T1;

//     fn get_leg(map: &TokenMap<T0, T1>) -> &Self::Result {
//         &map.1
//     }
// }

pub struct Marker0;
pub struct Marker1;

pub trait MapMarker {
    type Marker;
}

impl MapMarker for ETH {
    type Marker = Marker0;
}

impl MapMarker for ERC20 {
    type Marker = Marker1;
}

impl<T0, T1, M0, M1> GenericMapAccessor<T0, T1, M0, M1> for Marker0 {
    type Result = T0;

    fn get_leg(map: &GenericMap<T0, T1, M0, M1>) -> &Self::Result {
        &map.0
    }
}

impl<T0, T1, M0, M1> GenericMapAccessor<T0, T1, M0, M1> for Marker1 {
    type Result = T1;

    fn get_leg(map: &GenericMap<T0, T1, M0, M1>) -> &Self::Result {
        &map.1
    }
}
// problem- trait overlap
// Even if we set trait bounds M0: First and M1: Second, it is possible for
// a marker to implement both traits
// This problem doesn't arise when we use concrete marker structs.

// Think from the top
// In PairShape::update<ETH, ERC20>() we first explicitly set generic to ETH
// The top level value is known, rest of the inner functions become generic.
//
// For a struct of keys <ETH, ERC20>, we could have separate traits to operate on first
// and second elements because the element is always known at the top level.

#[cfg(test)]
mod tests {
    use super::*;

    fn get_for_leg<T0, T1, M>(
        token_map: &TokenMap<T0, T1>,
    ) -> &<M as GenericMapAccessor<T0, T1, ETH, ERC20>>::Result
    where
        M: GenericMapAccessor<T0, T1, ETH, ERC20>,
    {
        M::get_leg(token_map)
    }

    #[test]
    fn test_read_from_token_map() {
        let token_map = TokenMap::<u8, u16>::new(0, 1);

        let gg = <ETH as MapMarker>::Marker::get_leg(&token_map);
        // let eth_amount = ETH::get_leg(&token_map);

        // let eth_amount_v2 = get_for_leg::<u8, u16, ETH>(&token_map);

        // let erc_amount_v2 = get_for_leg::<u8, u16, ERC20>(&token_map);
    }
}
