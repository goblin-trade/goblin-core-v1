mod impl_index;

use core::marker::PhantomData;

use deku::DekuRead;
#[cfg(feature = "encode")]
use deku::DekuWrite;
use goblin_macros::fixed_codec;

use crate::{
    axis::market::HardcodedMarketList, axis_helpers::TokenPair, goblin_error::GoblinError, require,
};

/// Index for hardcoded market
#[fixed_codec(validate = Self::check)]
#[derive(Clone, Copy, PartialEq, PartialOrd, DekuRead)]
#[cfg_attr(feature = "encode", derive(DekuWrite))]
pub struct MarketIndex<TP>
where
    TP: TokenPair + HardcodedMarketList,
{
    #[codec(wire = u8)]
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

    /// Bounds check against the hardcoded list, kept out of the generated codec
    /// via `validate = Self::check`.
    fn check(&self) -> Result<(), GoblinError> {
        require!(
            self.inner < TP::HARDCODED_MARKET_LIST.len(),
            GoblinError::InvalidPayload
        );
        Ok(())
    }
}
