use crate::{
    goblin_error::GoblinError,
    hostio::HostioContext,
    input_processor::Decodable,
    instructions::ix_take,
    market::{Dynamic, Hardcoded, HardcodedMarketList, MarketAndKey, MarketHeader, MarketVariant},
    settlement::Delta,
    state::{DynamicMarketHasher, DynamicMarketKey, MarketState, SlotState},
    token::{CustomToken, TokenMarker, ERC20, ETH},
    types::{Base, Pair, Quote, TupleReader},
};

pub fn process_market<M, B, Q>(
    ctx: &HostioContext,
    offset: &mut usize,
    len: usize,
    delta: &mut Delta,
    custom_erc20_list: &[CustomToken],
) -> Result<(), GoblinError>
where
    M: MarketVariant,
    B: TokenMarker
        + 'static
        + TupleReader<
            <ETH as TokenMarker>::Deposit,
            <ERC20 as TokenMarker>::Deposit,
            (ETH, ERC20),
            Result = <B as TokenMarker>::Deposit,
        >,
    Q: TokenMarker
        + 'static
        + TupleReader<
            <ETH as TokenMarker>::Deposit,
            <ERC20 as TokenMarker>::Deposit,
            (ETH, ERC20),
            Result = <Q as TokenMarker>::Deposit,
        >,
    B::TokenIndex<Dynamic>: Decodable<B::TokenIndex<Dynamic>>,
    B::Deposit: Decodable<B::Deposit>,

    Q::TokenIndex<Dynamic>: Decodable<Q::TokenIndex<Dynamic>>,
    Q::Deposit: Decodable<Q::Deposit>,

    DynamicMarketKey<B, Q>: DynamicMarketHasher<B, Q>,
    MarketAndKey<Hardcoded, B, Q>: HardcodedMarketList<B, Q>,
    MarketState<M, B, Q>: SlotState<M::MarketKey<B, Q>>,
{
    let market_header = MarketHeader::decode(&ctx.args, offset, len)?;

    let decoded_market = M::decode(&ctx.args, offset, len, custom_erc20_list)?;
    let market_and_key = M::market_and_key_ref(&decoded_market)?;

    let mut market_state = MarketState::<M, B, Q>::load(&market_and_key.key).into_inner();

    if market_header.decode_deposit_amounts {
        let base_deposit = B::Deposit::decode(&ctx.args, offset, len)?;
        let quote_deposit = Q::Deposit::decode(&ctx.args, offset, len)?;
        let deposit_pair = Pair::new(base_deposit, quote_deposit);

        delta.local.deposits.set_deposits::<B, Q>(&deposit_pair);
    }

    // Take bid and take quote
    if Base::get(&market_header.execute_takes) {
        ix_take::<M, B, Q, Base>(
            ctx,
            offset,
            len,
            &mut delta.local,
            &market_and_key.market,
            &mut market_state,
        )?;
    }

    if Quote::get(&market_header.execute_takes) {
        ix_take::<M, B, Q, Quote>(
            ctx,
            offset,
            len,
            &mut delta.local,
            &market_and_key.market,
            &mut market_state,
        )?;
    }

    // // TODO commit local delta into global delta

    // Reset local delta for reuse
    delta.local.deposits.reset::<B, Q>();

    Ok(())
}
