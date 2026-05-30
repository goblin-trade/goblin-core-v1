use crate::{axis::update::SameUpdatePair, settlement::ConstZero};

pub type MakeDelta<O>
where
    O: Clone + Copy + ConstZero,
= SameUpdatePair<O>;

// //  TODO convert into Tuple over Increase & Decrease markers
// #[derive(Default, Clone, Copy)]
// pub struct MakeDelta<O>
// where
//     O: Clone + Copy + ConstZero,
// {
//     pub increase: O,
//     pub decrease: O,
// }

impl<O> ConstZero for MakeDelta<O>
where
    O: Clone + Copy + ConstZero,
{
    const ZEROED: Self = SameUpdatePair::new(O::ZEROED, O::ZEROED);
}
