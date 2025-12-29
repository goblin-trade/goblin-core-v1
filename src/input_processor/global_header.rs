use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx, EthTransfers, HeaderFlags, MarketCounts},
};

/// Arguments read from calldata
pub struct GlobalHeader {
    /// Flags and counts. Tells whether optional values should be read.
    pub flags: HeaderFlags,

    /// Number of markets to decode, namespaced by type
    pub market_counts: MarketCounts,

    /// Amount of ETH transferred in and due to be transferred out
    pub eth_transfers: EthTransfers,
}

impl<'a> Decodable<'a> for GlobalHeader {
    fn decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        let flags = HeaderFlags::decode(ctx)?;
        let market_counts = MarketCounts::decode(ctx)?;
        let eth_transfers = EthTransfers::new(ctx, &flags)?;

        Ok(Self {
            flags,
            market_counts,
            eth_transfers,
        })
    }
}
