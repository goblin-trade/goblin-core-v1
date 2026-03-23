use crate::{
    axis::{
        leg::{Base, Quote},
        market::{
            header::{
                inner_bitmap_header::InnerBitmapHeader, market_header::MarketHeader,
                outer_bitmap_header::OuterBitmapHeader,
            },
            market_locator::MarketLocator,
            market_marker::{
                hardcoded::{
                    hardcoded_market_index::HardcodedMarketIndex,
                    hardcoded_markets::HardcodedMarkets,
                },
                MarketMarker,
            },
        },
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    settlement::Delta,
    types::StoreReader,
};

pub fn process_market<'a, M, B, Q>(
    ctx: &DecodeCtx,
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

    let mut market_state = market_and_key.market_key.load();

    if market_header.decode_deposit_amounts {
        delta.local.deposits.set_deposits::<B, Q>(ctx)?;
    }

    market_header.execute_takes(ctx, &mut delta.local, market_and_key, &mut market_state)?;

    // The same bitmap can have both takes and makes
    for _ in 0..market_header.outer_bitmap_count {
        let outer_bitmap_header = OuterBitmapHeader::try_decode(ctx)?;

        for _ in 0..outer_bitmap_header.inner_bitmap_count {
            let inner_bitmap_header = InnerBitmapHeader::try_decode(ctx)?;

            for _ in 0..Base::get(&inner_bitmap_header.update_count) {}
            for _ in 0..Quote::get(&inner_bitmap_header.update_count) {}
        }
    }

    // Reset local delta for reuse
    delta.local.deposits.reset::<B, Q>();

    Ok(())
}
