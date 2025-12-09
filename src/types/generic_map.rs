use crate::types::ETHERC20Pair;
use core::marker::PhantomData;

/// A generic tuple whose fields can be accessed with generics
///
/// Used as a storage type for pairs of (ETH, ERC20), (Base, Quote)
/// and (HardcodedToken, CustomToken)
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq)]
pub struct GenericMap<T0, T1, K>(pub T0, pub T1, PhantomData<K>)
where
    T0: Clone + Copy + PartialEq,
    T1: Clone + Copy + PartialEq;

impl<T0, T1, K> GenericMap<T0, T1, K>
where
    T0: Clone + Copy + PartialEq,
    T1: Clone + Copy + PartialEq,
{
    pub fn new(t0: T0, t1: T1) -> Self {
        Self(t0, t1, PhantomData)
    }
}

/// Accessor trait to read from GenericMap using generic Marker type
pub trait GenericMapAccessor<T0, T1, K>
where
    T0: Clone + Copy + PartialEq,
    T1: Clone + Copy + PartialEq,
{
    type Result;

    fn get(map: &GenericMap<T0, T1, K>) -> Self::Result;

    fn get_leg(map: &GenericMap<T0, T1, K>) -> &Self::Result;

    fn get_leg_mut(map: &mut GenericMap<T0, T1, K>) -> &mut Self::Result;
}

type TokenMap<T0, T1> = GenericMap<T0, T1, ETHERC20Pair>;

#[cfg(test)]
mod tests {
    use crate::types::Marker;

    use super::*;

    fn process_leg<T0, T1, M>(map: &TokenMap<T0, T1>) -> &M::Result
    where
        T0: Clone + Copy + PartialEq,
        T1: Clone + Copy + PartialEq,
        M: GenericMapAccessor<T0, T1, ETHERC20Pair>,
    {
        M::get_leg(map)
    }

    fn process_leg_typed<M>(map: &TokenMap<u8, u16>) -> &M::Result
    where
        M: GenericMapAccessor<u8, u16, ETHERC20Pair>,
    {
        M::get_leg(map)
    }

    #[test]
    fn test_read_from_token_map() {
        let token_map = TokenMap::<u8, u16>::new(0, 1);

        let eth_amount = Marker::<0, ETHERC20Pair>::get_leg(&token_map);

        let eth_amount_v2 = process_leg::<u8, u16, Marker<0, ETHERC20Pair>>(&token_map);

        let eth_amount_v3 = process_leg_typed::<Marker<0, ETHERC20Pair>>(&token_map);

        // let eth_amount_v4 = ETH::get_leg(&token_map);
    }
}
