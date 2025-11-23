use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, Decodable},
    markets::{CommonMarket, HardcodedMarketList, MarketHeader, MarketVariant, PairShape},
    quantities::DeltaAtoms,
    settlement::{global::GlobalDelta, market::MarketDelta},
    state::{HardcodedMarketKey, MarketState, SlotState},
    tokens::HardcodedIndex,
    types::Address,
};

/// A market hardcoded within the smart contract. It keccak hash is also hardcoded,
/// allowing slot reads without having to compute hash at runtime.
///
/// * All token indices in hardcoded markets are hardcoded.
/// * It has 3 variants corresponding to the 3 pair shapes
pub struct HardcodedMarket<P>
where
    P: PairShape,
{
    /// The common market configuration (lot sizes, tick size, token indices).
    pub common: CommonMarket<HardcodedIndex, P>,

    /// The hardcoded keccak256 hash.
    pub keccak_hash: HardcodedMarketKey<P>,
}

impl<P> HardcodedMarket<P>
where
    P: PairShape + 'static + Decodable<P::ResolvedPair<DeltaAtoms>>,
    P::ResolvedPair<DeltaAtoms>: Default,
    Self: HardcodedMarketList<P>,
{
    pub const DISCRIMINATOR: u8 = HardcodedIndex::DISCRIMINATOR | (P::DISCRIMINATOR << 1);

    // TODO define common trait for both market types if they have common arguments
    // Currently HardcodedMarket doesn't require custom_erc20_list
    pub fn process(
        msg_sender: &Address,
        market_header: &MarketHeader,
        global_delta: &mut GlobalDelta,
        payload: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<(), GoblinError> {
        let market = Self::decode(payload, offset, len)?;
        let mut market_state = MarketState::load(&market.keccak_hash).into_inner();

        let market_delta =
            MarketDelta::<P>::new(market_header.decode_deposit_amounts, payload, offset, len)?;

        // Take bid and take quote
        if market_header.execute_takes.base {
            // ix_take::<HardcodedIndex, P, Base>(
            //     &market_delta,
            //     msg_sender,
            //     market,
            //     &mut market_state,
            //     payload,
            //     offset,
            //     len,
            // )?;

            // let match_result = ix_take::<HardcodedIndex, P, Base>(
            //     &mut market_delta,
            //     msg_sender.as_ref(),
            //     &indexed_market,
            //     market_state.as_mut(),
            //     payload,
            //     offset,
            //     len,
            // )?;
        }

        // Apply market delta updates on global delta
        P::commit_delta(&market.common.token_index_pair, global_delta, &market_delta)?;

        Ok(())
    }
}
