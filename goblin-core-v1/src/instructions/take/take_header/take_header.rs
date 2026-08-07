use crate::{
    axis::leg::leg_matcher::LegMatcher,
    goblin_error::GoblinError,
    input_processor::{DecodeCtx, FixedDecode, VariableDecode},
    instructions::{TakeHeaderMain, TakeHeaderOptional},
    quantities::Position,
};

/// Instructions for a limit order. Limit orders are also known as market orders or immediate or cancel (IOC).
///
/// Fill or Kill (FoK) is a special case of limit orders where the entire amount must be filled
/// otherwise the order gets cancelled, i.e.
///
/// num_lots == min_lots_to_fill
pub struct TakeHeader<In: LegMatcher> {
    /// The order size, i.e. number of lots to fill
    pub num_lots: In::Lots,

    /// The minimum number of base lots to fill, otherwise the order will be invalidated.
    pub min_lots_to_fill: In::Lots,

    /// The worst position to be matched against. Stop matching after this price is crossed.
    pub limit: Position,
}

impl<In: LegMatcher> TakeHeader<In> {
    pub fn try_new(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        let main_header = TakeHeaderMain::<In>::try_fixed_decode(ctx)?;
        let optional_header =
            TakeHeaderOptional::<In>::raw_variable_decode(ctx, &main_header.flags);

        Ok(Self {
            num_lots: main_header.num_lots,
            min_lots_to_fill: optional_header.min_lots_to_fill,
            limit: optional_header.limit,
        })
    }
}
