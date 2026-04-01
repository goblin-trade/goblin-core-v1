use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, Base, Pair, Quote, SamePair},
    types::StoreReader,
};

pub trait Bitmap<P>: PartialEq + Default
where
    P: Clone + Copy + PartialEq + PartialOrd,
{
    fn pos_active(&self, pos: P) -> bool;

    fn deactivate(&mut self, pos: P);

    fn bitmap_inactive(&self) -> bool {
        *self == Self::default()
    }

    fn holds_garbage(pos: P, pair: &SamePair<P>) -> bool {
        Base::closer_to_opposite_pole(pos, Base::get(pair))
            && Base::closer_to_opposite_pole(pos, Quote::get(pair))
    }
}
