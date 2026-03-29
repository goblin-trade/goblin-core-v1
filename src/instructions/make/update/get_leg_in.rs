use crate::{
    axis::leg::{Base, LegEnum, Pair, Quote},
    goblin_error::GoblinError,
    matching::bitmap::StoredCoordinates,
    quantities::Ticks,
    types::StoreReader,
};

/// Get the side of an update instruction
///
/// # Convention
///
/// * leg_in represents the input leg from perspective of the taker.
/// * A resting bid has In = Base while a resting ask has In = Quote.
/// * The `In` token represents the token the maker wants to obtain.
///
pub fn get_leg_in(
    price: Ticks,
    last_coordinates: &Pair<StoredCoordinates, StoredCoordinates>,
) -> Result<LegEnum, GoblinError> {
    // Ask- maker wants base token
    let last_price_base_in = Base::get(last_coordinates).price;
    if price >= last_price_base_in {
        return Ok(LegEnum::Base);
    }

    // Bid- maker wants quote token
    let last_price_quote_in = Quote::get(last_coordinates).price;
    if price <= last_price_quote_in {
        return Ok(LegEnum::Quote);
    }

    Err(GoblinError::NoRestingOrder)
}
