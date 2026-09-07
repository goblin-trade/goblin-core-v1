use crate::{quantities::DerivedPosition, state::bitmap::Bitmap};

/// Bitmap-index pair with convenience functions to activate and deactivate bits
pub struct BitmapUpdater<'a, const BITS: u16, const INNER_BITS: u16> {
    pub bitmap: &'a mut Bitmap<BITS, INNER_BITS>,
    pub pos: DerivedPosition<u8, INNER_BITS>,
}

impl<'a, const BITS: u16, const INNER_BITS: u16> BitmapUpdater<'a, BITS, INNER_BITS> {
    pub fn activate(&mut self) {
        self.bitmap.activate(self.pos);
    }

    pub fn deactivate(&mut self) {
        self.bitmap.deactivate(self.pos);
    }
}
