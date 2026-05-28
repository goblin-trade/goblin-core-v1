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
