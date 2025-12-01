use crate::{
    goblin_error::GoblinError,
    hostio::HostioContext,
    input_processor::Decodable,
    instructions::ix_take,
    markets::{CommonMarket, MarketHeader, MarketVariant, PairShape},
    quantities::DeltaAtoms,
    settlement::{global_delta::GlobalDelta, local_delta::LocalDelta},
    state::{DynamicMarketHasher, DynamicMarketKey, MarketState, SlotState},
    tokens::{CustomToken, DynamicIndex},
    types::Base,
};

/// A market whose token indices are dynamically specified at runtime.
/// Works with any token pair shape (ETH–ERC20, ERC20–ETH, ERC20–ERC20).

pub struct DynamicMarket<P>
where
    P: PairShape,
{
    /// The common market configuration (lot sizes, tick size, token indices).
    pub common: CommonMarket<DynamicIndex, P>,
}

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
        ctx: &HostioContext,
        market_header: &MarketHeader,
        global_delta: &mut GlobalDelta,
        custom_erc20_list: &[CustomToken],
        offset: &mut usize,
        len: usize,
    ) -> Result<(), GoblinError> {
        let market = Self::decode(&ctx.args, offset, len)?;
        let market_key = DynamicMarketKey::hash(&market.common, custom_erc20_list)?;
        let mut market_state = MarketState::load(&market_key).into_inner();

        let mut local_delta =
            LocalDelta::<P>::new(market_header.decode_deposit_amounts, &ctx.args, offset, len)?;

        // Take bid and take quote
        if market_header.execute_takes.base {
            ix_take::<DynamicIndex, P, Base>(
                ctx,
                &mut local_delta,
                &market.common,
                &mut market_state,
                offset,
                len,
            )?;
        }

        P::commit_delta(&market.common, global_delta, &local_delta)?;

        Ok(())
    }
}
