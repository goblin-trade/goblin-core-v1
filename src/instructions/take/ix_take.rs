use crate::{
    goblin_error::GoblinError, input_processor::ArgsBuffer,
    instructions::take::take_packet::TakePacket, matching::match_order, state::MarketState,
    types::Side,
};

pub fn ix_take(
    market_state: &mut MarketState,
    side: Side,
    payload: &ArgsBuffer,
    len: usize,
    offset: &mut usize,
) -> Result<(), GoblinError> {
    let packet = TakePacket::decode(side, payload, len, offset)?;
    // match_order(
    //     market_state,
    //     side,
    //     packet.num_lots,
    //     packet.min_lots_to_fill,
    //     packet.price_limit,
    // )?;

    Ok(())
}
