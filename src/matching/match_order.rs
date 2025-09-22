use crate::{
    goblin_error::GoblinError,
    markets::IndexedMarket,
    matching::quote_iterator::RestingOrderPositionIterator,
    quantities::{QuantityOps, Ticks},
    require,
    settlement::{MakerSideDelta, MarketMakerDeltas},
    state::{MarketState, RestingOrder, RestingOrderKey, SlotState},
    types::{Address, LegMarker},
};

pub struct SenderSideDelta<In: LegMarker> {
    pub maker_side_delta: MakerSideDelta<In>,
    pub released_by_self_trade: <In::Opposite as LegMarker>::MatchingLots,
}

impl<In: LegMarker> Default for SenderSideDelta<In> {
    fn default() -> Self {
        Self {
            maker_side_delta: MakerSideDelta::default(),
            released_by_self_trade: <In::Opposite as LegMarker>::MatchingLots::default(),
        }
    }
}

pub fn match_order<In: LegMarker>(
    pending_maker_updates: &mut MarketMakerDeltas,
    taker: &Address,
    indexed_market: &IndexedMarket,
    market_state: &mut MarketState,
    num_lots: In::Lots,
    min_lots_to_fill: In::Lots,
    price_limit: Ticks,
) -> Result<SenderSideDelta<In>, GoblinError> {
    let budget = In::matching_lots_taker(num_lots, indexed_market.base.lot_size);

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
    let best_opposite_price = In::Opposite::best_market_price_mut(market_state);
    if In::Opposite::closer_to_centre(price_limit, *best_opposite_price) {
        require!(
            min_lots_to_fill == In::Lots::ZERO,
            GoblinError::TakerPriceLimitReached
        );

        return Ok(SenderSideDelta::default());
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
    let match_result = SenderSideDelta::<In> {
        maker_side_delta: MakerSideDelta {
            free_matching_lots_in: matched,
            locked_matching_lots_out: matched_opposite,
        },
        released_by_self_trade,
    };

    Ok(match_result)
}

// pub fn match_order_old<S: SideMarker>(
//     pending_maker_updates: &mut PendingMakerUpdates,
//     taker: &Address,
//     indexed_market: &IndexedMarket,
//     market_state: &mut MarketState,
//     num_lots: <S::InputLeg as LegMarker>::Lots,
//     min_lots_to_fill: <S::InputLeg as LegMarker>::Lots,
//     price_limit: Ticks,
// ) -> Result<MatchResult<S>, GoblinError> {
//     let budget = S::get_matching_lots(num_lots, indexed_market.base.lot_size);

//     let mut remaining_budget = budget;
//     let mut matched_opposite = <S::Opposite as SideMarker>::MatchingLots::from(0);
//     let mut released = <S::Opposite as SideMarker>::MatchingLots::from(0);

//     // Halt early if best price is further from the centre than the price limit
//     let best_opposite_price = S::Opposite::best_price_mut(market_state);
//     if S::Opposite::closer_to_centre(*best_opposite_price, price_limit) {
//         require!(
//             min_lots_to_fill == S::Lots::from(0),
//             GoblinError::TakerPriceLimitReached
//         );

//         return Ok(MatchResult::default());
//     }

//     let mut quote_iterator = RestingOrderPositionIterator::<S::Opposite>::new(best_opposite_price);

//     while let Some(resting_order_position) = quote_iterator.next() {
//         let price = resting_order_position.price();
//         if S::Opposite::closer_to_centre(price, price_limit) {
//             break;
//         }

//         // Read resting order slot now
//         let resting_order_key = RestingOrderKey::new(
//             &resting_order_position.inner_bitmap_key,
//             resting_order_position.inner_index,
//         );
//         let mut resting_order = RestingOrder::load(&resting_order_key);
//         let RestingOrder {
//             trader: maker,
//             size: resting_order_size,
//         } = *resting_order.as_ref();

//         let quote =
//             S::get_quote_from_base_lots(resting_order_size, indexed_market.tick_size, price);
//         let quote_opposite = S::Opposite::get_quote_from_base_lots(
//             resting_order_size,
//             indexed_market.tick_size,
//             price,
//         );

//         // Self trade- close the resting order and mark lots for release
//         if maker == *taker {
//             released += quote_opposite;
//             continue;
//         }

//         // Entire quote consumed but budget remains. Iterate to clear the last order and read the next one.
//         // Equal to case- we need to call next() to clear the last order.
//         if remaining_budget > quote {
//             remaining_budget -= quote;
//             matched_opposite += quote_opposite;

//             let lots = S::get_lots_from_quote(quote, indexed_market.base_lot_size);
//             let lots_opposite =
//                 S::Opposite::get_lots_from_quote(quote_opposite, indexed_market.base_lot_size);

//             let pending_maker_update_mut = pending_maker_updates
//                 .get_or_insert_mut(maker)
//                 .ok_or(GoblinError::MakerListFull)?;

//             pending_maker_update_mut.accumulate_match_result::<S>(lots, lots_opposite)?;
//         } else {
//             if remaining_budget == quote {
//                 // Sub case where both budget and resting order are exhausted
//                 // Call next() to clear this resting order
//                 quote_iterator.next();
//             } else {
//                 // Order partly remains, write surplus back to slot
//                 let surplus = quote - remaining_budget;
//                 let surplus_base_lots =
//                     S::get_base_lots_from_quote(surplus, indexed_market.tick_size, price);

//                 (*resting_order.as_mut()).size = surplus_base_lots;
//                 resting_order.as_mut().store(&resting_order_key);
//             }
//             // Budget exhausted but quote remains. Stop matching.
//             let matched_quote = remaining_budget;
//             let matched_quote_opposite =
//                 S::get_opposite_quote(matched_quote, indexed_market.tick_size, price);

//             remaining_budget = S::MatchingLots::from(0);
//             matched_opposite += matched_quote_opposite;

//             break;
//         }
//     }

//     let consumed_budget = budget - remaining_budget;
//     let matched_lots = S::get_lots_from_quote(consumed_budget, indexed_market.base_lot_size);
//     require!(
//         matched_lots >= min_lots_to_fill,
//         GoblinError::InsufficientTakerFill
//     );
//     let matched_lots_opposite =
//         S::Opposite::get_lots_from_quote(matched_opposite, indexed_market.base_lot_size);

//     let released_by_self_trade =
//         S::Opposite::get_lots_from_quote(released, indexed_market.base_lot_size);

//     Ok(MatchResult {
//         pending_update: MakerUpdateSide {
//             locked_lots_out: matched_lots_opposite,
//             free_lots_in: matched_lots,
//         },
//         released_by_self_trade,
//     })
// }

// pub fn match_order<S: SideMarker>(
//     indexed_market: &IndexedMarket,
//     market_state: &mut MarketState,
//     num_lots: S::Lots,
//     min_lots_to_fill: S::Lots,
//     price_limit: Ticks,
// ) -> Result<MatchResult<S>, GoblinError> {
//     let budget = S::get_budget(num_lots, indexed_market.base_lot_size);

//     let mut matched = S::MatchingLots::from(0);
//     let mut matched_opposite = <S::Opposite as SideMarker>::MatchingLots::from(0);

//     // Halt early if best price is further from the centre than the price limit
//     let best_opposite_price = S::Opposite::best_price_mut(market_state);
//     if S::Opposite::closer_to_centre(*best_opposite_price, price_limit) {
//         require!(
//             min_lots_to_fill == S::Lots::from(0),
//             GoblinError::TakerPriceLimitReached
//         );

//         return Ok(MatchResult {
//             lots_in: S::Lots::from(0),
//             lots_out: <S::Opposite as SideMarker>::Lots::from(0),
//         });
//     }

//     let mut quote_iterator = RestingOrderPositionIterator::<S::Opposite>::new(best_opposite_price);

//     while let Some(resting_order_position) = quote_iterator.next() {
//         let price = resting_order_position.price();
//         if S::Opposite::closer_to_centre(price, price_limit) {
//             break;
//         }

//         // Read resting order slot now
//         let resting_order_key = RestingOrderKey::new(
//             &resting_order_position.inner_bitmap_key,
//             resting_order_position.inner_index,
//         );
//         let mut resting_order = RestingOrder::load(&resting_order_key);
//         let resting_order_size = resting_order.as_ref().size;

//         let quote =
//             S::get_quote_from_base_lots(resting_order_size, indexed_market.tick_size, price);

//         // Stop when budget is completely matched but resting order remains
//         //
//         //         @
//         //         @ surplus
//         //   quote _______
//         //         @     $
//         //         @     $  contribution
//         //         @     $
//         //         _______ budget
//         // matched *     $
//         //         *     $
//         if (matched + quote) > budget {
//             let surplus = (matched + quote) - budget;
//             let contribution = budget - matched;

//             matched = budget; // equivalent to matched += contribution
//             matched_opposite +=
//                 S::get_opposite_quote(contribution, indexed_market.tick_size, price);

//             // Write the quote surplus back to the resting order
//             let surplus_base_lots =
//                 S::get_base_lots_from_quote(surplus, indexed_market.tick_size, price);

//             (*resting_order.as_mut()).size = surplus_base_lots;
//             resting_order.as_mut().store(&resting_order_key);

//             // Read and update trader state of resting order owner
//             // If it is a self trade, no need to read state now. Just update delta

//             break;
//         }

//         // order is fully consumed
//         // The next iteration will deactivate the order
//         matched += quote;
//         matched_opposite += S::Opposite::get_quote_from_base_lots(
//             resting_order_size,
//             indexed_market.tick_size,
//             price,
//         );

//         // TODO we still need to update trader state
//     }

//     let matched_lots = S::get_lots_from_quote(matched, indexed_market.base_lot_size);
//     require!(
//         matched_lots >= min_lots_to_fill,
//         GoblinError::InsufficientTakerFill
//     );
//     let matched_lots_opposite =
//         S::Opposite::get_lots_from_quote(matched_opposite, indexed_market.base_lot_size);

//     // Update deltas
//     Ok(MatchResult {
//         lots_in: matched_lots,
//         lots_out: matched_lots_opposite,
//     })
// }
