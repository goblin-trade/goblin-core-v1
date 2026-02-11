use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base, Leg, Quote},
        market::{market_marker::MarketMarker, CommonMarket},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    matching::resting_order_iterator::RestingOrderIterator,
    quantities::{BaseLotsPerBaseUnit, QuantityOps, QuoteLotsPerQuoteUnit, Ticks},
    require,
    settlement::{
        local_delta::{LocalDelta, MakerDelta, TakerDelta},
        MatchedLots,
    },
    state::{MarketState, Preimage, RestingOrder, RestingOrderPreimage},
    types::{Address, StoreReader, Tuple},
};

/// Match a take order
///
/// We match against resting orders until one of these conditions is met
/// * The budget is consumed
/// * Price limit reached
/// * All resting orders are popped
///
pub fn match_order<M, B, Q, In>(
    taker: &Address,
    local_delta: &mut LocalDelta,
    market: &CommonMarket<M, B, Q>,
    market_state: &mut MarketState<M, B, Q>,
    num_lots: In::Lots,
    min_lots_to_fill: In::Lots,
    price_limit: Ticks,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher
        + StoreReader<Tuple<MakerDelta<Base>, MakerDelta<Quote>, Leg>, Result = MakerDelta<In>>
        + StoreReader<Tuple<TakerDelta<Base>, TakerDelta<Quote>, Leg>, Result = TakerDelta<In>>
        + StoreReader<
            Tuple<BaseLotsPerBaseUnit, QuoteLotsPerQuoteUnit, Leg>,
            Result = In::LotsPerUnit,
        >,
    In::Opposite: StoreReader<Tuple<Ticks, Ticks, Leg>, Result = Ticks>,
{
    let base_lot_size = Base::get(&market.lot_size_pair);
    let budget = In::matching_lots_taker(num_lots, base_lot_size);

    let mut taker_in = In::MatchingLots::ZERO;
    let mut taker_out = <In::Opposite as LegMatcher>::MatchingLots::ZERO;
    let mut taker_self_trade_unlocked = <In::Opposite as LegMatcher>::MatchingLots::ZERO;

    let mut resting_order_iterator = RestingOrderIterator::<In>::new(
        &mut market_state.best_prices,
        price_limit,
        min_lots_to_fill,
    )?;

    while budget > taker_in {
        if let Some(resting_order_position) = resting_order_iterator.next() {
            let price = resting_order_position.price();

            // price limit reached, stop matching
            if In::Opposite::closer_to_centre(price_limit, price) {
                break;
            }

            // Read resting order amount
            let resting_order_preimage = RestingOrderPreimage {
                inner_bitmap_key: resting_order_position.inner_bitmap_key,
                inner_index: resting_order_position.inner_index,
            };
            let resting_order_key = resting_order_preimage.hash();
            let mut resting_order = resting_order_key.load();

            let RestingOrder {
                maker,
                size: resting_order_size,
            } = resting_order;

            let quote = In::matching_lots_maker(resting_order_size, market.tick_size, price);
            let quote_opposite =
                In::Opposite::matching_lots_maker(resting_order_size, market.tick_size, price);

            // Self trade- close the resting order and mark lots for release
            if maker == *taker {
                taker_self_trade_unlocked += quote_opposite;
                continue;
            }

            if budget < taker_in + quote {
                // Resting order consumes the budget. Part of the resting order remains, write it back.
                let surplus = taker_in + quote - budget;
                let surplus_base_lots =
                    In::base_lots_from_matching(surplus, market.tick_size, price);
                resting_order.size = surplus_base_lots;
                resting_order_key.store(&resting_order);

                let consumed = budget - taker_in;
                let consumed_base_lots = resting_order_size - surplus_base_lots;
                let consumed_opposite =
                    In::Opposite::matching_lots_maker(consumed_base_lots, market.tick_size, price);

                taker_in += consumed;
                taker_out += consumed_opposite;
            } else {
                // Resting order is completely consumed. Index to the next resting order/

                // Update taker
                taker_in += quote;
                taker_out += quote_opposite;

                // Update maker
                let maker_delta_pair = local_delta
                    .local_maker_deltas
                    .get_or_insert_mut(maker)
                    .ok_or(GoblinError::MakerListFull)?;

                let maker_delta = In::get_leg_mut(maker_delta_pair);
                maker_delta
                    .matched_lots
                    .checked_add(MatchedLots {
                        taker_in: quote,
                        taker_out: quote_opposite,
                    })
                    .ok_or(GoblinError::DeltaOverflow)?;

                if budget == taker_in + quote {
                    // Call next to remove resting order from book and index to the next one,
                    // then stop matching
                    resting_order_iterator.next();
                    break;
                } else {
                    // More budget remains, continue looping
                    continue;
                }
            }
        } else {
        }
    }
    // TODO check min_lots_to_fill
    let taker_delta = In::get_leg_mut(&mut local_delta.local_sender_delta.taker_delta_pair);
    *taker_delta = TakerDelta {
        matched_lots: MatchedLots {
            taker_in,
            taker_out,
        },
        taker_self_trade_unlocked,
    };

    Ok(())
}
