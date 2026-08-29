use crate::axis::{
    leg::Pair,
    market::market_locator::hardcoded::HardcodedMarketList,
    token::{token_marker::TokenMarker, token_quantity::TokenQuantity},
};

// TODO better to have HardcodedMarketList bound here
pub trait TokenPair: 'static + Clone + Copy + PartialEq + PartialOrd {
    type Base: TokenMarker;
    type Quote: TokenMarker;

    const DISCRIMINATOR: u8 = (<Self::Base as TokenQuantity>::DISCRIMINATOR << 3)
        + (<Self::Quote as TokenQuantity>::DISCRIMINATOR << 4);
}

impl<B, Q> TokenPair for Pair<B, Q>
where
    B: TokenMarker,
    Q: TokenMarker,
    Self: HardcodedMarketList,
{
    type Base = B;
    type Quote = Q;
}
