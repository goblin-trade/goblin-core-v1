use crate::{
    axis::{
        market::{
            market_counts::{dynamic::DynamicCounts, hardcoded::HardcodedCounts, MarketCounts},
            Dynamic, Hardcoded, MarketVariantPair,
        },
        token::{token_list::custom_erc20::CustomERC20List, token_reader::TokenDataTriple},
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx, HeaderFlags, MsgTransfers},
    settlement::Delta,
    types::{Address, StoreReader, Tuple},
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
    pub custom_recipient: Option<&'a Address>,

    /// Number of hardcoded and dynamic markets to process
    pub market_counts: MarketVariantPair<HardcodedCounts, DynamicCounts>,

    pub token_data_triple: TokenDataTriple<'a>,
}

impl<'a> GlobalHeader<'a> {
    pub fn new(ctx: &'a DecodeCtx) -> Result<Self, GoblinError> {
        // TODO decode msg_sender here
        //
        // problem with DecodeV2 definition
        // Trait assumes we read purely from ctx
        // However this function performs hostio calls too
        //
        // We should separate hostio and calldata components?
        // But this breaks symmetric type MsgTransfers::ETHTransfers which has
        // msg_value from calldata and eth_out from ctx
        //
        // However if we perform a clean split, we can use #[derive(DecodableV2)]

        // This function decodes + performs hostio calls
        // msg_transfers holds msg_value for ETH as read from hostio
        //
        // TODO define conditional_decode(ctx, &flags);
        // Turn it into a trait ConditionalDecode
        let flags = HeaderFlags::try_decode(ctx)?;
        let msg_transfers = MsgTransfers::try_new(ctx, &flags)?;

        let recipient = if flags.recipient_provided {
            Some(ctx.zero_copy_unchecked::<Address>())
        } else {
            None
        };

        let hardcoded_counts = HardcodedCounts::try_decode(ctx)?;
        let dynamic_counts = if flags.process_dynamic_markets {
            DynamicCounts::new(ctx)?
        } else {
            DynamicCounts::default()
        };

        let market_counts = Tuple::new(hardcoded_counts, dynamic_counts);

        let custom_erc20_list = if flags.read_custom_erc20 {
            CustomERC20List::try_decode(ctx)?
        } else {
            CustomERC20List::decode_empty(ctx)
        };
        let token_data_triple = TokenDataTriple::from(custom_erc20_list);

        Ok(Self {
            flags,
            msg_transfers,
            custom_recipient: recipient,
            market_counts,
            token_data_triple,
        })
    }

    fn recipient(&'a self, msg_sender: &'a Address) -> &'a Address {
        self.custom_recipient.unwrap_or(msg_sender)
    }

    pub fn process(
        &'a self,
        msg_sender: &'a Address,
        ctx: &DecodeCtx,
        delta: &mut Delta,
    ) -> Result<(), GoblinError> {
        let hardcoded_counts = Hardcoded::get_leg(&self.market_counts);
        hardcoded_counts.process(msg_sender, ctx, &self.token_data_triple, delta)?;

        let dynamic_counts = Dynamic::get_leg(&self.market_counts);
        dynamic_counts.process(msg_sender, ctx, &self.token_data_triple, delta)?;

        delta.global.settle(
            self.recipient(msg_sender),
            &self.token_data_triple,
            &self.msg_transfers,
        )
    }
}
