use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder},
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
    /// The order size, i.e. number of lots to fill
    pub num_lots: u64,

    // Optional control parameters
    /// The minimum number of base lots to fill, otherwise the order will be invalidated.
    pub min_lots_to_fill: u64,

    /// The worst price to be matched against. Stop matching after this price is crossed.
    pub price_limit: Ticks,
}

impl TakePacket {
    pub fn decode(
        side: Side,
        payload: &ArgsBuffer,
        len: usize,
        offset: &mut usize,
    ) -> Result<Self, GoblinError> {
        // Bits 0 and 1 hold flags. Rest of the 62 bits hold order size
        let byte = payload.decode::<u64>(offset, len)?;
        let read_min_lots_to_fill = byte & 0b01 != 0;
        let read_price_limit = byte & 0b10 != 0;
        let num_lots = byte >> 2;

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
            num_lots,
            min_lots_to_fill,
            price_limit,
        })
    }
}
