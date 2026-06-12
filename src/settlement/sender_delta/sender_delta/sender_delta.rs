use crate::settlement::{
    sender_delta::{MakeDelta, TakeDelta},
    CheckedAdd, ConstZero,
};

#[derive(Default, Clone, Copy)]
pub struct SenderDelta<I, O>
where
    I: Clone + Copy + ConstZero + CheckedAdd,
    O: Clone + Copy + ConstZero + CheckedAdd,
{
    pub take: TakeDelta<I, O>,
    pub make: MakeDelta<O>,
}

impl<I, O> ConstZero for SenderDelta<I, O>
where
    I: Clone + Copy + ConstZero + CheckedAdd,
    O: Clone + Copy + ConstZero + CheckedAdd,
{
    const ZEROED: Self = Self {
        take: TakeDelta::ZEROED,
        make: MakeDelta::ZEROED,
    };
}

impl<I, O> CheckedAdd for SenderDelta<I, O>
where
    I: Clone + Copy + ConstZero + CheckedAdd,
    O: Clone + Copy + ConstZero + CheckedAdd,
{
    fn checked_add(self, rhs: Self) -> Option<Self> {
        Some(Self {
            take: self.take.checked_add(rhs.take)?,
            make: self.make.checked_add(rhs.make)?,
        })
    }
}
