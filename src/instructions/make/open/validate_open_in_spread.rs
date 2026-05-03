use crate::{
    axis::leg::{leg_matcher::LegMatcher, Leg, SamePair},
    goblin_error::GoblinError,
    quantities::{Position, Ticks},
    require,
    types::{StoreReader, Tuple},
};

pub fn validate_open_in_spread<In>(
    last_positions: &mut SamePair<Position>,
    position_2: Position,
) -> Result<(), GoblinError>
where
    In: LegMatcher,
    In::Opposite: StoreReader<Tuple<Position, Position, Leg>, Result = Position>,
{
    // order and opposite last position cannot lie on the same price
    let opposite_last_position = In::Opposite::get(last_positions);
    let price = Ticks::from(position_2);
    let opposite_last_price = Ticks::from(opposite_last_position);
    require!(price != opposite_last_price, GoblinError::InvalidOpenPrice);

    // Update last position in market state
    let last_position_mut = In::Opposite::get_leg_mut(last_positions);
    *last_position_mut = position_2;

    Ok(())
}
