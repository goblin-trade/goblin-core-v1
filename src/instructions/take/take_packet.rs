use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder},
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
pub struct TakePacket {
    /// The market to trade on
    pub market_index: MarketIndex,

    /// The order side
    pub side: Side,

    /// The order size, i.e. number of lots to fill
    pub num_lots: u64,

    // Optional control parameters
    /// The minimum number of base lots to fill, otherwise the order will be invalidated.
    pub min_lots_to_fill: u64,

    /// The worst price to be matched against. Stop matching after this price is crossed.
    pub price_limit: Ticks,
}

/// The order side and flags telling which flags to read, compressed in 1 byte
struct TakeFlags {
    pub side: Side,
    pub read_min_lots_to_fill: bool,
    pub read_price_limit: bool,
}

impl TakeFlags {
    fn new(flags: u8) -> Self {
        Self {
            side: Side::from(flags & 0b1 == 1),
            read_min_lots_to_fill: flags & 0b10 == 1,
            read_price_limit: flags & 0b100 == 1,
        }
    }
}

impl TakePacket {
    // Fixed min size for
    // * flags: 1
    // * market_index: 1
    // * num_lots: 8
    const MIN_SIZE: usize = 1 + 1 + 8;

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
        let TakeFlags {
            side,
            read_min_lots_to_fill,
            read_price_limit,
        } = TakeFlags::new(flags);

        let market_index = MarketIndex(payload.decode_unchecked::<u8>(*offset));
        let num_lots = payload.decode_unchecked::<u64>(*offset);

        // Decode optional fields
        let min_lots_to_fill = match read_min_lots_to_fill {
            true => payload.decode::<u64>(offset, len)?,
            false => 0,
        };

        let price_limit = match read_price_limit {
            true => Ticks(payload.decode::<u32>(offset, len)?),
            false => match side {
                Side::Bid => Ticks::MAX,
                Side::Ask => Ticks::ZERO,
            },
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
        })
    }
}
