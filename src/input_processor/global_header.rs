use crate::{
    goblin_error::GoblinError,
    input_processor::{
        Decodable, DecodeCtx, DynamicMarketHeader, EthTransfers, HardcodedMarketHeader, HeaderFlags,
    },
    types::Address,
};

/// Arguments read from calldata
pub struct GlobalHeader<'a> {
    /// Flags and counts. Tells whether optional values should be read.
    pub flags: HeaderFlags,

    /// Amount of ETH transferred in and due to be transferred out
    pub eth_transfers: EthTransfers,

    /// Hardcoded market header
    pub hardcoded_market_header: HardcodedMarketHeader,

    /// Dynamic market header
    pub dynamic_market_header: Option<DynamicMarketHeader<'a>>,

    /// Optional custom recipient
    pub recipient: Option<&'a Address>,
    // /// Addresses of custom erc20 tokens to use
    // pub custom_erc20_list: &'a [CustomERC20Data],
}

impl<'a> GlobalHeader<'a> {
    pub fn new(ctx: &'a DecodeCtx) -> Result<Self, GoblinError> {
        let flags = HeaderFlags::try_decode(ctx)?;
        let eth_transfers = EthTransfers::new(ctx, &flags)?;

        let hardcoded_market_header = HardcodedMarketHeader::try_decode(ctx)?;
        let dynamic_market_header = if flags.process_dynamic_markets {
            Some(DynamicMarketHeader::new(ctx)?)
        } else {
            None
        };

        let recipient = if flags.recipient_provided {
            Some(ctx.zero_copy_unchecked::<Address>())
        } else {
            None
        };

        // let custom_erc20_list =
        //     ctx.zero_copy_slice_unchecked::<CustomERC20Data>(flags.custom_erc20_count);

        Ok(Self {
            flags,
            hardcoded_market_header,
            dynamic_market_header,
            eth_transfers,
            recipient,
            // custom_erc20_list,
        })
    }
}
