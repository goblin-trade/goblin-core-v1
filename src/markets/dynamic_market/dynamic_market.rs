use crate::{
    goblin_error::GoblinError,
    hostio::HostioContext,
    input_processor::Decodable,
    instructions::ix_take,
    markets::{CommonMarket, Dynamic, MarketHeader, PairShape},
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
    pub common: CommonMarket<Dynamic, B, Q>,
}

impl<B, Q> DynamicMarket<B, Q>
where
    B: TokenMarker
        + Decodable<B::Deposit>
        + TupleReader<
            <ETH as TokenMarker>::Deposit,
            <ERC20 as TokenMarker>::Deposit,
            (ETH, ERC20),
            Result = <B as TokenMarker>::Deposit,
        >,
    Q: TokenMarker
        + Decodable<Q::Deposit>
        + TupleReader<
            <ETH as TokenMarker>::Deposit,
            <ERC20 as TokenMarker>::Deposit,
            (ETH, ERC20),
            Result = <Q as TokenMarker>::Deposit,
        >,
    DynamicMarket<B, Q>: Decodable<DynamicMarket<B, Q>>,
    DynamicMarketKey<B, Q>: DynamicMarketHasher<B, Q>, // P: PairShape
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
            let base_deposit = B::decode(&ctx.args, offset, len)?;
            let quote_deposit = Q::decode(&ctx.args, offset, len)?;
            let deposit_pair = Pair::new(base_deposit, quote_deposit);

            delta.local.deposits.set_deposits::<B, Q>(&deposit_pair);
        }

        // Take bid and take quote
        if Base::get(&market_header.execute_takes) {
            ix_take::<Dynamic, B, Q, Base>(
                ctx,
                &mut delta.local,
                &market.common,
                &mut market_state,
                offset,
                len,
            )?;
        }

        if Quote::get(&market_header.execute_takes) {
            ix_take::<Dynamic, B, Q, Quote>(
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
        delta.local.deposits.reset::<B, Q>();

        Ok(())
    }
}
