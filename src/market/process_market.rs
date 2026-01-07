use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    market::{Dynamic, Hardcoded, HardcodedMarketList, MarketAndKey, MarketHeader, MarketVariant},
    settlement::Delta,
    state::MarketState,
    token::{CustomToken, TokenMarker, ERC20, ETH},
    types::{Address, TupleReader},
};

pub fn process_market<'a, M, B, Q>(
    ctx: &'a DecodeCtx<'a>,
    msg_sender: &Address,
    custom_erc20_list: &[CustomToken],
    delta: &mut Delta,
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
    B::TokenIndex<Dynamic>: Decodable<'a>,
    Q::TokenIndex<Dynamic>: Decodable<'a>,
    B::Deposit: Decodable<'a>,
    Q::Deposit: Decodable<'a>,
    MarketAndKey<Hardcoded, B, Q>: HardcodedMarketList<B, Q>,
    // MarketState<Dynamic, B, Q>: DynamicMarketHasher<B, Q>,
{
    let market_header = MarketHeader::<M, B, Q>::try_decode(ctx)?;

    let decoded_market = M::decode(ctx, custom_erc20_list)?;
    let market_and_key = M::market_and_key_ref(&decoded_market)?;

    let mut market_state = market_and_key.key.load();

    market_header.set_deposits(ctx, delta)?;
    market_header.execute_takes(
        ctx,
        msg_sender,
        &mut delta.local,
        &market_and_key.market,
        &mut market_state,
    )?;

    // // // TODO commit local delta into global delta

    // // Reset local delta for reuse
    // delta.local.deposits.reset::<B, Q>();

    Ok(())
}
