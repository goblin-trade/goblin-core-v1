use crate::{
    input_processor::{
        ArgsReader, FixedCodec, HeaderFlags, MarketCounts, VariableDecode,
        global_args::header::Header,
    },
    quantities::UnsidedAtoms,
};

impl<'a> VariableDecode<'a> for Header {
    type Flags = HeaderFlags;

    fn size(flags: &Self::Flags) -> usize {
        (flags.withdraw_eth as usize * UnsidedAtoms::<u32>::ENCODED_SIZE)
            + MarketCounts::size(&flags.process_dynamic_markets)
    }

    fn raw_variable_decode(reader: &'a ArgsReader, flags: &Self::Flags) -> Self {
        let eth_out_due = if flags.withdraw_eth {
            UnsidedAtoms::<u32>::raw_fixed_decode(reader)
        } else {
            UnsidedAtoms::default()
        };

        let market_counts =
            MarketCounts::raw_variable_decode(reader, &flags.process_dynamic_markets);

        Self {
            eth_out_due_u32: eth_out_due,
            market_counts,
        }
    }
}
