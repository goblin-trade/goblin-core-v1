use crate::{axis::update::SameUpdatePair, settlement::ConstZero};

pub type MakeDelta<O> = SameUpdatePair<O>;

impl<O> ConstZero for MakeDelta<O>
where
    O: Clone + Copy + ConstZero,
{
    const ZEROED: Self = SameUpdatePair::new(O::ZEROED, O::ZEROED);
}
