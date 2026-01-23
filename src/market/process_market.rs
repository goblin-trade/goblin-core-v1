use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    market::{Hardcoded, HardcodedMarketList, MarketAndKey, MarketHeader, MarketVariant},
    settlement::Delta,
    token::{CustomERC20Data, TokenMarker},
    types::Address,
};

pub fn process_market<'a, M, B, Q>(
    ctx: &'a DecodeCtx<'a>,
    msg_sender: &Address,
    custom_erc20_list: &[CustomERC20Data],
    delta: &mut Delta,
) -> Result<(), GoblinError>
where
    M: MarketVariant,
    B: TokenMarker + 'static,
    Q: TokenMarker + 'static,
    B::TokenIndex: Decodable<'a>,
    Q::TokenIndex: Decodable<'a>,
    B::Deposit: Decodable<'a>,
    Q::Deposit: Decodable<'a>,
    MarketAndKey<Hardcoded, B, Q>: HardcodedMarketList<B, Q>,
{
    let market_header = MarketHeader::<M, B, Q>::try_decode(ctx)?;

    let decoded_market: <M as MarketVariant>::DecodedMarket<B, Q> =
        M::decode(ctx, custom_erc20_list)?;
    let market_and_key = M::market_and_key_ref(&decoded_market)?;

    let mut market_state = market_and_key.key.load();

    if market_header.decode_deposit_amounts {
        delta.local.deposits.set_deposits::<B, Q>(ctx)?;
    }

    market_header.execute_takes(
        ctx,
        msg_sender,
        &mut delta.local,
        &market_and_key.market,
        &mut market_state,
    )?;

    // // TODO commit local delta into global delta

    // Reset local delta for reuse
    delta.local.deposits.reset::<B, Q>();

    Ok(())
}
