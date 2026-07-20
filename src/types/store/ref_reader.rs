/// Variant of StoreReader with lifetime
///
/// It allows us to read TokenDataTriple which stores references.
///
/// TODO explore unified StoreReader with lifetime later.
/// It will create more complex trait bounds in reader traits, but they remain
/// isolated in these traits
///
/// pub trait LegReader: LegMath
///     + for<'a> StoreReader<'a, Tuple<BaseLotsPerBaseUnit, QuoteLotsPerQuoteUnit, Leg>, Result = Self::LotsPerUnit>
///
pub trait RefReader<'a, S> {
    type Result: Clone + Copy;

    fn get_with_lifetime(store: &'a S) -> Self::Result;
}
