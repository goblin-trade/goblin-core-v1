#[derive(Clone, Copy)]
pub struct Range<C>
where
    C: Clone + Copy + PartialEq,
{
    pub start: C,
    pub limit: C,
}

impl<C> Range<C>
where
    C: Clone + Copy + PartialEq,
{
    /// Whether start equals limit
    ///
    /// For OuterPos and Row, we additionally need to check the top level dimensions.
    ///
    /// We compare OuterPos only if we are on the same OuterBitmapIndex
    pub fn on_limit(&self) -> bool {
        self.start == self.limit
    }
}
