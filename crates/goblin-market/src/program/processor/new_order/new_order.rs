use alloc::vec::Vec;
use stylus_sdk::alloy_primitives::{Address, FixedBytes};

use crate::{
    parameters::{BASE_TOKEN, QUOTE_TOKEN},
    program::{
        maybe_invoke_deposit, maybe_invoke_withdraw, GoblinError, GoblinResult, NewOrderError,
        PricesNotInOrder,
    },
    quantities::{BaseAtomsRaw, BaseLots, QuoteAtomsRaw, QuoteLots, Ticks, WrapperU64, MAX_TICK},
    require,
    state::{
        insert::resting_order_inserter::RestingOrderInserter,
        order::{order_id::OrderId, resting_order::SlotRestingOrder},
        MarketState, OrderPacket, OrderPacketMetadata, Side, SlotActions, SlotStorage, TraderState,
    },
    GoblinMarket,
};

use super::{place_order_inner, CondensedOrder};

#[derive(Clone, Copy)]
pub struct OrderToInsert {
    pub order_id: OrderId,
    pub resting_order: SlotRestingOrder,
}

/// Place multiple post-only orders. Used for market making
///
/// # Arguments
///
/// * `bids`
/// * `asks`
/// * `trader` - Place orders for this wallet
/// * `fail_on_cross` - Whether to fail on cross or whether to amend to amend the price.
/// * `skip_on_insufficient_funds` - Whether to skip orders with insufficient funds,
/// or whether to revert the whole TX.
/// * `tick_offset` - Adjust the price by given number of ticks if there are no slots available
/// at current price. The entire TX fails if a single resting order can't be offsetted.
/// * `client_order_id` - ID provided by trader to uniquely identify this order. It is only emitted
/// in the event and has no impact on trades. Pass 0 as the default value.
/// * `use_free_funds` - Whether to only use free funds, or transfer new tokens in to place these orders
///
pub fn place_multiple_new_orders(
    context: &mut GoblinMarket,
    bids: Vec<FixedBytes<21>>,
    asks: Vec<FixedBytes<21>>,
    trader: Address,
    fail_on_cross: bool,
    skip_on_insufficient_funds: bool,
    tick_offset: u8,
    client_order_id: u128,
    no_deposit: bool,
) -> GoblinResult<()> {
    let slot_storage = &mut SlotStorage::new();

    // Read states
    let mut market_state = MarketState::read_from_slot(slot_storage);
    let mut trader_state = TraderState::read_from_slot(slot_storage, trader);

    let mut quote_lots_to_deposit = QuoteLots::ZERO;
    let mut base_lots_to_deposit = BaseLots::ZERO;

    // Read quote and base lots available with trader
    // Lazy load ERC20 balances
    let mut base_lots_available = trader_state.base_lots_free;
    let mut quote_lots_available = trader_state.quote_lots_free;
    let mut base_allowance_read = false;
    let mut quote_allowance_read = false;

    // The last placed order. Used to
    // - ensure orders are sorted
    // - optimize check_for_cross() using sorted property
    // - find the best available order ID for the current order
    // - merge with the current order, if they have the same order ID and expiry params
    let mut last_order: Option<OrderToInsert> = None;

    // orders at centre of the book are placed first, then move away.
    // bids- descending order
    // asks- ascending order
    for (book_orders, side, outer_index_count) in [
        (&bids, Side::Bid, market_state.bids_outer_indices),
        (&asks, Side::Ask, market_state.asks_outer_indices),
    ]
    .iter()
    {
        let mut resting_order_inserter = RestingOrderInserter::new(*side, *outer_index_count);

        for order_bytes in *book_orders {
            let condensed_order = CondensedOrder::from(order_bytes);

            // Ensure orders are in correct order, i.e. moving away from the centre.
            // Descending for bids and ascending for asks.
            //
            // Orders with same price are allowed.
            // Orders with the same price and expiry parameters are combined
            if *side == Side::Bid {
                if let Some(last_order) = last_order {
                    require!(
                        condensed_order.price_in_ticks <= last_order.order_id.price_in_ticks,
                        GoblinError::PricesNotInOrder(PricesNotInOrder {})
                    );
                }
            } else {
                if let Some(last_order) = last_order {
                    require!(
                        condensed_order.price_in_ticks >= last_order.order_id.price_in_ticks,
                        GoblinError::PricesNotInOrder(PricesNotInOrder {})
                    );
                }
                // Price can't exceed max
                require!(
                    condensed_order.price_in_ticks <= Ticks::new(MAX_TICK),
                    GoblinError::PricesNotInOrder(PricesNotInOrder {})
                );
            }

            let mut order_packet = OrderPacket::PostOnly {
                side: *side,
                price_in_ticks: condensed_order.price_in_ticks,
                num_base_lots: condensed_order.size_in_base_lots,
                client_order_id,
                fail_on_cross,
                use_only_deposited_funds: no_deposit,
                track_block: condensed_order.track_block,
                last_valid_block_or_unix_timestamp_in_seconds: condensed_order
                    .last_valid_block_or_unix_timestamp_in_seconds,
                fail_silently_on_insufficient_funds: skip_on_insufficient_funds,
                tick_offset,
            };

            let matching_engine_response = {
                if order_packet.fail_silently_on_insufficient_funds()
                    && !order_packet.has_sufficient_funds(
                        context,
                        trader,
                        &mut base_lots_available,
                        &mut quote_lots_available,
                        &mut base_allowance_read,
                        &mut quote_allowance_read,
                    )
                {
                    // Skip this order if the trader does not have sufficient funds
                    continue;
                }

                // matching_engine_response gives the number of tokens required
                // these are added and then compared in the end
                let (order_to_insert, matching_engine_response) = place_order_inner(
                    // order_inserter.index_list_iterator.slot_storage,
                    slot_storage,
                    &mut market_state,
                    &mut trader_state,
                    trader,
                    &mut order_packet,
                    last_order,
                )
                .ok_or(GoblinError::NewOrderError(NewOrderError {}))?;

                if let Some(ref mut last_order) = last_order {
                    let new_order = order_to_insert.unwrap();

                    if last_order.order_id == new_order.order_id {
                        // Combine resting orders
                        last_order
                            .resting_order
                            .merge_order(&new_order.resting_order);
                    } else {
                        // Write the old order to slot and cache the new order
                        resting_order_inserter.insert_resting_order(
                            slot_storage,
                            &mut market_state,
                            &last_order.resting_order,
                            &last_order.order_id,
                        )?;

                        *last_order = new_order;
                    }
                } else {
                    last_order = order_to_insert;
                }

                matching_engine_response
            };

            let quote_lots_deposited =
                matching_engine_response.get_deposit_amount_bid_in_quote_lots();
            let base_lots_deposited =
                matching_engine_response.get_deposit_amount_ask_in_base_lots();

            if skip_on_insufficient_funds {
                // Decrement the available funds by the amount that was deposited after each iteration
                // This should never underflow, but if it does, the program will panic and the transaction will fail
                quote_lots_available -=
                    quote_lots_deposited + matching_engine_response.num_free_quote_lots_used;
                base_lots_available -=
                    base_lots_deposited + matching_engine_response.num_free_base_lots_used;
            }

            quote_lots_to_deposit += quote_lots_deposited;
            base_lots_to_deposit += base_lots_deposited;
        }

        // Write the last order after the loop ends
        if let Some(last_order_value) = last_order {
            resting_order_inserter.insert_resting_order(
                slot_storage,
                &mut market_state,
                &last_order_value.resting_order,
                &last_order_value.order_id,
            )?;
            // Clear the value. The bid should not be used in the asks loop.
            last_order = None;
        }

        // Write cached outer indices to slot
        resting_order_inserter.write_prepared_indices(slot_storage, &mut market_state);
    }

    // Write state
    // TODO check other writes in check_for_cross()
    market_state.write_to_slot(slot_storage)?;
    trader_state.write_to_slot(slot_storage, trader);
    SlotStorage::storage_flush_cache(true);

    if !no_deposit {
        maybe_invoke_deposit(
            context,
            QuoteAtomsRaw::from_lots(quote_lots_to_deposit).as_u256(),
            QUOTE_TOKEN,
            trader,
        )?;
        maybe_invoke_deposit(
            context,
            BaseAtomsRaw::from_lots(base_lots_to_deposit).as_u256(),
            BASE_TOKEN,
            trader,
        )?;
    }
    // base_lots_to_deposit and quote_lots_to_deposit are guaranteed to be 0 in
    // no deposit case. place_order_inner() checks for verify_no_deposit()

    Ok(())
}

/// Process a single, IOC, Post-only or limit order for both deposit and no-deposit cases
///
/// # Arguments
///
/// * `context` - GoblinMarket context for token XSS
/// * `order_packet`
/// * `trader`
///
pub fn process_new_order(
    context: &mut GoblinMarket,
    order_packet: &mut OrderPacket,
    trader: Address,
) -> GoblinResult<()> {
    let slot_storage = &mut SlotStorage::new();
    let mut market_state = MarketState::read_from_slot(slot_storage);
    let mut trader_state = TraderState::read_from_slot(slot_storage, trader);

    let side = order_packet.side();

    let (
        quote_atoms_to_withdraw,
        quote_atoms_to_deposit,
        base_atoms_to_withdraw,
        base_atoms_to_deposit,
    ) = {
        // If the order should fail silently on insufficient funds, and the trader does not have
        // sufficient funds for the order, return silently without modifying the book.
        if order_packet.fail_silently_on_insufficient_funds() {
            let mut base_lots_available = trader_state.base_lots_free;
            let mut quote_lots_available = trader_state.quote_lots_free;

            if !order_packet.has_sufficient_funds(
                context,
                trader,
                &mut base_lots_available,
                &mut quote_lots_available,
                &mut false,
                &mut false,
            ) {
                return Ok(());
            }
        }

        let (order_to_insert, matching_engine_response) = place_order_inner(
            slot_storage,
            &mut market_state,
            &mut trader_state,
            trader,
            order_packet,
            None,
        )
        .ok_or(GoblinError::NewOrderError(NewOrderError {}))?;

        if let Some(OrderToInsert {
            order_id,
            resting_order,
        }) = order_to_insert
        {
            let mut resting_order_inserter =
                RestingOrderInserter::new(side, market_state.outer_index_count(side));
            resting_order_inserter.insert_resting_order(
                slot_storage,
                &mut market_state,
                &resting_order,
                &order_id,
            )?;
            resting_order_inserter.write_prepared_indices(slot_storage, &mut market_state);
        }

        (
            QuoteAtomsRaw::from_lots(matching_engine_response.num_quote_lots_out),
            QuoteAtomsRaw::from_lots(
                matching_engine_response.get_deposit_amount_bid_in_quote_lots(),
            ),
            BaseAtomsRaw::from_lots(matching_engine_response.num_base_lots_out),
            BaseAtomsRaw::from_lots(matching_engine_response.get_deposit_amount_ask_in_base_lots()),
        )
    };

    // Write state
    market_state.write_to_slot(slot_storage)?;
    trader_state.write_to_slot(slot_storage, trader);
    SlotStorage::storage_flush_cache(true);

    if !order_packet.no_deposit_or_withdrawal() {
        match side {
            Side::Bid => {
                // Bid (buy)- deposit quote token, withdraw base token
                maybe_invoke_withdraw(
                    context,
                    base_atoms_to_withdraw.as_u256(),
                    BASE_TOKEN,
                    trader,
                )?;
                maybe_invoke_deposit(
                    context,
                    quote_atoms_to_deposit.as_u256(),
                    QUOTE_TOKEN,
                    trader,
                )?;
            }
            Side::Ask => {
                // Ask (sell)- deposit base token, withdraw quote token
                maybe_invoke_withdraw(
                    context,
                    quote_atoms_to_withdraw.as_u256(),
                    QUOTE_TOKEN,
                    trader,
                )?;
                maybe_invoke_deposit(context, base_atoms_to_deposit.as_u256(), BASE_TOKEN, trader)?;
            }
        }
    }

    Ok(())
}
