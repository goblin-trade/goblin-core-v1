use core::marker::PhantomData;

use crate::{market::MarketMarker, token::TokenMarker, types::Pair};

pub struct MarketHeader<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    /// Whether to read decode deposit amounts
    pub decode_deposit_amounts: bool,

    /// Whether to execute base-in and quote-in take orders
    pub execute_takes: Pair<bool, bool>,

    /// Number of outer bitmap indices
    pub outer_bitmap_indices: u8,

    _marker: PhantomData<(M, B, Q)>,
}

impl<M, B, Q> MarketHeader<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    pub fn new(
        decode_deposit_amounts: bool,
        execute_takes: Pair<bool, bool>,
        outer_bitmap_indices: u8,
    ) -> Self {
        Self {
            decode_deposit_amounts,
            execute_takes,
            outer_bitmap_indices,
            _marker: PhantomData,
        }
    }
}
