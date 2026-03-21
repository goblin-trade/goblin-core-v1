use crate::{
    axis::{
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

    // Indices hold In: Legmarker generic
    // We need separate operations for each side?
    // Correct. Suppose if we want to open a bid resting order at position X, but the
    // market updates and X falls on ask side. We must then revert.
    //
    // Problem- a bitmap can hold both bid and ask orders.
    // In: LegMarker is used for traversal, but in this case we need it only for
    // representing position.
    //
    // Possible solutions
    // 1. Declare new unsided coordinate types
    // 2. Separate loops for the same bitmap. But this will cause the bitmap to be
    // read twice
    // 3. Refactor the generic system. Turn side In into a variable? No, this will
    // break the matching math.
    //
    // Remove In: LegMarker from coordinates. Pass In: LegMarker as a function
    // generic when obtaining the linear iterators.
    for _ in 0..market_header.outer_bitmap_count {
        let outer_bitmap_header = OuterBitmapHeader::try_decode(ctx)?;

        for _ in 0..outer_bitmap_header.inner_bitmap_count {
            let inner_bitmap_header = InnerBitmapHeader::try_decode(ctx)?;

            for _ in 0..inner_bitmap_header.update_count {}
        }
    }

    // Reset local delta for reuse
    delta.local.deposits.reset::<B, Q>();

    Ok(())
}
