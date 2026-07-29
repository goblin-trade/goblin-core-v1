use crate::{
    axis::{
        leg::Base,
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
            market_spec::MarketSpec,
            token_pair::TokenPair,
            Readables, Writables,
        },
        token::{token_marker::TokenMarker, token_reader::TokenDataTriple},
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    settlement::Delta,
    types::{Address, StoreReader},
};
pub fn process_market<'a, MS>(
    msg_sender: &Address,
    ctx: &DecodeCtx,
    token_data_triple: &TokenDataTriple<'a>,
    delta: &mut Delta,
) -> Result<(), GoblinError>
where
    MS: MarketSpec,
    HardcodedMarketIndex<MS::Base, MS::Quote>: HardcodedMarkets<MS::Base, MS::Quote>,
{
    let market_header = MarketHeader::try_decode(ctx)?;

    // if market_header.decode_deposit_amounts {
    //     delta.local.deposits.decode_and_set::<B, Q>(ctx)?;
    // }

    // let market_locator = M::MarketLocator::<B, Q>::decode_locator(ctx, token_data_triple)?;
    // let market_readables = market_locator.locate_market()?;

    // let market_state = &mut market_readables.market_key.load();

    // let readables = &Readables {
    //     msg_sender,
    //     market_readables,
    // };

    // let writables = &mut Writables {
    //     local_delta: &mut delta.local,
    //     market_state,
    // };

    // // TODO convert to axis- make and take?
    // market_header.execute_takes(ctx, readables, writables)?;
    // market_header.execute_makes(ctx, readables, writables)?;

    // delta.commit_local_delta::<B, Q>(
    //     &market_readables.market.token_index_pair,
    //     &market_readables.market.lot_size_pair,
    // )?;

    // // Clear deposit amounts so that store can be used for the next market
    // delta.local.deposits.reset::<B, Q>();

    Ok(())
}

// pub fn process_market<'a, M, B, Q>(
//     msg_sender: &Address,
//     ctx: &DecodeCtx,
//     token_data_triple: &TokenDataTriple<'a>,
//     delta: &mut Delta,
// ) -> Result<(), GoblinError>
// where
//     M: MarketMarker,
//     B: TokenMarker,
//     Q: TokenMarker,
//     HardcodedMarketIndex<B, Q>: HardcodedMarkets<B, Q>,
// {
//     let market_header = MarketHeader::<M, B, Q>::try_decode(ctx)?;

//     if market_header.decode_deposit_amounts {
//         delta.local.deposits.decode_and_set::<B, Q>(ctx)?;
//     }

//     let market_locator = M::MarketLocator::<B, Q>::decode_locator(ctx, token_data_triple)?;
//     let market_readables = market_locator.locate_market()?;

//     let market_state = &mut market_readables.market_key.load();

//     let readables = &Readables {
//         msg_sender,
//         market_readables,
//     };

//     let writables = &mut Writables {
//         local_delta: &mut delta.local,
//         market_state,
//     };

//     // TODO convert to axis- make and take?
//     market_header.execute_takes(ctx, readables, writables)?;
//     market_header.execute_makes(ctx, readables, writables)?;

//     delta.commit_local_delta::<B, Q>(
//         &market_readables.market.token_index_pair,
//         &market_readables.market.lot_size_pair,
//     )?;

//     // Clear deposit amounts so that store can be used for the next market
//     delta.local.deposits.reset::<B, Q>();

//     Ok(())
// }
