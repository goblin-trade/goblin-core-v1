use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx, EthTransfers, HeaderFlags, MarketCounts},
    token::CustomToken,
    types::Address,
};

/// Arguments read from calldata
pub struct GlobalHeader<'a> {
    /// Flags and counts. Tells whether optional values should be read.
    pub flags: HeaderFlags,

    /// Number of markets to decode, namespaced by type
    pub market_counts: MarketCounts,

    /// Amount of ETH transferred in and due to be transferred out
    pub eth_transfers: EthTransfers,

    /// Optional custom recipient
    pub recipient: Option<&'a Address>,

    /// Addresses of custom erc20 tokens to use
    pub custom_erc20_list: &'a [CustomToken],
}

impl<'a> Decodable<'a> for GlobalHeader<'a> {
    fn decode(ctx: &'a DecodeCtx<'a>) -> Result<Self, GoblinError> {
        let flags = HeaderFlags::decode(ctx)?;
        let market_counts = MarketCounts::decode(ctx)?;
        let eth_transfers = EthTransfers::new(ctx, &flags)?;

        let recipient = if flags.recipient_provided {
            Some(ctx.decode_ref_unchecked::<Address>())
        } else {
            None
        };

        let custom_erc20_list = ctx.decode_slice_unchecked::<CustomToken>(flags.custom_erc20_count);

        Ok(Self {
            flags,
            market_counts,
            eth_transfers,
            recipient,
            custom_erc20_list,
        })
    }
}
