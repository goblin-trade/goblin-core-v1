use crate::{
    axis::leg::{leg_matcher::LegMatcher, leg_validator::LegValidator},
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    quantities::Ticks,
    require,
};

/// Instructions for a limit order. Limit orders are also known as market orders or immediate or cancel (IOC).
///
/// Fill or Kill (FoK) is a special case of limit orders where the entire amount must be filled
/// otherwise the order gets cancelled, i.e.
///
/// num_lots == min_lots_to_fill
pub struct TakePacket<In: LegMatcher + LegValidator> {
    /// The order size, i.e. number of lots to fill
    pub num_lots: In::Lots,

    /// The minimum number of base lots to fill, otherwise the order will be invalidated.
    pub min_lots_to_fill: In::Lots,

    /// The worst price to be matched against. Stop matching after this price is crossed.
    pub price_limit: Ticks,
}

impl<In: LegMatcher + LegValidator> Decodable for TakePacket<In> {
    fn try_decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        // 2 bits for flags and rest 62 bits for num_lots
        let flags_and_num_lots_raw = u64::try_decode(ctx)?;

        let read_min_lots_to_fill = flags_and_num_lots_raw & 0b01 != 0;
        let read_price_limit = flags_and_num_lots_raw & 0b10 != 0;

        let num_lots = In::Lots::from(flags_and_num_lots_raw >> 2);

        let min_lots_to_fill = In::Lots::from(match read_min_lots_to_fill {
            true => u64::try_decode(ctx)?,
            false => 0,
        });

        let price_limit = match read_price_limit {
            true => Ticks::new(u32::try_decode(ctx)? as u64),
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
