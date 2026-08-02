use crate::{
    axis::{
        market::{
            header::market_header::MarketHeader,
            market_locator::{hardcoded::HardcodedMarketList, MarketLocator},
            market_marker::MarketMarker,
            token_pair::TokenPair,
            Readables, Writables,
        },
        token::token_reader::TokenDataTriple,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    settlement::Delta,
    types::Address,
};

#[inline(never)]
pub fn process_market<'a, M, TP>(
    msg_sender: &Address,
    ctx: &DecodeCtx,
    token_data_triple: &TokenDataTriple<'a>,
    delta: &mut Delta,
) -> Result<(), GoblinError>
where
    M: MarketMarker + MarketLocator<TP>,
    TP: TokenPair + HardcodedMarketList,
{
    let market_header = MarketHeader::<(M, TP)>::try_decode(ctx)?;

    if market_header.decode_deposit_amounts {
        delta.local.deposits.decode_and_set::<TP>(ctx)?;
    }

    let market_locator = M::decode_locator(ctx, token_data_triple)?;
    let market_readables = M::locate_market(&market_locator);

    let readables = &Readables {
        msg_sender,
        market_readables,
    };

    let market_state = &mut market_readables.market_key.load();
    let writables = &mut Writables {
        local_delta: &mut delta.local,
        market_state,
    };

    // TODO convert to axis- make and take?
    market_header.execute_takes(ctx, readables, writables)?;
    market_header.execute_makes(ctx, readables, writables)?;

    delta.commit_local_delta::<TP>(
        &market_readables.market.token_index_pair,
        &market_readables.market.lot_size_pair,
    )?;

    // Clear deposit amounts so that store can be used for the next market
    delta.local.deposits.reset::<TP>();

    Ok(())
}
