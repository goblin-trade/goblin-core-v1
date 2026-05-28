use crate::settlement::{CheckedAdd, ConstZero};

#[derive(Default, Clone, Copy)]
pub struct TakeDelta<I, O>
where
    I: Clone + Copy + ConstZero + CheckedAdd,
    O: Clone + Copy + ConstZero + CheckedAdd,
{
    pub take_in: I,
    pub take_out: O,
}

impl<I, O> ConstZero for TakeDelta<I, O>
where
    I: Clone + Copy + ConstZero + CheckedAdd,
    O: Clone + Copy + ConstZero + CheckedAdd,
{
    const ZEROED: Self = Self {
        take_in: I::ZEROED,
        take_out: O::ZEROED,
    };
}

impl<I, O> CheckedAdd for TakeDelta<I, O>
where
    I: Clone + Copy + ConstZero + CheckedAdd,
    O: Clone + Copy + ConstZero + CheckedAdd,
{
    fn checked_add(self, rhs: Self) -> Option<Self> {
        Some(Self {
            take_in: self.take_in.checked_add(rhs.take_in)?,
            take_out: self.take_out.checked_add(rhs.take_out)?,
        })
    }
}
