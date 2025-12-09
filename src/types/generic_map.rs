use core::marker::PhantomData;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GenericMap<T0: Clone + Copy, T1: Clone + Copy, K>(T0, T1, PhantomData<K>);

impl<T0: Clone + Copy, T1: Clone + Copy, K> GenericMap<T0, T1, K> {
    pub fn new(t0: T0, t1: T1) -> Self {
        Self(t0, t1, PhantomData)
    }
}

pub trait GenericMapAccessor<T0: Clone + Copy, T1: Clone + Copy, K> {
    type Result;

    fn get(map: &GenericMap<T0, T1, K>) -> Self::Result;

    fn get_leg(map: &GenericMap<T0, T1, K>) -> &Self::Result;

    fn get_leg_mut(map: &mut GenericMap<T0, T1, K>) -> &mut Self::Result;
}

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Marker<const N: usize, K>(PhantomData<K>);

impl<T0: Clone + Copy, T1: Clone + Copy, K> GenericMapAccessor<T0, T1, K> for Marker<0, K> {
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

impl<T0: Clone + Copy, T1: Clone + Copy, K> GenericMapAccessor<T0, T1, K> for Marker<1, K> {
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

type TokenMap<T0, T1> = GenericMap<T0, T1, ETHERC20Pair>;

type ETH = Marker<0, ETHERC20Pair>;
type ERC20 = Marker<1, ETHERC20Pair>;

type Base = Marker<0, BaseQuotePair>;
type Quote = Marker<1, BaseQuotePair>;

type HardcodedMarker = Marker<0, BaseQuotePair>;
type CustomMarker = Marker<1, BaseQuotePair>;

#[cfg(test)]
mod tests {
    use super::*;

    fn process_leg<T0, T1, M>(map: &TokenMap<T0, T1>) -> &M::Result
    where
        T0: Clone + Copy,
        T1: Clone + Copy,
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

        let eth_amount_v4 = ETH::get_leg(&token_map);
    }
}
