use crate::{
    axis::token::token_reader::TokenDataTriple, input_processor::MarketCountsV2,
    quantities::UnsidedAtoms, types::Address,
};

/// Arguments read from calldata
pub struct GlobalHeader<'a> {
    /// Amount of ETH atoms pending withdrawal, as read from global namespace header
    ///
    /// The actual amount withdrawn is MIN(available, widthdrawal_due)
    /// This allows us to withdraw max available amount by passing u64::MAX
    ///
    /// The amount is transferred out internally (store credit) or externally (transfer call).
    pub eth_out_due: UnsidedAtoms,

    /// Optional custom recipient
    pub custom_recipient: Option<&'a Address>,

    /// Number of hardcoded and dynamic markets to process
    pub market_counts: MarketCountsV2,
    pub token_data_triple: TokenDataTriple<'a>,
}
