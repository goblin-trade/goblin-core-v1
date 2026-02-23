/// Trait representing a bitmap coordinate
pub trait Coordinate {
    type Inner;

    const MIN: Self;
    const MAX: Self;

    fn inner(self) -> Self::Inner;

    /// Return an iterator for remaining values of this coordinate
    /// starting from the current position
    fn iter(self) -> impl Iterator<Item = Self>;

    fn closer_to_centre(self, other: Self) -> bool;
}
