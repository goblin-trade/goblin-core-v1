use crate::{
    goblin_error::GoblinError,
    hostio::HostioContext,
    input_processor::Decodable,
    instructions::ix_take,
    markets::{CommonMarket, HardcodedMarketList, MarketHeader, MarketVariant, PairShape},
    quantities::DeltaAtoms,
    settlement::{global_delta::GlobalDelta, local_delta::LocalDelta},
    state::{HardcodedMarketKey, MarketState, SlotState},
    tokens::HardcodedIndex,
    types::Base,
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
        ctx: &HostioContext,
        market_header: &MarketHeader,
        global_delta: &mut GlobalDelta,
        offset: &mut usize,
        len: usize,
    ) -> Result<(), GoblinError> {
        let market = Self::decode(&ctx.args, offset, len)?;
        let mut market_state = MarketState::load(&market.keccak_hash).into_inner();

        let mut local_delta =
            LocalDelta::<P>::new(market_header.decode_deposit_amounts, &ctx.args, offset, len)?;

        // Take bid and take quote
        if market_header.execute_takes.base {
            ix_take::<HardcodedIndex, P, Base>(
                ctx,
                &mut local_delta,
                &market.common,
                &mut market_state,
                offset,
                len,
            )?;
        }

        // Apply market delta updates on global delta
        P::commit_delta(&market.common, global_delta, &local_delta)?;

        Ok(())
    }
}
