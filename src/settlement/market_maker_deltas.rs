use core::mem::MaybeUninit;

use crate::{
    quantities::{BaseLots, QuoteLots},
    types::Address,
};

pub const MAX_MARKET_MAKERS: usize = 15;

pub struct MarketMakerDeltas {
    inner: [MaybeUninit<MarketMakerDelta>; MAX_MARKET_MAKERS],
    pub len: usize,
}

impl Default for MarketMakerDeltas {
    fn default() -> Self {
        Self {
            inner: [const { MaybeUninit::uninit() }; MAX_MARKET_MAKERS],
            len: 0,
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct MarketMakerDelta {
    pub address: Address,

    pub locked_base_lots_out: BaseLots,
    pub free_base_lots_in: BaseLots,

    pub locked_quote_lots_out: QuoteLots,
    pub free_quote_lots_in: QuoteLots,
}
