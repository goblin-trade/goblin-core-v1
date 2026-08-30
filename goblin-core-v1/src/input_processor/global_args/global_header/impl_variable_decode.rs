use crate::{
    axis::token::{token_list::custom_erc20::CustomERC20List, token_reader::TokenDataTriple},
    input_processor::{
        global_args::global_header::GlobalHeader, ArgsReader, FixedDecode, HeaderFlags,
        MarketCounts, VariableDecode,
    },
    quantities::UnsidedAtoms,
    types::Address,
};

impl<'a> VariableDecode<'a> for GlobalHeader<'a> {
    type Flags = HeaderFlags;

    fn size(flags: &Self::Flags) -> usize {
        (flags.withdraw_eth as usize * UnsidedAtoms::ENCODED_SIZE)
            + (flags.read_custom_recipient as usize * core::mem::size_of::<Address>())
            + MarketCounts::size(&flags.process_dynamic_markets)
            + CustomERC20List::size(flags)
    }

    fn raw_variable_decode(reader: &'a ArgsReader, flags: &Self::Flags) -> Self {
        let eth_out_due = if flags.withdraw_eth {
            UnsidedAtoms::raw_fixed_decode(reader)
        } else {
            UnsidedAtoms::default()
        };

        let custom_recipient = if flags.read_custom_recipient {
            Some(reader.zero_copy_unchecked::<Address>())
        } else {
            None
        };

        let market_counts =
            MarketCounts::raw_variable_decode(reader, &flags.process_dynamic_markets);

        let custom_erc20_list = CustomERC20List::raw_variable_decode(reader, &flags);
        let token_data_triple = TokenDataTriple::from(custom_erc20_list);

        Self {
            eth_out_due,
            custom_recipient,
            market_counts,
            token_data_triple,
        }
    }
}
