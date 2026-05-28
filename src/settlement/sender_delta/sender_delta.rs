use crate::settlement::{
    sender_delta::{MakeDelta, TakeDelta},
    ConstZero,
};

#[derive(Default, Clone, Copy)]
pub struct SenderDelta<I, O>
where
    I: Clone + Copy + ConstZero,
    O: Clone + Copy + ConstZero,
{
    pub take: TakeDelta<I, O>,
    pub make: MakeDelta<O>,
}

impl<I, O> ConstZero for SenderDelta<I, O>
where
    I: Clone + Copy + ConstZero,
    O: Clone + Copy + ConstZero,
{
    const ZEROED: Self = Self {
        take: TakeDelta::ZEROED,
        make: MakeDelta::ZEROED,
    };
}
