use crate::settlement::{
    sender_delta::{MakeDelta, TakeDelta},
    CheckedOps, ConstZero,
};

#[derive(Default, Clone, Copy)]
pub struct SenderDelta<I, O>
where
    I: Clone + Copy + ConstZero + CheckedOps,
    O: Clone + Copy + ConstZero + CheckedOps,
{
    pub take: TakeDelta<I, O>,
    pub make: MakeDelta<O>,
}

impl<I, O> ConstZero for SenderDelta<I, O>
where
    I: Clone + Copy + ConstZero + CheckedOps,
    O: Clone + Copy + ConstZero + CheckedOps,
{
    const ZEROED: Self = Self {
        take: TakeDelta::ZEROED,
        make: MakeDelta::ZEROED,
    };
}

impl<I, O> CheckedOps for SenderDelta<I, O>
where
    I: Clone + Copy + ConstZero + CheckedOps,
    O: Clone + Copy + ConstZero + CheckedOps,
{
    fn checked_add(self, rhs: Self) -> Option<Self> {
        Some(Self {
            take: self.take.checked_add(rhs.take)?,
            make: self.make.checked_add(rhs.make)?,
        })
    }

    fn checked_sub(self, rhs: Self) -> Option<Self> {
        Some(Self {
            take: self.take.checked_sub(rhs.take)?,
            make: self.make.checked_sub(rhs.make)?,
        })
    }
}
