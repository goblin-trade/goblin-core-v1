use crate::{
    axis::{
        market::{
            market_counts::{dynamic::DynamicCounts, hardcoded::HardcodedCounts, MarketCounts},
            Dynamic, Hardcoded, MarketVariantPair,
        },
        token::{token_list::custom_erc20::CustomERC20List, token_reader::TokenDataTriple},
    },
    goblin_error::GoblinError,
    input_processor::{DecodeCtx, FixedDecode, HeaderFlags, VariableDecode},
    quantities::UnsidedAtoms,
    settlement::Delta,
    types::{Address, StoreReader},
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

// impl<'a> GlobalHeader<'a> {
//     pub fn process(
//         &'a self,
//         hostio_fields: &HostioFields,
//         ctx: &DecodeCtx,
//         delta: &mut Delta,
//     ) -> Result<(), GoblinError> {
//         let hardcoded_counts = Hardcoded::get_leg(&self.market_counts);
//         hardcoded_counts.process(
//             &hostio_fields.msg_sender,
//             ctx,
//             &self.token_data_triple,
//             delta,
//         )?;

//         let dynamic_counts = Dynamic::get_leg(&self.market_counts);
//         dynamic_counts.process(
//             &hostio_fields.msg_sender,
//             ctx,
//             &self.token_data_triple,
//             delta,
//         )?;

//         delta.global.settle(
//             self.recipient(msg_sender),
//             &self.token_data_triple,
//             &self.msg_transfers,
//         )
//     }
// }
