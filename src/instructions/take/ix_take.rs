use crate::{
    goblin_error::GoblinError,
    input_processor::ArgsBuffer,
    instructions::take::take_packet::TakePacket,
    markets::IndexedMarket,
    matching::{match_order, MatchResult},
    state::MarketState,
    types::SideMarker,
};

pub fn ix_take<S: SideMarker>(
    indexed_market: &IndexedMarket,
    market_state: &mut MarketState,
    payload: &ArgsBuffer,
    len: usize,
    offset: &mut usize,
) -> Result<MatchResult<S>, GoblinError>
where
    S::Lots: From<u64>,
    S::Lots: Default,
    S::Lots: PartialOrd,
    S::Quote: Copy,
    S::Quote: core::ops::Add<Output = S::Quote>,
    S::Quote: core::ops::AddAssign,
    S::Quote: core::ops::Sub<Output = S::Quote>,
    S::Quote: core::ops::SubAssign,
    S::Quote: PartialOrd,
    S::Quote: From<u64>,
    S::Opposite: SideMarker,
    <S::Opposite as SideMarker>::Quote: From<u64>,
    <S::Opposite as SideMarker>::Quote: core::ops::AddAssign,
    <S::Opposite as SideMarker>::Lots: From<u64>,
    <S::Opposite as SideMarker>::Lots: Default,
{
    let packet = TakePacket::<S>::decode(payload, len, offset)?;

    match_order::<S>(
        indexed_market,
        market_state,
        packet.num_lots,
        packet.min_lots_to_fill,
        packet.price_limit,
    )
}
