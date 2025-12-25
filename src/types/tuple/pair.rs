use crate::impl_tuple_reader;
use crate::market::MarketVariant;
use crate::token::TokenMarker;
use crate::types::{Base, Quote, Tuple, TupleReader};

// Apply the macro to create implementations for all desired pairs

// impl_tuple_reader!(HardcodedToken, CustomToken);

impl_tuple_reader!(Base, Quote);

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
