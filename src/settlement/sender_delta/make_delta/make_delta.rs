use crate::{
    axis::update::SameUpdatePair,
    settlement::{CheckedOps, ConstZero},
};

pub type MakeDelta<O> = SameUpdatePair<O>;

impl<O> ConstZero for MakeDelta<O>
where
    O: Clone + Copy + ConstZero,
{
    const ZEROED: Self = SameUpdatePair::new(O::ZEROED, O::ZEROED);
}

impl<O> CheckedOps for MakeDelta<O>
where
    O: Clone + Copy + ConstZero + CheckedOps,
{
    fn checked_add(self, rhs: Self) -> Option<Self> {
        Some(Self::new(
            self.0.checked_add(rhs.0)?,
            self.1.checked_add(rhs.1)?,
        ))
    }

    fn checked_sub(self, rhs: Self) -> Option<Self> {
        Some(Self::new(
            self.0.checked_sub(rhs.0)?,
            self.1.checked_sub(rhs.1)?,
        ))
    }
}
