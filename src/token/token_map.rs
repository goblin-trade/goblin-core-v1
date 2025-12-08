use core::marker::PhantomData;

use crate::token::{HardcodedToken, TokenMarker, ERC20, ETH};

// pub struct TokenMap<T0, T1, V>(T0, T1, PhantomData<V>);

// Looks wrong- there shouldn't be 2 versions of GlobalSenderDelta for each TokenMarker
// pub type GlobalSenderDeltaV2<TokenMarker> = TokenMap<u8, u8, TokenMarker>;
// pub type TokenSenderDeltasV2<ERC20Token> = TokenMap<u8, u8, ERC20Token>;

pub struct TokenMap<T0, T1>(pub T0, pub T1);
pub struct ERC20Map<T0, T1>(pub T0, pub T1);

pub type GlobalSenderDeltaV2 = TokenMap<u8, u8>;

pub trait TokenMapGetter<T0, T1, M> {
    type MapResult;

    // Problem- we need a composite type if we want a common function
    // .get_ref::<ETH>()
    fn get_ref(&self) -> &Self::MapResult;
}

// Alt design- don't create new trait
// Just implement for ETH and ERC20

// 1. TokenMarker- ETH and ERC20
impl<T0, T1> TokenMapGetter<T0, T1, ETH> for TokenMap<T0, T1> {
    type MapResult = T0;

    fn get_ref(&self) -> &Self::MapResult {
        &self.0
    }
}

impl<T0, T1> TokenMapGetter<T0, T1, ERC20> for TokenMap<T0, T1> {
    type MapResult = T1;

    fn get_ref(&self) -> &Self::MapResult {
        &self.1
    }
}

// 2. ERC20Marker- HardcodedToken and CustomToken
impl<T0, T1> TokenMapGetter<T0, T1, HardcodedToken> for ERC20Map<T0, T1> {
    type MapResult = T0;

    fn get_ref(&self) -> &Self::MapResult {
        &self.0
    }
}

impl<T0, T1> TokenMapGetter<T0, T1, ERC20> for ERC20Map<T0, T1> {
    type MapResult = T1;

    fn get_ref(&self) -> &Self::MapResult {
        &self.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access() {
        let global_delta = TokenMap::<u8, u16>(0, 1);

        let eth_value = <TokenMap<u8, u16> as TokenMapGetter<u8, u16, ETH>>::get_ref(&global_delta);
        let erc20_value =
            <TokenMap<u8, u16> as TokenMapGetter<u8, u16, ERC20>>::get_ref(&global_delta);

        // let eth_value = global_delta.get_ref::<u8, u8, ETH>();

        // TODO use get_ref() for ETH
    }
}
