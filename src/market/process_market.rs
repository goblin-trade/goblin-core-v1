use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    market::{Dynamic, Hardcoded, HardcodedMarketList, MarketAndKey, MarketHeader, MarketVariant},
    quantities::DeltaAtoms,
    settlement::Delta,
    token::{CustomERC20Data, HardcodedERC20, TokenMarker, ERC20, ETH},
    types::{Address, TupleMarker, TupleReader},
};

// problem- ERC20 doesn't implement TokenMarker now
// We must use HardcodedERC20 or CustomERC20
pub fn process_market<'a, M, B, Q>(
    ctx: &'a DecodeCtx<'a>,
    msg_sender: &Address,
    custom_erc20_list: &[CustomERC20Data],
    delta: &mut Delta,
) -> Result<(), GoblinError>
where
    M: MarketVariant,
    B: TokenMarker + 'static,
    B::TupleMarker: TupleReader<
        (),
        DeltaAtoms,
        (ETH, ERC20),
        Result = <B::TupleMarker as TupleMarker>::Deposit,
    >,
    Q: TokenMarker + 'static,
    Q::TupleMarker: TupleReader<
        (),
        DeltaAtoms,
        (ETH, ERC20),
        Result = <Q::TupleMarker as TupleMarker>::Deposit,
    >,
    B::TokenIndex: Decodable<'a>,
    Q::TokenIndex: Decodable<'a>,
    B::Deposit: Decodable<'a>,
    Q::Deposit: Decodable<'a>,
    MarketAndKey<Hardcoded, B, Q>: HardcodedMarketList<B, Q>,
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
