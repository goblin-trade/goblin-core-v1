/// Trait representing a bitmap coordinate
pub trait Coordinate {
    type Inner;

    const MIN: Self;
    const MAX: Self;

    fn inner(self) -> Self::Inner;
}
