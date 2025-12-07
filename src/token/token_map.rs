use core::marker::PhantomData;

use crate::token::{TokenMarker, ETH};

pub struct TokenMap<T0, T1, V>(T0, T1, PhantomData<V>);

// Looks wrong- there shouldn't be 2 versions of GlobalSenderDelta for each TokenMarker
pub type GlobalSenderDeltaV2<TokenMarker> = TokenMap<u8, u8, TokenMarker>;

pub type TokenSenderDeltasV2<ERC20Token> = TokenMap<u8, u8, ERC20Token>;
