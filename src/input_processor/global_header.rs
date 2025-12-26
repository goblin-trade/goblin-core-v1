use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, Decodable, EthTransfers, HeaderFlags, MarketCounts},
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

impl Decodable for GlobalHeader {
    fn decode(args: &ArgsBuffer, offset: &mut usize, len: usize) -> Result<Self, GoblinError> {
        let flags = HeaderFlags::decode(args, offset, len)?;
        let market_counts = MarketCounts::decode(args, offset, len)?;
        let eth_transfers = EthTransfers::new(&flags, args, offset)?;

        Ok(Self {
            flags,
            market_counts,
            eth_transfers,
        })
    }
}
