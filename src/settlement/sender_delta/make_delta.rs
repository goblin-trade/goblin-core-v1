use crate::settlement::ConstZero;

#[derive(Default, Clone, Copy)]
pub struct MakeDelta<O>
where
    O: Clone + Copy + ConstZero,
{
    pub increase: O,
    pub reduce: O,
}

impl<O> ConstZero for MakeDelta<O>
where
    O: Clone + Copy + ConstZero,
{
    const ZEROED: Self = Self {
        increase: O::ZEROED,
        reduce: O::ZEROED,
    };
}
