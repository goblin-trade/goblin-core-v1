pub mod market_counts;

pub use market_counts::*;

mod impl_variable_decode;

use crate::quantities::UnsidedAtoms;

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
