use crate::{
    axis::market::{
        market_counts::{dynamic::DynamicCounts, hardcoded::HardcodedCounts},
        MarketVariantPair,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx, HeaderFlags, MsgTransfers},
    types::{Address, Tuple},
};

/// Arguments read from calldata
pub struct GlobalHeader<'a> {
    /// Flags and counts. Tells whether optional values should be read.
    pub flags: HeaderFlags,

    /// Tokens transferred at the top level through calldata
    ///
    /// * ETH is deposited via msg.value. ETH withdraw amount is namespaced at calldata level
    /// not market namespace level.
    ///
    /// * ERC20 tokens deltas are read at the market level. They are stubs in the calldata level.
    pub msg_transfers: MsgTransfers,

    /// Optional custom recipient
    pub recipient: Option<&'a Address>,

    /// Number of hardcoded and dynamic markets to process
    pub market_counts: MarketVariantPair<HardcodedCounts, Option<DynamicCounts<'a>>>,
}

impl<'a> GlobalHeader<'a> {
    pub fn new(ctx: &'a DecodeCtx) -> Result<Self, GoblinError> {
        let flags = HeaderFlags::try_decode(ctx)?;
        let msg_transfers = MsgTransfers::try_new(ctx, &flags)?;

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
            msg_transfers,
            recipient,
        })
    }
}
