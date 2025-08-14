use crate::{
    goblin_error::GoblinError,
    input_processor::ArgsBuffer,
    instructions::take::take_packet::TakePacket,
    matching::match_order,
    quantities::{BaseLots, Ticks},
    state::MarketState,
    types::{Side, SideMarker},
};

pub fn ix_take<S: SideMarker>(
    market_state: &mut MarketState,
    get_quote: impl Fn(BaseLots, Ticks) -> S::Quote,
    payload: &ArgsBuffer,
    len: usize,
    offset: &mut usize,
) -> Result<(), GoblinError>
where
    S::Quote: From<u64>,
    S::Quote: PartialOrd,
    S::Quote: core::ops::SubAssign,
    S::Opposite: SideMarker,
{
    let packet = TakePacket::<S>::decode(payload, len, offset)?;

    match_order::<S>(
        market_state,
        packet.num_lots,
        packet.min_lots_to_fill,
        packet.price_limit,
        get_quote,
    )?;
    Ok(())
}
