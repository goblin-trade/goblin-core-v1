use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
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

impl<'a, In: LegMarker> Decodable<'a> for TakePacket<In> {
    fn decode(ctx: &DecodeCtx<'a>) -> Result<Self, GoblinError> {
        let byte = ctx.decode::<u64>()?;
        let read_min_lots_to_fill = byte & 0b01 != 0;
        let read_price_limit = byte & 0b10 != 0;

        let num_lots = In::Lots::from(byte >> 2);

        let min_lots_to_fill = In::Lots::from(match read_min_lots_to_fill {
            true => ctx.decode::<u64>()?,
            false => 0,
        });

        let price_limit = match read_price_limit {
            true => Ticks::new(ctx.decode::<u32>()? as u64),
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
