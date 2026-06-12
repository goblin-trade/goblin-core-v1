use crate::{
    axis::update::SameUpdatePair,
    settlement::{CheckedAdd, ConstZero},
};

pub type MakeDelta<O> = SameUpdatePair<O>;

impl<O> ConstZero for MakeDelta<O>
where
    O: Clone + Copy + ConstZero,
{
    const ZEROED: Self = SameUpdatePair::new(O::ZEROED, O::ZEROED);
}

impl<O> CheckedAdd for MakeDelta<O>
where
    O: Clone + Copy + ConstZero + CheckedAdd,
{
    fn checked_add(self, rhs: Self) -> Option<Self> {
        Some(Self::new(
            self.0.checked_add(rhs.0)?,
            self.1.checked_add(rhs.1)?,
        ))
    }
}
