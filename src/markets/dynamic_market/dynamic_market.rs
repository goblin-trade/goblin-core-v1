use crate::{
    goblin_error::GoblinError,
    hostio::HostioContext,
    input_processor::Decodable,
    instructions::ix_take,
    markets::{CommonMarket, MarketHeader, PairShape},
    quantities::DeltaAtoms,
    settlement::Delta,
    state::{DynamicMarketHasher, DynamicMarketKey, MarketState, SlotState},
    token::{CustomToken, DynamicIndex, TokenMarker, ERC20, ETH},
    types::{Base, Pair, Quote, TripleReader, TupleReader},
};

/// A market whose token indices are dynamically specified at runtime.
/// Works with any token pair shape (ETH–ERC20, ERC20–ETH, ERC20–ERC20).

pub struct DynamicMarket<B, Q>
where
    B: TokenMarker,
    Q: TokenMarker,
{
    /// The common market configuration (lot sizes, tick size, token indices).
    pub common: CommonMarket<DynamicIndex, B, Q>,
}

impl<B, Q> DynamicMarket<B, Q>
where
    B: TokenMarker,
    Q: TokenMarker,
    // P: PairShape
    //     + Decodable<P::ResolvedPair<DynamicIndex>>
    //     + Decodable<P::ResolvedPair<DeltaAtoms>>
    //     + TripleReader<
    //         DeltaAtoms,
    //         DeltaAtoms,
    //         Pair<DeltaAtoms, DeltaAtoms>,
    //         ((ETH, ERC20), (ERC20, ETH), (ERC20, ERC20)),
    //         Result = P::ResolvedPair<DeltaAtoms>,
    //     >,
    // P::ResolvedPair<DeltaAtoms>: Default,
    // DynamicMarketKey<P>: DynamicMarketHasher<P>,
{
    pub fn process(
        ctx: &HostioContext,
        market_header: &MarketHeader,
        delta: &mut Delta,
        custom_erc20_list: &[CustomToken],
        offset: &mut usize,
        len: usize,
    ) -> Result<(), GoblinError> {
        let market = Self::decode(&ctx.args, offset, len)?;
        let market_key = DynamicMarketKey::hash(&market.common, custom_erc20_list)?;
        let mut market_state = MarketState::load(&market_key).into_inner();

        if market_header.decode_deposit_amounts {
            // Decode and set deposit amounts
            // New format- decode for base first, then quote?
            // B::decode()?
            let deposit_pair = P::get_leg_mut(&mut delta.local.deposits);
            *deposit_pair = P::decode(&ctx.args, offset, len)?;
        }

        // Take bid and take quote
        if Base::get(&market_header.execute_takes) {
            ix_take::<DynamicIndex, P, Base>(
                ctx,
                &mut delta.local,
                &market.common,
                &mut market_state,
                offset,
                len,
            )?;
        }

        if Quote::get(&market_header.execute_takes) {
            ix_take::<DynamicIndex, P, Quote>(
                ctx,
                &mut delta.local,
                &market.common,
                &mut market_state,
                offset,
                len,
            )?;
        }

        // P::commit_local_delta(&market.common, delta)?;

        // Reset local delta for reuse
        // delta.local.reset::<P>();

        Ok(())
    }
}
