pub mod market_counts;

pub use market_counts::*;

use deku::{DekuError, DekuReader};

use crate::{
    input_processor::{ArgsReaderV2, HeaderFlags},
    quantities::UnsidedAtoms,
};

/// Arguments read from calldata
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
    pub eth_out_due_u32: UnsidedAtoms<u32>,

    /// Number of hardcoded and dynamic markets to process
    pub market_counts: MarketCounts,
}

impl Header {
    /// Decode the header, whose layout depends on `flags`.
    pub fn decode(reader: &mut ArgsReaderV2<'_>, flags: &HeaderFlags) -> Result<Self, DekuError> {
        let eth_out_due_u32 = if flags.withdraw_eth {
            UnsidedAtoms::<u32>::from_reader_with_ctx(reader, ())?
        } else {
            UnsidedAtoms::default()
        };

        let market_counts = MarketCounts::decode(reader, flags.process_dynamic_markets)?;

        Ok(Self {
            eth_out_due_u32,
            market_counts,
        })
    }
}
