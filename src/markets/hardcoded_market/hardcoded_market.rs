use crate::{
    goblin_error::GoblinError,
    hostio::HostioContext,
    input_processor::Decodable,
    instructions::ix_take,
    markets::{CommonMarket, HardcodedMarketList, MarketHeader, PairShape},
    quantities::DeltaAtoms,
    settlement::{local_delta::DepositPair, Delta},
    state::{HardcodedMarketKey, MarketState, SlotState},
    token::{HardcodedIndex, TokenMarker, ERC20, ETH},
    types::{Base, Pair, TripleReader, TupleReader},
};

/// A market hardcoded within the smart contract. It keccak hash is also hardcoded,
/// allowing slot reads without having to compute hash at runtime.
///
/// * All token indices in hardcoded markets are hardcoded.
/// * It has 3 variants corresponding to the 3 pair shapes
pub struct HardcodedMarket<B, Q>
where
    B: TokenMarker,
    Q: TokenMarker,
{
    /// The common market configuration (lot sizes, tick size, token indices).
    pub common: CommonMarket<HardcodedIndex, B, Q>,

    /// The hardcoded keccak256 hash.
    pub keccak_hash: HardcodedMarketKey<B, Q>,
}

impl<B, Q> HardcodedMarket<B, Q>
where
    B: TokenMarker + 'static + Decodable<B::Deposit>,
    Q: TokenMarker + 'static + Decodable<Q::Deposit>,
    // P: PairShape
    //     + 'static
    //     + Decodable<P::ResolvedPair<DeltaAtoms>>
    //     + TripleReader<
    //         DeltaAtoms,
    //         DeltaAtoms,
    //         Pair<DeltaAtoms, DeltaAtoms>,
    //         ((ETH, ERC20), (ERC20, ETH), (ERC20, ERC20)),
    //         Result = P::ResolvedPair<DeltaAtoms>,
    //     >,
    // P::ResolvedPair<DeltaAtoms>: Default,
    // Self: HardcodedMarketList<P>,
{
    // TODO define common trait for both market types if they have common arguments
    // Currently HardcodedMarket doesn't require custom_erc20_list
    pub fn process(
        ctx: &HostioContext,
        market_header: &MarketHeader,
        delta: &mut Delta,
        offset: &mut usize,
        len: usize,
    ) -> Result<(), GoblinError> {
        let market = Self::decode(&ctx.args, offset, len)?;
        let mut market_state = MarketState::load(&market.keccak_hash).into_inner();

        if market_header.decode_deposit_amounts {
            let base_deposit = B::decode(&ctx.args, offset, len)?;
            let quote_deposit = Q::decode(&ctx.args, offset, len)?;

            let deposit_pair = DepositPair::new(base_deposit, quote_deposit);

            // // TODO add deposit directly here? It could make P::commit_local_delta() cleaner
            // let deposit_pair = P::get_leg_mut(&mut delta.local.deposits);
            // *deposit_pair = P::decode(&ctx.args, offset, len)?;
        }

        // // Take bid and take quote
        // if Base::get(&market_header.execute_takes) {
        //     ix_take::<HardcodedIndex, P, Base>(
        //         ctx,
        //         &mut delta.local,
        //         &market.common,
        //         &mut market_state,
        //         offset,
        //         len,
        //     )?;
        // }

        // // Apply market delta updates on global delta
        // // P::commit_local_delta(&market.common, delta)?;

        // // Reset local delta for reuse
        // delta.local.reset::<P>();

        Ok(())
    }
}
