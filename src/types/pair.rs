use crate::types::{Base, Quote, Tuple};

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
///     In: LegMarker + TupleReader<BaseLotsPerBaseUnit, QuoteLotsPerQuoteUnit, (Base, Quote), Result = In::LotsPerUnit>,
/// {
///     let lot_size = In::get_leg(legs);
///     assert!(In::lots_per_unit_valid(*lot_size));
/// }
/// ```

pub type Pair<T0, T1> = Tuple<T0, T1, (Base, Quote)>;
