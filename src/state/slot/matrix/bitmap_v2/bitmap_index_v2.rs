use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    quantities::{InnerPosV2, OuterPosV2},
    state::{
        bitmap::{
            inner_bitmap::preimage::InnerBitmapPreimage,
            outer_bitmap::preimage::OuterBitmapPreimage,
        },
        bitmap_v2::ordered_index::{OrderedIndex, OuterIndex},
    },
};

pub trait BitmapIndexV2: OrderedIndex {
    type Preimage<M, B, Q>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker;

    fn inner(&self) -> u8;
    fn byte_index(&self) -> usize {
        self.inner() as usize / 8
    }
    fn bit_index(&self) -> usize {
        self.inner() as usize % 8
    }
}

impl BitmapIndexV2 for OuterPosV2 {
    type Preimage<M, B, Q>
        = OuterBitmapPreimage<M, B, Q>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker;

    fn inner(&self) -> u8 {
        self.inner
    }
}

impl BitmapIndexV2 for InnerPosV2 {
    type Preimage<M, B, Q>
        = InnerBitmapPreimage<M, B, Q>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker;

    fn inner(&self) -> u8 {
        self.inner
    }
}
