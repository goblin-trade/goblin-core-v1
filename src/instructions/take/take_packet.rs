use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder},
    quantities::Ticks,
    require,
    types::LegMarker,
};

/// Instructions for a limit order. Limit orders are also known as market orders or immediate or cancel (IOC).
///
/// Fill or Kill (FoK) is a special case of limit orders where the entire amount must be filled
/// otherwise the order gets cancelled, i.e.
///
/// num_lots == min_lots_to_fill
pub struct TakePacket<In: LegMarker> {
    /// The order size, i.e. number of lots to fill
    pub num_lots: In::Lots,

    /// The minimum number of base lots to fill, otherwise the order will be invalidated.
    pub min_lots_to_fill: In::Lots,

    /// The worst price to be matched against. Stop matching after this price is crossed.
    pub price_limit: Ticks,
}

impl<In: LegMarker> TakePacket<In> {
    pub fn decode(
        payload: &ArgsBuffer,
        len: usize,
        offset: &mut usize,
    ) -> Result<Self, GoblinError> {
        let byte = payload.decode::<u64>(offset, len)?;
        let read_min_lots_to_fill = byte & 0b01 != 0;
        let read_price_limit = byte & 0b10 != 0;

        let num_lots = In::Lots::from(byte >> 2);

        let min_lots_to_fill = In::Lots::from(match read_min_lots_to_fill {
            true => payload.decode::<u64>(offset, len)?,
            false => 0,
        });

        let price_limit = match read_price_limit {
            true => Ticks::new(payload.decode::<u32>(offset, len)? as u64),
            false => In::DEFAULT_PRICE_LIMIT,
        };

        require!(
            num_lots > In::Lots::from(0) && In::price_limit_valid(price_limit),
            GoblinError::InvalidTakeArgs
        );

        Ok(Self {
            num_lots,
            min_lots_to_fill,
            price_limit,
        })
    }
}
