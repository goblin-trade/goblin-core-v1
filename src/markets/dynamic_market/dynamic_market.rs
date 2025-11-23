use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, Decodable},
    markets::{CommonMarket, MarketHeader, MarketVariant, PairShape},
    quantities::DeltaAtoms,
    settlement::{global::GlobalDelta, market::MarketDelta},
    state::{DynamicMarketHasher, DynamicMarketKey, MarketState, SlotState},
    tokens::{CustomToken, DynamicIndex},
    types::Address,
};

/// A market whose token indices are dynamically specified at runtime.
/// Works with any token pair shape (ETH–ERC20, ERC20–ETH, ERC20–ERC20).
pub type DynamicMarket<P: PairShape> = CommonMarket<DynamicIndex, P>;

impl<P> DynamicMarket<P>
where
    P: PairShape
        + Decodable<P::ResolvedPair<DynamicIndex>>
        + Decodable<P::ResolvedPair<DeltaAtoms>>,
    P::ResolvedPair<DeltaAtoms>: Default,
    DynamicMarketKey<P>: DynamicMarketHasher<P>,
{
    pub const DISCRIMINATOR: u8 = DynamicIndex::DISCRIMINATOR | (P::DISCRIMINATOR << 1);

    pub fn process(
        msg_sender: &Address,
        market_header: &MarketHeader,
        global_delta: &mut GlobalDelta,
        custom_erc20_list: &[CustomToken],
        payload: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<(), GoblinError> {
        let market = Self::decode(payload, offset, len)?;
        let market_key = DynamicMarketKey::hash(&market, custom_erc20_list)?;
        let market_state = MarketState::load(&market_key);

        let market_delta =
            MarketDelta::<P>::new(market_header.decode_deposit_amounts, payload, offset, len)?;

        P::commit_delta(&market.token_index_pair, global_delta, &market_delta)?;

        Ok(())
    }
}
