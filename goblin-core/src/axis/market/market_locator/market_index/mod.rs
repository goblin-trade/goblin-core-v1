mod impl_index;

use core::marker::PhantomData;

use crate::codec::{CodecError, CodecResult, GoblinRead, Reader};
#[cfg(feature = "encode")]
use crate::codec::{GoblinWrite, Writer};

use crate::{axis::market::HardcodedMarketList, axis_helpers::TokenPair};

/// Index for hardcoded market
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct MarketIndex<TP>
where
    TP: TokenPair + HardcodedMarketList,
{
    pub inner: usize,
    _marker: PhantomData<TP>,
}

impl<'de, TP> GoblinRead<'de, ()> for MarketIndex<TP>
where
    TP: TokenPair + HardcodedMarketList,
{
    fn from_reader_with_ctx(reader: &mut Reader<'de>, (): ()) -> CodecResult<Self> {
        let inner = reader.read_u8()? as usize;
        if inner < TP::HARDCODED_MARKET_LIST.len() {
            Ok(Self {
                inner,
                _marker: PhantomData,
            })
        } else {
            Err(CodecError::InvalidValue)
        }
    }
}

#[cfg(feature = "encode")]
impl<TP> GoblinWrite<()> for MarketIndex<TP>
where
    TP: TokenPair + HardcodedMarketList,
{
    fn to_writer(&self, writer: &mut Writer<'_>, (): ()) -> CodecResult<()> {
        writer.write_u8(self.inner as u8)
    }
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
