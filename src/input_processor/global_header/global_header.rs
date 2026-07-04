use crate::{
    axis::market::{
        market_counts::{dynamic::DynamicCounts, hardcoded::HardcodedCounts},
        MarketVariantPair,
    },
    axis::token::token_global_transfer::ETHTransfers,
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx, HeaderFlags},
    types::{Address, Tuple},
};

/// Arguments read from calldata
pub struct GlobalHeader<'a> {
    /// Flags and counts. Tells whether optional values should be read.
    pub flags: HeaderFlags,

    /// Amount of ETH transferred in and due to be transferred out
    pub eth_transfers: ETHTransfers,

    /// Optional custom recipient
    pub recipient: Option<&'a Address>,

    /// Number of hardcoded and dynamic markets to process
    pub market_counts: MarketVariantPair<HardcodedCounts, Option<DynamicCounts<'a>>>,
}

impl<'a> GlobalHeader<'a> {
    pub fn new(ctx: &'a DecodeCtx) -> Result<Self, GoblinError> {
        let flags = HeaderFlags::try_decode(ctx)?;
        let eth_transfers = ETHTransfers::new(ctx, &flags)?;

        let recipient = if flags.recipient_provided {
            Some(ctx.zero_copy_unchecked::<Address>())
        } else {
            None
        };

        let hardcoded_counts = HardcodedCounts::try_decode(ctx)?;
        let dynamic_counts = if flags.process_dynamic_markets {
            Some(DynamicCounts::new(ctx)?)
        } else {
            None
        };

        let market_counts = Tuple::new(hardcoded_counts, dynamic_counts);

        Ok(Self {
            flags,
            market_counts,
            eth_transfers,
            recipient,
        })
    }
}
