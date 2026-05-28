use crate::settlement::ConstZero;

#[derive(Default, Clone, Copy)]
pub struct TakeDelta<I, O>
where
    I: Clone + Copy + ConstZero,
    O: Clone + Copy + ConstZero,
{
    pub take_in: I,
    pub take_out: O,
}

impl<I, O> ConstZero for TakeDelta<I, O>
where
    I: Clone + Copy + ConstZero,
    O: Clone + Copy + ConstZero,
{
    const ZEROED: Self = Self {
        take_in: I::ZEROED,
        take_out: O::ZEROED,
    };
}
