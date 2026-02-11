use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    market::{
        market_marker::{
            hardcoded::{HardcodedMarketIndex, HardcodedMarkets},
            MarketMarker,
        },
        MarketHeader, MarketLocator,
    },
    settlement::Delta,
    token::TokenMarker,
    types::Address,
};

pub fn process_market<'a, M, B, Q>(
    ctx: &DecodeCtx,
    msg_sender: &Address,
    erc20_list: M::ERC20List<'a>,
    delta: &mut Delta,
) -> Result<(), GoblinError>
where
    B: TokenMarker,
    Q: TokenMarker,
    M: MarketMarker,
    HardcodedMarketIndex<B, Q>: HardcodedMarkets<B, Q>,
{
    let market_header = MarketHeader::<M, B, Q>::try_decode(ctx)?;

    let market_locator = M::MarketLocator::<B, Q>::decode_locator(ctx, erc20_list)?;
    let market_and_key = market_locator.locate_market()?;

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
