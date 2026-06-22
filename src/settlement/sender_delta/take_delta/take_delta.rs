use crate::settlement::{CheckedOps, ConstZero};

#[derive(Default, Clone, Copy)]
pub struct TakeDelta<I, O>
where
    I: Clone + Copy + ConstZero + CheckedOps,
    O: Clone + Copy + ConstZero + CheckedOps,
{
    pub take_in: I,
    pub take_out: O,
}

impl<I, O> ConstZero for TakeDelta<I, O>
where
    I: Clone + Copy + ConstZero + CheckedOps,
    O: Clone + Copy + ConstZero + CheckedOps,
{
    const ZEROED: Self = Self {
        take_in: I::ZEROED,
        take_out: O::ZEROED,
    };
}

impl<I, O> CheckedOps for TakeDelta<I, O>
where
    I: Clone + Copy + ConstZero + CheckedOps,
    O: Clone + Copy + ConstZero + CheckedOps,
{
    fn checked_add(self, rhs: Self) -> Option<Self> {
        Some(Self {
            take_in: self.take_in.checked_add(rhs.take_in)?,
            take_out: self.take_out.checked_add(rhs.take_out)?,
        })
    }

    fn checked_sub(self, rhs: Self) -> Option<Self> {
        Some(Self {
            take_in: self.take_in.checked_sub(rhs.take_in)?,
            take_out: self.take_out.checked_sub(rhs.take_out)?,
        })
    }
}
