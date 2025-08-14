use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder},
    quantities::Ticks,
    require,
    types::SideMarker,
};

/// Instructions for a limit order. Limit orders are also known as market orders or immediate or cancel (IOC).
///
/// Fill or Kill (FoK) is a special case of limit orders where the entire amount must be filled
/// otherwise the order gets cancelled, i.e.
///
/// num_lots == min_lots_to_fill
pub struct TakePacket<S: SideMarker> {
    /// The order size, i.e. number of lots to fill
    pub num_lots: S::Lots,

    /// The minimum number of base lots to fill, otherwise the order will be invalidated.
    pub min_lots_to_fill: S::Lots,

    /// The worst price to be matched against. Stop matching after this price is crossed.
    pub price_limit: Ticks,
}

// For Bids, AdjustedQuoteLots = QuoteLots * BaseLotsPerBaseUnit
// If we read AdjustedQuoteLots from args then modulo BaseLotsPerBaseUnit should be 0
// We need to read base_lot_size from market key
//
// So bids case needs one more variable. We have to add another lambda function for validation.
// Ideally validation should happen in this struct itself, not externally.
//
// Options
// * Get rid of lambdas. Pass base_lot_size and tick_size as args. Pass base_lot_size to TakePacket::decode() to validate
// AdjustedQuoteLots.
// * Accept QuoteLots as input instead of AdjustedQuoteLots. This will remove the need to test validity.
// - All QuoteLot inputs are valid but all AdjustedQuoteLot inputs are not. We are allowing invalid states to be entered
// although we reject errors. Important- we also need to check if minimum_limit is valid, which eliminates any savings.
// DECISION- accept QuoteLots as input
impl<S: SideMarker> TakePacket<S>
where
    S::Lots: From<u64>,
    S::Lots: PartialOrd,
{
    pub fn decode(
        payload: &ArgsBuffer,
        len: usize,
        offset: &mut usize,
    ) -> Result<Self, GoblinError> {
        let byte = payload.decode::<u64>(offset, len)?;
        let read_min_lots_to_fill = byte & 0b01 != 0;
        let read_price_limit = byte & 0b10 != 0;

        let num_lots = S::Lots::from(byte >> 2);

        let min_lots_to_fill = S::Lots::from(match read_min_lots_to_fill {
            true => payload.decode::<u64>(offset, len)?,
            false => 0,
        });

        let price_limit = match read_price_limit {
            true => Ticks(payload.decode::<u32>(offset, len)?),
            false => S::DEFAULT_PRICE_LIMIT,
        };

        require!(
            num_lots > S::Lots::from(0) && S::price_limit_valid(price_limit),
            GoblinError::InvalidTakeArgs
        );

        Ok(Self {
            num_lots,
            min_lots_to_fill,
            price_limit,
        })
    }
}
