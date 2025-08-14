use crate::{
    goblin_error::GoblinError,
    input_processor::ArgsBuffer,
    instructions::take::take_packet::TakePacket,
    matching::match_order,
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick},
    state::MarketState,
    types::SideMarker,
};

pub fn ix_take<S: SideMarker>(
    market_state: &mut MarketState,
    tick_size: QuoteLotsPerBaseUnitPerTick,
    base_lot_size: BaseLotsPerBaseUnit,
    payload: &ArgsBuffer,
    len: usize,
    offset: &mut usize,
) -> Result<(), GoblinError>
where
    S::Lots: From<u64>,
    S::Lots: PartialOrd,
    S::Quote: core::ops::SubAssign,
    S::Opposite: SideMarker,
{
    let packet = TakePacket::<S>::decode(payload, len, offset)?;

    match_order::<S>(
        market_state,
        tick_size,
        base_lot_size,
        packet.num_lots,
        packet.min_lots_to_fill,
        packet.price_limit,
    )?;

    Ok(())
}
