pub mod market_counts;

pub use market_counts::*;

mod impl_variable_decode;

use crate::{axis::token::TokenDataTriple, quantities::UnsidedAtoms, types::Address};

/// Arguments read from calldata
pub struct GlobalHeader<'a> {
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
    pub eth_out_due: UnsidedAtoms<u64>,

    /// Optional custom recipient
    pub custom_recipient: Option<&'a Address>,

    /// Number of hardcoded and dynamic markets to process
    pub market_counts: MarketCounts,
    pub token_data_triple: TokenDataTriple<'a>,
}
