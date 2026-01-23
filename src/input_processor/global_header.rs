use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx, EthTransfers, HeaderFlags, MarketCounts},
    token::CustomERC20Data,
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
    pub custom_erc20_list: &'a [CustomERC20Data],
}

impl<'a> GlobalHeader<'a> {
    pub fn new(ctx: &'a DecodeCtx) -> Result<Self, GoblinError> {
        let flags = HeaderFlags::try_decode(ctx)?;
        let market_counts = MarketCounts::try_decode(ctx)?;
        let eth_transfers = EthTransfers::new(ctx, &flags)?;

        let recipient = if flags.recipient_provided {
            Some(ctx.zero_copy_unchecked::<Address>())
        } else {
            None
        };

        let custom_erc20_list =
            ctx.zero_copy_slice_unchecked::<CustomERC20Data>(flags.custom_erc20_count);

        Ok(Self {
            flags,
            market_counts,
            eth_transfers,
            recipient,
            custom_erc20_list,
        })
    }
}
