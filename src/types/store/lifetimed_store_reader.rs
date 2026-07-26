/// Variant of StoreReader with lifetime. It allows us to read TokenDataTriple which stores references.
///
/// pub trait LegReader: LegMath
///     + for<'a> StoreReader<'a, Tuple<BaseLotsPerBaseUnit, QuoteLotsPerQuoteUnit, Leg>, Result = Self::LotsPerUnit>
///
pub trait LifetimedStoreReader<'a, S> {
    type Result: Clone + Copy;

    fn get_lifetimed(store: &'a S) -> Self::Result;
}
