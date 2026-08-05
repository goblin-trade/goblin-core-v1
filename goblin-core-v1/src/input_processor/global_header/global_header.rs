use crate::{
    axis::{
        market::{
            market_counts::{dynamic::DynamicCounts, hardcoded::HardcodedCounts},
            MarketVariantPair,
        },
        token::{token_list::custom_erc20::CustomERC20List, token_reader::TokenDataTriple},
    },
    input_processor::{DecodeCtx, FixedDecode, HeaderFlags, VariableDecode},
    quantities::UnsidedAtoms,
    types::Address,
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
    pub market_counts: MarketVariantPair<HardcodedCounts, DynamicCounts>,

    pub token_data_triple: TokenDataTriple<'a>,
}
