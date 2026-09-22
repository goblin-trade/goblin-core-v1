pub mod market_counts;

pub use market_counts::*;

use deku::DekuRead;
#[cfg(feature = "encode")]
use deku::DekuWrite;

use crate::{input_processor::HeaderFlags, quantities::UnsidedAtoms};

/// Arguments read from calldata
///
/// The layout depends on [`HeaderFlags`], which is threaded through as Deku
/// decoding context.
#[derive(DekuRead)]
#[cfg_attr(feature = "encode", derive(DekuWrite))]
#[deku(ctx = "flags: &HeaderFlags")]
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
    #[deku(cond = "flags.withdraw_eth")]
    pub eth_out_due_u32: UnsidedAtoms<u32>,

    /// Number of hardcoded and dynamic markets to process
    #[deku(ctx = "flags.process_dynamic_markets")]
    pub market_counts: MarketCounts,
}
