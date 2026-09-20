mod impl_index;

use core::marker::PhantomData;

use deku::DekuRead;
#[cfg(feature = "encode")]
use deku::DekuWrite;

use crate::{axis::market::HardcodedMarketList, axis_helpers::TokenPair};

/// Index for hardcoded market
#[derive(Clone, Copy, PartialEq, PartialOrd, DekuRead)]
#[cfg_attr(feature = "encode", derive(DekuWrite))]
pub struct MarketIndex<TP>
where
    TP: TokenPair + HardcodedMarketList,
{
    #[deku(bytes = "1", assert = "*inner < TP::HARDCODED_MARKET_LIST.len()")]
    pub inner: usize,
    #[deku(skip)]
    _marker: PhantomData<TP>,
}

impl<TP> MarketIndex<TP>
where
    TP: TokenPair + HardcodedMarketList,
{
    pub const fn new(inner: usize) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
}
