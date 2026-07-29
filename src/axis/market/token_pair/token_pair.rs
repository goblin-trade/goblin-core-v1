use crate::axis::{
    leg::Pair,
    token::{token_marker::TokenMarker, token_quantity::TokenQuantity},
};

pub trait TokenPair: 'static + Clone + Copy {
    type Base: TokenMarker;
    type Quote: TokenMarker;

    const DISCRIMINATOR: u8 = (<Self::Base as TokenQuantity>::DISCRIMINATOR << 3)
        + (<Self::Quote as TokenQuantity>::DISCRIMINATOR << 4);
}

impl<B, Q> TokenPair for Pair<B, Q>
where
    B: TokenMarker,
    Q: TokenMarker,
{
    type Base = B;
    type Quote = Q;
}
