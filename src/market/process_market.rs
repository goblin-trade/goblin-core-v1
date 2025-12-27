use crate::{
    goblin_error::GoblinError,
    hostio::HostioContext,
    input_processor::Decodable,
    market::{Dynamic, Hardcoded, HardcodedMarketList, MarketAndKey, MarketHeader, MarketVariant},
    settlement::Delta,
    state::{DynamicMarketHasher, DynamicMarketKey, MarketState, SlotState},
    token::{CustomToken, TokenMarker, ERC20, ETH},
    types::TupleReader,
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
    B::TokenIndex<Dynamic>: Decodable,
    Q::TokenIndex<Dynamic>: Decodable,

    DynamicMarketKey<B, Q>: DynamicMarketHasher<B, Q>,
    MarketAndKey<Hardcoded, B, Q>: HardcodedMarketList<B, Q>,
    MarketState<M, B, Q>: SlotState<M::MarketKey<B, Q>>,
{
    let market_header = MarketHeader::<M, B, Q>::decode(&ctx.args, offset, len)?;

    let decoded_market = M::decode(&ctx.args, offset, len, custom_erc20_list)?;
    let market_and_key = M::market_and_key_ref(&decoded_market)?;

    let mut market_state = MarketState::<M, B, Q>::load(&market_and_key.key);

    market_header.set_deposits(&ctx.args, offset, len, delta)?;
    market_header.execute_takes(
        ctx,
        offset,
        len,
        &mut delta.local,
        &market_and_key.market,
        market_state.as_mut(),
    )?;

    // // // TODO commit local delta into global delta

    // // Reset local delta for reuse
    // delta.local.deposits.reset::<B, Q>();

    Ok(())
}
