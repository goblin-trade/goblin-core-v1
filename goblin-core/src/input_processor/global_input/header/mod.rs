pub mod market_counts;

pub use market_counts::*;

use crate::codec::{CodecResult, GoblinRead, Reader};
#[cfg(feature = "encode")]
use crate::codec::{GoblinWrite, Writer};

use crate::{input_processor::HeaderFlags, quantities::UnsidedAtoms};

/// Arguments read from calldata
///
/// The layout depends on [`HeaderFlags`], which is passed as decoding context.
pub struct Header {
    /// Amount of ETH atoms pending withdrawal, as read from global namespace header
    ///
    /// # Decoding
    ///
    /// Decoded as u32, then cast to u64.
    ///
    /// The actual amount withdrawn is MIN(available, widthdrawal_due)
    /// This allows us to withdraw max available amount by passing u32::MAX
    ///
    /// The amount is transferred out internally (store credit) or externally (transfer call).
    ///
    /// Only present when [`HeaderFlags::withdraw_eth`] is set; otherwise defaults to zero.
    pub eth_out_due_u32: UnsidedAtoms<u32>,

    /// Number of hardcoded and dynamic markets to process
    pub market_counts: MarketCounts,
}

impl<'de> GoblinRead<'de, HeaderFlags> for Header {
    fn from_reader_with_ctx(reader: &mut Reader<'de>, flags: HeaderFlags) -> CodecResult<Self> {
        let eth_out_due_u32 = if flags.withdraw_eth {
            UnsidedAtoms::<u32>::from_reader_with_ctx(reader, ())?
        } else {
            UnsidedAtoms::<u32>::default()
        };

        let market_counts =
            MarketCounts::from_reader_with_ctx(reader, flags.process_dynamic_markets)?;

        Ok(Self {
            eth_out_due_u32,
            market_counts,
        })
    }
}

#[cfg(feature = "encode")]
impl GoblinWrite<HeaderFlags> for Header {
    fn to_writer(&self, writer: &mut Writer<'_>, flags: HeaderFlags) -> CodecResult<()> {
        if flags.withdraw_eth {
            self.eth_out_due_u32.to_writer(writer, ())?;
        }

        self.market_counts
            .to_writer(writer, flags.process_dynamic_markets)?;

        Ok(())
    }
}
