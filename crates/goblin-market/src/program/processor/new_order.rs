use alloc::vec::Vec;
use stylus_sdk::alloy_primitives::{Address, FixedBytes, B256};

use crate::{
    parameters::{BASE_TOKEN, QUOTE_TOKEN},
    program::{
        maybe_invoke_deposit, maybe_invoke_withdraw, GoblinError, GoblinResult, NewOrderError,
        PricesNotInOrder,
    },
    quantities::{BaseAtomsRaw, BaseLots, QuoteAtomsRaw, QuoteLots, Ticks, WrapperU64, MAX_TICK},
    require,
    state::{
        MarketState, MatchingEngine, MatchingEngineResponse, OrderId, OrderPacket,
        OrderPacketMetadata, Side, SlotActions, SlotRestingOrder, SlotStorage, TraderState,
    },
    GoblinMarket,
};

#[derive(Clone, Copy)]
pub struct OrderToInsert {
    pub order_id: OrderId,
    pub resting_order: SlotRestingOrder,
}

pub enum FailedMultipleLimitOrderBehavior {
    /// Orders will never cross the spread. Instead they will be amended to the closest non-crossing price.
    /// The entire transaction will fail if matching engine returns None for any order, which indicates an error.
    ///
    /// If an order has insufficient funds, the entire transaction will fail.
    FailOnInsufficientFundsAndAmendOnCross = 0,

    /// If any order crosses the spread or has insufficient funds, the entire transaction will fail.
    FailOnInsufficientFundsAndFailOnCross = 1,

    /// Orders will be skipped if the user has insufficient funds.
    /// Crossing orders will be amended to the closest non-crossing price.
    SkipOnInsufficientFundsAndAmendOnCross = 2,

    /// Orders will be skipped if the user has insufficient funds.
    /// If any order crosses the spread, the entire transaction will fail.
    SkipOnInsufficientFundsAndFailOnCross = 3,
}

impl From<u8> for FailedMultipleLimitOrderBehavior {
    fn from(value: u8) -> Self {
        match value {
            0 => FailedMultipleLimitOrderBehavior::FailOnInsufficientFundsAndAmendOnCross,
            1 => FailedMultipleLimitOrderBehavior::FailOnInsufficientFundsAndFailOnCross,
            2 => FailedMultipleLimitOrderBehavior::SkipOnInsufficientFundsAndAmendOnCross,
            3 => FailedMultipleLimitOrderBehavior::SkipOnInsufficientFundsAndFailOnCross,
            _ => panic!(
                "Invalid value for  FailedMultipleLimitOrderBehavior: {}",
                value
            ),
        }
    }
}

impl FailedMultipleLimitOrderBehavior {
    pub fn should_fail_on_cross(&self) -> bool {
        matches!(
            self,
            FailedMultipleLimitOrderBehavior::FailOnInsufficientFundsAndFailOnCross
                | FailedMultipleLimitOrderBehavior::SkipOnInsufficientFundsAndFailOnCross
        )
    }

    pub fn should_skip_orders_with_insufficient_funds(&self) -> bool {
        matches!(
            self,
            FailedMultipleLimitOrderBehavior::SkipOnInsufficientFundsAndAmendOnCross
                | FailedMultipleLimitOrderBehavior::SkipOnInsufficientFundsAndFailOnCross
        )
    }
}

pub struct CondensedOrder {
    // Order price in ticks
    pub price_in_ticks: Ticks,

    // Order size
    pub size_in_base_lots: BaseLots,

    // Whether to track block or unix timestamp
    pub track_block: bool,

    // The last valid block or unix timestamp, depending on the value of
    // track_block. Set value as 0 to disable FOK.
    pub last_valid_block_or_unix_timestamp_in_seconds: u32,
}

impl From<&FixedBytes<21>> for CondensedOrder {
    fn from(bytes: &FixedBytes<21>) -> Self {
        CondensedOrder {
            price_in_ticks: Ticks::new(u64::from_be_bytes(bytes[0..8].try_into().unwrap())),
            size_in_base_lots: BaseLots::new(u64::from_be_bytes(bytes[8..16].try_into().unwrap())),
            track_block: (bytes[16] & 0b0000_0001) != 0,
            last_valid_block_or_unix_timestamp_in_seconds: u32::from_be_bytes(
                bytes[17..21].try_into().unwrap(),
            ),
        }
    }
}

pub fn place_multiple_new_orders(
    context: &mut GoblinMarket,
    trader: Address,
    to: Address,
    failed_multiple_limit_order_behavior: FailedMultipleLimitOrderBehavior,
    bids: Vec<FixedBytes<21>>,
    asks: Vec<FixedBytes<21>>,
    client_order_id: u128,
    no_deposit: bool,
    tick_offset: u8,
) -> GoblinResult<()> {
    let slot_storage = &mut SlotStorage::new();

    // Read states
    let mut market_state = MarketState::read_from_slot(slot_storage);
    let mut trader_state = TraderState::read_from_slot(slot_storage, trader);

    let mut matching_engine = MatchingEngine { slot_storage };

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
    for (book_orders, side) in [(&bids, Side::Bid), (&asks, Side::Ask)].iter() {
        for order_bytes in *book_orders {
            let condensed_order = CondensedOrder::from(order_bytes);

            // Ensure orders are in correct order- descending for bids and ascending for asks
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
                reject_post_only: failed_multiple_limit_order_behavior.should_fail_on_cross(),
                use_only_deposited_funds: no_deposit,
                track_block: condensed_order.track_block,
                last_valid_block_or_unix_timestamp_in_seconds: condensed_order
                    .last_valid_block_or_unix_timestamp_in_seconds,
                fail_silently_on_insufficient_funds: failed_multiple_limit_order_behavior
                    .should_skip_orders_with_insufficient_funds(),
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
                let (order_to_insert, matching_engine_response) = matching_engine
                    .place_order_inner(
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
                        write_order(*last_order);
                        *last_order = new_order;
                    }
                } else {
                    last_order = order_to_insert;
                }

                matching_engine_response
            };

            // Write the last order after the loop ends
            if let Some(last_order_value) = last_order {
                write_order(last_order_value);
                // Clear the value. The bid should not be used in the asks loop.
                last_order = None;
            }

            let quote_lots_deposited =
                matching_engine_response.get_deposit_amount_bid_in_quote_lots();
            let base_lots_deposited =
                matching_engine_response.get_deposit_amount_ask_in_base_lots();

            if failed_multiple_limit_order_behavior.should_skip_orders_with_insufficient_funds() {
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
    }

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

fn write_order(order: OrderToInsert) {
    todo!()
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
        let mut matching_engine = MatchingEngine { slot_storage };

        let (order_to_insert, matching_engine_response) = matching_engine
            .place_order_inner(
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
            matching_engine.insert_order_in_book(
                &mut market_state,
                &resting_order,
                side,
                &order_id,
            )?;
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
