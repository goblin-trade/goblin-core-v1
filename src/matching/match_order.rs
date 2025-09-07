use crate::{
    goblin_error::GoblinError,
    markets::IndexedMarket,
    matching::quote_iterator::RestingOrderPositionIterator,
    quantities::Ticks,
    require,
    settlement::{MakerUpdateSide, PendingMakerUpdates},
    state::{MarketState, RestingOrder, RestingOrderKey, SlotState},
    types::{Address, SideMarker},
};

pub struct MatchResult<S: SideMarker> {
    pub pending_update: MakerUpdateSide<S>,
    pub released_by_self_trade: <S::Opposite as SideMarker>::Lots,
}

impl<S: SideMarker> Default for MatchResult<S> {
    fn default() -> Self {
        Self {
            pending_update: MakerUpdateSide::default(),
            released_by_self_trade: <S::Opposite as SideMarker>::Lots::default(),
        }
    }
}

pub fn match_order<S: SideMarker>(
    pending_maker_updates: &mut PendingMakerUpdates,
    taker: &Address,
    indexed_market: &IndexedMarket,
    market_state: &mut MarketState,
    num_lots: S::Lots,
    min_lots_to_fill: S::Lots,
    price_limit: Ticks,
) -> Result<MatchResult<S>, GoblinError> {
    let budget = S::get_budget(num_lots, indexed_market.base_lot_size);

    let mut remaining_budget = budget;
    let mut matched_opposite = <S::Opposite as SideMarker>::MatchingLots::from(0);
    let mut released = <S::Opposite as SideMarker>::MatchingLots::from(0);

    // Halt early if best price is further from the centre than the price limit
    let best_opposite_price = S::Opposite::best_price_mut(market_state);
    if S::Opposite::closer_to_centre(*best_opposite_price, price_limit) {
        require!(
            min_lots_to_fill == S::Lots::from(0),
            GoblinError::TakerPriceLimitReached
        );

        return Ok(MatchResult::default());
    }

    let mut quote_iterator = RestingOrderPositionIterator::<S::Opposite>::new(best_opposite_price);

    while let Some(resting_order_position) = quote_iterator.next() {
        let price = resting_order_position.price();
        if S::Opposite::closer_to_centre(price, price_limit) {
            break;
        }

        // Read resting order slot now
        let resting_order_key = RestingOrderKey::new(
            &resting_order_position.inner_bitmap_key,
            resting_order_position.inner_index,
        );
        let mut resting_order = RestingOrder::load(&resting_order_key);
        let RestingOrder {
            trader: maker,
            size: resting_order_size,
        } = *resting_order.as_ref();

        let quote =
            S::get_quote_from_base_lots(resting_order_size, indexed_market.tick_size, price);
        let quote_opposite = S::Opposite::get_quote_from_base_lots(
            resting_order_size,
            indexed_market.tick_size,
            price,
        );

        // Self trade- close the resting order and mark lots for release
        if maker == *taker {
            released += quote_opposite;
            continue;
        }

        // Entire quote consumed but budget remains. Iterate to clear the last order and read the next one.
        // Equal to case- we need to call next() to clear the last order.
        if remaining_budget > quote {
            remaining_budget -= quote;
            matched_opposite += quote_opposite;

            let lots = S::get_lots_from_quote(quote, indexed_market.base_lot_size);
            let lots_opposite =
                S::Opposite::get_lots_from_quote(quote_opposite, indexed_market.base_lot_size);

            let pending_maker_update_mut = pending_maker_updates
                .get_or_insert_mut(&maker)
                .ok_or(GoblinError::DeltaListFull)?;

            pending_maker_update_mut.accumulate_match_result::<S>(lots, lots_opposite)?;
        } else {
            if remaining_budget == quote {
                // Sub case where both budget and resting order are exhausted
                // Call next() to clear this resting order
                quote_iterator.next();
            } else {
                // Order partly remains, write surplus back to slot
                let surplus = quote - remaining_budget;
                let surplus_base_lots =
                    S::get_base_lots_from_quote(surplus, indexed_market.tick_size, price);

                (*resting_order.as_mut()).size = surplus_base_lots;
                resting_order.as_mut().store(&resting_order_key);
            }
            // Budget exhausted but quote remains. Stop matching.
            let matched_quote = remaining_budget;
            let matched_quote_opposite =
                S::get_opposite_quote(matched_quote, indexed_market.tick_size, price);

            remaining_budget = S::MatchingLots::from(0);
            matched_opposite += matched_quote_opposite;

            break;
        }
    }

    let consumed_budget = budget - remaining_budget;
    let matched_lots = S::get_lots_from_quote(consumed_budget, indexed_market.base_lot_size);
    require!(
        matched_lots >= min_lots_to_fill,
        GoblinError::InsufficientTakerFill
    );
    let matched_lots_opposite =
        S::Opposite::get_lots_from_quote(matched_opposite, indexed_market.base_lot_size);

    let released_by_self_trade =
        S::Opposite::get_lots_from_quote(released, indexed_market.base_lot_size);

    Ok(MatchResult {
        pending_update: MakerUpdateSide {
            locked_lots_out: matched_lots_opposite,
            free_lots_in: matched_lots,
        },
        released_by_self_trade,
    })
}

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
