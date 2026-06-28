use crate::{
    axis::{
        market::{
            header::market_header::MarketHeader,
            market_locator::MarketLocator,
            market_marker::{
                hardcoded::{
                    hardcoded_market_index::HardcodedMarketIndex,
                    hardcoded_markets::HardcodedMarkets,
                },
                MarketMarker,
            },
            Readables, Writables,
        },
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    settlement::Delta,
    types::Address,
};

pub fn process_market<'a, M, B, Q>(
    msg_sender: &Address,
    ctx: &DecodeCtx,
    erc20_list: M::ERC20List<'a>,
    delta: &mut Delta,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    HardcodedMarketIndex<B, Q>: HardcodedMarkets<B, Q>,
{
    let market_header = MarketHeader::<M, B, Q>::try_decode(ctx)?;
    if market_header.decode_deposit_amounts {
        delta.local.deposits.read_deposits::<B, Q>(ctx)?;
    }

    let market_locator = M::MarketLocator::<B, Q>::decode_locator(ctx, erc20_list)?;
    let market_readables = market_locator.locate_market()?;

    let market_state = &mut market_readables.market_key.load();

    let readables = &Readables {
        msg_sender,
        market_readables,
    };

    let writables = &mut Writables {
        local_delta: &mut delta.local,
        market_state,
    };

    market_header.execute_takes(ctx, readables, writables)?;
    market_header.execute_makes(ctx, readables, writables)?;

    delta.commit_local_delta::<B, Q>(
        &market_readables.market.token_index_pair,
        &market_readables.market.lot_size_pair,
    )
}
