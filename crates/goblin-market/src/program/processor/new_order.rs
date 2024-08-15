use alloc::vec::Vec;
use stylus_sdk::alloy_primitives::{Address, B256};

use crate::{
    parameters::{BASE_TOKEN, QUOTE_TOKEN},
    program::{
        maybe_invoke_deposit, maybe_invoke_withdraw, GoblinError, GoblinResult, NewOrderError,
        UndefinedFailedMultipleLimitOrderBehavior,
    },
    quantities::{BaseAtomsRaw, BaseLots, QuoteAtomsRaw, Ticks, WrapperU64},
    state::{
        MarketState, MatchingEngine, OrderId, OrderPacket, OrderPacketMetadata, Side, SlotActions,
        SlotRestingOrder, SlotStorage, TraderState,
    },
    GoblinMarket,
};

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

impl FailedMultipleLimitOrderBehavior {
    pub fn decode(value: u8) -> GoblinResult<FailedMultipleLimitOrderBehavior> {
        match value {
            0 => Ok(FailedMultipleLimitOrderBehavior::FailOnInsufficientFundsAndAmendOnCross),
            1 => Ok(FailedMultipleLimitOrderBehavior::FailOnInsufficientFundsAndFailOnCross),
            2 => Ok(FailedMultipleLimitOrderBehavior::SkipOnInsufficientFundsAndAmendOnCross),
            3 => Ok(FailedMultipleLimitOrderBehavior::SkipOnInsufficientFundsAndFailOnCross),
            _ => Err(GoblinError::UndefinedFailedMultipleLimitOrderBehavior(
                UndefinedFailedMultipleLimitOrderBehavior {},
            )),
        }
    }

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

    /// If price_on_ticks has no available slots, try placing the order at a less aggresive
    /// price (away from the centre) by amending the price by these many ticks.
    pub amend_x_ticks: u8,
}

impl CondensedOrder {
    pub fn decode(bytes: &[u8; 32]) -> Self {
        CondensedOrder {
            price_in_ticks: Ticks::new(u64::from_be_bytes(bytes[0..8].try_into().unwrap())),
            size_in_base_lots: BaseLots::new(u64::from_be_bytes(bytes[8..16].try_into().unwrap())),
            track_block: (bytes[16] & 0b0000_0001) != 0,
            last_valid_block_or_unix_timestamp_in_seconds: u32::from_be_bytes(
                bytes[17..21].try_into().unwrap(),
            ),
            amend_x_ticks: bytes[21],
        }
    }
}

/// Create multiple new orders
///
/// Each order request is (price in ticks, size in base lots, side)
/// The order ID is derived by reading index list and bitmaps.
/// Note- Side must be known. We don't want an order intended as a bid being placed as an ask.
///
/// Increase feature- placing order at the same price increases the order
/// But this will require us to read the RestingOrder slots one by one to
/// know the best order belonging to the trader. AVOID
/// Alternative- Cancel and place. If done atomically it will have the same or better index
pub fn process_multiple_new_orders(
    context: &mut GoblinMarket,
    to: Address,
    failed_multiple_limit_order_behavior: FailedMultipleLimitOrderBehavior,
    bids: Vec<B256>,
    asks: Vec<B256>,
    client_order_id: u128,
    use_free_funds: bool,
) -> GoblinResult<()> {
    let mut matching_engine = MatchingEngine {
        slot_storage: &mut SlotStorage::new(),
    };

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
        if order_packet.fail_silently_on_insufficient_funds()
            && !order_packet.has_sufficient_funds(context, &trader_state, trader)
        {
            return Ok(());
        }

        let mut matching_engine = MatchingEngine { slot_storage };

        let (order_to_insert, matching_engine_response) = matching_engine
            .place_order_inner(&mut market_state, &mut trader_state, trader, order_packet)
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
