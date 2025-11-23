use crate::{
    goblin_error::GoblinError,
    markets::IndexedMarket,
    matching::{quote_iterator::RestingOrderPositionIterator, MatchResult},
    quantities::{QuantityOps, Ticks},
    require,
    settlement::{MakerSideDelta, MarketMakerDeltas},
    state::{MarketState, RestingOrder, RestingOrderKey, SlotState},
    types::{Address, Base, LegMarker, PairAccessor, Quote},
};

pub fn match_order<In>(
    pending_maker_updates: &mut MarketMakerDeltas,
    taker: &Address,
    indexed_market: &IndexedMarket,
    market_state: &mut MarketState,
    num_lots: In::Lots,
    min_lots_to_fill: In::Lots,
    price_limit: Ticks,
) -> Result<MatchResult<In>, GoblinError>
where
    In: LegMarker
        + PairAccessor<MakerSideDelta<Base>, MakerSideDelta<Quote>, Result = MakerSideDelta<In>>,
    In::Opposite: PairAccessor<Ticks, Ticks, Result = Ticks>,
{
    let budget = In::matching_lots_taker(num_lots, indexed_market.base_lot_size());

    // The amount matched and transferred in, i.e lost by taker and transferred to makers.
    // We keep matching until
    // - The entire budget is matched, or
    // - We reach price_limit, or
    // - We run out of resting orders
    let mut matched = In::MatchingLots::ZERO;

    // The opposite amount transferred out, i.e. lost by makers and gained by the taker.
    let mut matched_opposite = <In::Opposite as LegMarker>::MatchingLots::ZERO;

    // The opposite amount unlocked upon self trade
    let mut released_by_self_trade = <In::Opposite as LegMarker>::MatchingLots::ZERO;

    // Halt early if best price is further from the centre than the price limit
    let best_opposite_price = In::Opposite::get_leg_mut(&mut market_state.best_prices);

    if In::Opposite::closer_to_centre(price_limit, *best_opposite_price) {
        require!(
            min_lots_to_fill == In::Lots::ZERO,
            GoblinError::TakerPriceLimitReached
        );

        return Ok(MatchResult::default());
    }

    let mut resting_order_position_iterator =
        RestingOrderPositionIterator::<In::Opposite>::new(best_opposite_price);

    while budget > matched {
        let next_position = resting_order_position_iterator.next();
        match next_position {
            Some(resting_order_position) => {
                let price = resting_order_position.price();

                // price limit reached, stop matching
                if In::Opposite::closer_to_centre(price_limit, price) {
                    break;
                }

                // Read resting order amount
                let resting_order_key = RestingOrderKey::new(
                    &resting_order_position.inner_bitmap_key,
                    resting_order_position.inner_index,
                );
                let mut resting_order = RestingOrder::load(&resting_order_key);
                let RestingOrder {
                    maker,
                    size: resting_order_size,
                } = *resting_order.as_ref();

                let quote =
                    In::matching_lots_maker(resting_order_size, indexed_market.tick_size, price);
                let quote_opposite = In::Opposite::matching_lots_maker(
                    resting_order_size,
                    indexed_market.tick_size,
                    price,
                );

                // Self trade- close the resting order and mark lots for release
                if maker == *taker {
                    released_by_self_trade += quote_opposite;
                    continue;
                }

                if budget < matched + quote {
                    // Resting order consumes the budget. Part of the resting order remains, write it back.
                    let surplus = matched + quote - budget;
                    let surplus_base_lots =
                        In::base_lots_from_matching(surplus, indexed_market.tick_size, price);
                    (*resting_order.as_mut()).size = surplus_base_lots;
                    resting_order.as_mut().store(&resting_order_key);

                    let consumed = budget - matched;
                    let consumed_base_lots = resting_order_size - surplus_base_lots;
                    let consumed_opposite = In::Opposite::matching_lots_maker(
                        consumed_base_lots,
                        indexed_market.tick_size,
                        price,
                    );

                    matched += consumed;
                    matched_opposite += consumed_opposite;
                } else {
                    // Resting order is completely consumed. Index to the next resting order/

                    // Update taker
                    matched += quote;
                    matched_opposite += quote_opposite;

                    // Update maker
                    let pending_maker_update_mut = pending_maker_updates
                        .get_or_insert_mut(maker)
                        .ok_or(GoblinError::MakerListFull)?;

                    pending_maker_update_mut
                        .accumulate_match_result::<In>(quote, quote_opposite)?;

                    if budget == matched + quote {
                        // Call next to remove resting order from book and index to the next one,
                        // then stop matching
                        resting_order_position_iterator.next();
                        break;
                    } else {
                        // More budget remains, continue looping
                        continue;
                    }
                }
            }
            None => break,
        }
    }
    let match_result = MatchResult::<In> {
        maker_side_delta: MakerSideDelta {
            free_matching_lots_in: matched,
            locked_matching_lots_out: matched_opposite,
        },
        released_by_self_trade,
    };

    Ok(match_result)
}
