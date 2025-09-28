use crate::types::{Base, Quote};

/// A generic container for the base and quote sides of a market.
///
/// # Example
///
/// Define a pair of lot sizes and access one leg generically:
///
/// ```rs
/// pub type LotSizePair = Pair<BaseLotsPerBaseUnit, QuoteLotsPerQuoteUnit>;
///
/// fn use_lot_size<In>(legs: &LotSizePair)
/// where
///     In: LegMarker + PairAccessor<BaseLotsPerBaseUnit, QuoteLotsPerQuoteUnit, Result = In::LotsPerUnit>,
/// {
///     let lot_size = In::get_leg(legs);
///     assert!(In::lots_per_unit_valid(*lot_size));
/// }
/// ```
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Pair<B, Q> {
    pub base: B,
    pub quote: Q,
}

/// Trait to generically access one leg of the pair given the generic <L: LegMarker>
pub trait PairAccessor<B, Q> {
    type Result;

    fn get_leg(pair: &Pair<B, Q>) -> &Self::Result;
    fn get_leg_mut(pair: &mut Pair<B, Q>) -> &mut Self::Result;
}

impl<B, Q> PairAccessor<B, Q> for Base {
    type Result = B;

    fn get_leg(pair: &Pair<B, Q>) -> &Self::Result {
        &pair.base
    }

    fn get_leg_mut(pair: &mut Pair<B, Q>) -> &mut Self::Result {
        &mut pair.base
    }
}

impl<B, Q> PairAccessor<B, Q> for Quote {
    type Result = Q;

    fn get_leg(pair: &Pair<B, Q>) -> &Self::Result {
        &pair.quote
    }

    fn get_leg_mut(pair: &mut Pair<B, Q>) -> &mut Self::Result {
        &mut pair.quote
    }
}
