use crate::{
    axis::{
        market::{
            market_counts::{dynamic::DynamicCounts, hardcoded::HardcodedCounts},
            MarketVariantPair,
        },
        token::{token_list::custom_erc20::CustomERC20List, token_reader::TokenDataTriple},
    },
    input_processor::{
        global_args::global_header::GlobalHeader, DecodeCtx, FixedDecode, HeaderFlags, VariableDecode,
    },
    quantities::UnsidedAtoms,
    types::Address,
};

impl<'a> VariableDecode<'a> for GlobalHeader<'a> {
    type Flags = HeaderFlags;

    fn size(flags: &Self::Flags) -> usize {
        (flags.withdraw_eth as usize * UnsidedAtoms::ENCODED_SIZE)
            + (flags.read_custom_recipient as usize * core::mem::size_of::<Address>())
            + HardcodedCounts::ENCODED_SIZE
            + (flags.process_dynamic_markets as usize * DynamicCounts::ENCODED_SIZE)
            + CustomERC20List::size(flags)
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

        let custom_erc20_list = CustomERC20List::raw_variable_decode(ctx, &flags);
        let token_data_triple = TokenDataTriple::from(custom_erc20_list);

        Self {
            eth_out_due,
            custom_recipient,
            market_counts,
            token_data_triple,
        }
    }
}
