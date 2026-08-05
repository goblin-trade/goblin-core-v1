use crate::{
    axis::{
        market::{
            market_counts::{dynamic::DynamicCounts, hardcoded::HardcodedCounts, MarketCounts},
            Dynamic, Hardcoded, MarketVariantPair,
        },
        token::{
            token_list::custom_erc20::{
                custom_erc20_count::custom_erc20_count, custom_erc20_list::custom_erc20_list,
                CustomERC20Count, CustomERC20List,
            },
            token_reader::TokenDataTriple,
        },
    },
    goblin_error::GoblinError,
    input_processor::{
        Decodable, DecodablePrimitive, DecodeCtx, FixedDecode, HeaderFlags, MsgTransfers,
        VariableDecode,
    },
    quantities::UnsidedAtoms,
    settlement::Delta,
    types::{Address, StoreReader, Tuple},
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

impl<'a> VariableDecode<'a> for GlobalHeader<'a> {
    type Flags = HeaderFlags;

    fn size(flags: &Self::Flags) -> usize {
        (flags.withdraw_eth as usize * UnsidedAtoms::ENCODED_SIZE)
            + (flags.read_custom_recipient as usize * core::mem::size_of::<Address>())
            + HardcodedCounts::ENCODED_SIZE
            + (flags.process_dynamic_markets as usize * DynamicCounts::ENCODED_SIZE)
            + CustomERC20Count::size(flags)
        // is logic wrong?
        // we cannot get full size from flags because length of custom erc20 list depends
        // on the value read from CustomERC20Count?
    }

    fn raw_variable_decode(ctx: &'a DecodeCtx, flags: &Self::Flags) -> Self {
        let eth_out_due = if flags.withdraw_eth {
            UnsidedAtoms::raw_fixed_decode(ctx)
        } else {
            UnsidedAtoms::default()
        };

        let custom_recipient = if flags.read_custom_recipient {
            Some(ctx.zero_copy_unchecked::<Address>())
        } else {
            None
        };

        // TODO account for 2 bytes
        let hardcoded_counts = HardcodedCounts::raw_fixed_decode(ctx);

        let dynamic_counts = if flags.process_dynamic_markets {
            DynamicCounts::raw_fixed_decode(ctx)
        } else {
            DynamicCounts::default()
        };

        let market_counts = MarketVariantPair::new(hardcoded_counts, dynamic_counts);

        let custom_erc20_count = CustomERC20Count::raw_variable_decode(ctx, flags);
        let custom_erc20_list = CustomERC20List::raw_variable_decode(ctx, &custom_erc20_count);
        let token_data_triple = TokenDataTriple::from(custom_erc20_list);

        Self {
            eth_out_due,
            custom_recipient,
            market_counts,
            token_data_triple,
        }
    }
}

// impl<'a> GlobalHeader<'a> {
//     pub fn new(ctx: &'a DecodeCtx) -> Result<Self, GoblinError> {
//         let flags = HeaderFlags::try_decode(ctx)?;
//         let msg_transfers = MsgTransfers::try_new(ctx, &flags)?;

//         let recipient = if flags.read_custom_recipient {
//             Some(ctx.zero_copy_unchecked::<Address>())
//         } else {
//             None
//         };

//         // 2 bytes
//         let hardcoded_counts = HardcodedCounts::try_decode(ctx)?;

//         // 4 bytes but conditional
//         let dynamic_counts = if flags.process_dynamic_markets {
//             DynamicCounts::new(ctx)?
//         } else {
//             DynamicCounts::default()
//         };

//         let market_counts = Tuple::new(hardcoded_counts, dynamic_counts);

//         let custom_erc20_list = if flags.read_custom_erc20 {
//             CustomERC20List::try_decode(ctx)?
//         } else {
//             CustomERC20List::decode_empty(ctx)
//         };
//         let token_data_triple = TokenDataTriple::from(custom_erc20_list);

//         Ok(Self {
//             flags,
//             msg_transfers,
//             custom_recipient: recipient,
//             market_counts,
//             token_data_triple,
//         })
//     }

//     fn recipient(&'a self, msg_sender: &'a Address) -> &'a Address {
//         self.custom_recipient.unwrap_or(msg_sender)
//     }

//     pub fn process(
//         &'a self,
//         msg_sender: &'a Address,
//         ctx: &DecodeCtx,
//         delta: &mut Delta,
//     ) -> Result<(), GoblinError> {
//         let hardcoded_counts = Hardcoded::get_leg(&self.market_counts);
//         hardcoded_counts.process(msg_sender, ctx, &self.token_data_triple, delta)?;

//         let dynamic_counts = Dynamic::get_leg(&self.market_counts);
//         dynamic_counts.process(msg_sender, ctx, &self.token_data_triple, delta)?;

//         delta.global.settle(
//             self.recipient(msg_sender),
//             &self.token_data_triple,
//             &self.msg_transfers,
//         )
//     }
// }
