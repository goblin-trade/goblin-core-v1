use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder},
    instructions::take::self_trade_behavior::SelfTradeBehavior,
    markets::MarketIndex,
    quantities::Ticks,
    require,
    types::Side,
};

/// Instructions for a limit order. Limit orders are also known as market orders or immediate or cancel (IOC).
///
/// Fill or Kill (FoK) is a special case of limit orders where the entire amount must be filled
/// otherwise the order gets cancelled, i.e.
///
/// num_lots == min_lots_to_fill
pub struct TakeHeader {
    /// The market to trade on
    pub market_index: MarketIndex,

    /// The order side
    pub side: Side,

    /// The order size, i.e. number of lots to fill
    num_lots: u64,

    /// The minimum number of base lots to fill, otherwise the order will be invalidated.
    min_lots_to_fill: u64,

    // Optional parameters
    /// The worst price to be matched against. Stop matching after this price is crossed.
    pub price_limit: Ticks,

    /// Max number of orders to match against. Pass u8::MAX for max matching.
    pub match_limit: u8,

    /// How to handle self trades
    pub self_trade_behavior: SelfTradeBehavior,
}

/// The order side and flags telling which flags to read, compressed in 1 byte
struct TakeHeaderFlags {
    pub side: Side,
    pub read_price_limit: bool,
    pub read_match_limit: bool,
    pub read_self_trade_behavior: bool,
}

impl TakeHeaderFlags {
    fn new(flags: u8) -> Self {
        Self {
            side: Side::from(flags & 0b1 == 1),
            read_price_limit: flags & 0b10 == 1,
            read_match_limit: flags & 0b100 == 1,
            read_self_trade_behavior: flags & 0b1000 == 1,
        }
    }
}

impl TakeHeader {
    // Fixed min size for
    // * flags: 1
    // * market_index: 1
    // * num_lots: 8
    // * min_lots_to_fill: 8
    const MIN_SIZE: usize = 1 + 1 + 8 + 8;

    pub fn decode(
        payload: &ArgsBuffer,
        len: usize,
        offset: &mut usize,
    ) -> Result<Self, GoblinError> {
        crate::require!(
            len >= *offset + Self::MIN_SIZE,
            crate::goblin_error::GoblinError::InvalidPayload
        );

        // Fixed fields
        // We should perform a single size check instead of doing 3
        let flags = payload.decode_unchecked::<u8>(*offset);
        let TakeHeaderFlags {
            side,
            read_price_limit,
            read_match_limit,
            read_self_trade_behavior,
        } = TakeHeaderFlags::new(flags);

        let market_index = MarketIndex(payload.decode_unchecked::<u8>(*offset));
        let num_lots = payload.decode_unchecked::<u64>(*offset);
        let min_lots_to_fill = payload.decode_unchecked::<u64>(*offset);

        // Decode optional fields

        let price_limit = match read_price_limit {
            true => Ticks(payload.decode::<u32>(offset, len)?),
            false => match side {
                Side::Bid => Ticks::MAX,
                Side::Ask => Ticks::ZERO,
            },
        };

        let match_limit = match read_match_limit {
            true => payload.decode::<u8>(offset, len)?,
            false => u8::MAX,
        };

        let self_trade_behavior = match read_self_trade_behavior {
            true => SelfTradeBehavior::try_from(payload.decode::<u8>(offset, len)?)?,
            false => SelfTradeBehavior::CancelProvide,
        };

        // Validate
        //
        // * Lot size > 0
        // * price limit cannot be 0 for bids as it will give an undefined value.
        // We don't need to check for bids and Ticks::MAX because 2^64 - 1 is a finite value, not infinity.
        require!(
            num_lots > 0 && (side == Side::Ask || price_limit > Ticks::ZERO),
            GoblinError::InvalidTakeArgs
        );

        Ok(Self {
            market_index,
            side,
            num_lots,
            min_lots_to_fill,
            price_limit,
            match_limit,
            self_trade_behavior,
        })
    }
}
