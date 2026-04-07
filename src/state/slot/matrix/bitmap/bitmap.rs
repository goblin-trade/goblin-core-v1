use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, Base, Pair, Quote, SamePair},
    state::bitmap::bitmap_index::BitmapIndex,
    types::StoreReader,
};

pub trait Bitmap<F, P0, P1>: PartialEq + Default
where
    F: Clone + Copy + PartialEq + PartialOrd,
    P0: BitmapIndex,
    P1: Clone + Copy + PartialEq + PartialOrd,
{
    fn pos_active(&self, pos: P1) -> bool;

    // fn pos_active_checked(&self, pos: (P0, P1), limits: &SamePair<(P0, P1)>) -> bool {
    //     if Base::closer_to_opposite_pole(pos, Base::get(limits))
    //         && Quote::closer_to_opposite_pole(pos, Quote::get(limits))
    //     {
    //         return false;
    //     }

    //     self.pos_active(pos.1)
    // }

    fn deactivate(&mut self, pos: P1);

    fn bitmap_inactive(&self) -> bool {
        *self == Self::default()
    }

    // fn holds_garbage(pos: P1, pair: &SamePair<P1>) -> bool {
    //     Base::closer_to_opposite_pole(pos, Base::get(pair))
    //         && Base::closer_to_opposite_pole(pos, Quote::get(pair))
    // }
}
