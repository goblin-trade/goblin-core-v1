use crate::{
    axis::{
        occupancy::{
            occupancy_marker::OccupancyMarker,
            OccupancyEnum::{Occupied, Vacant},
        },
        update::{
            UpdateEnum::{Decrease, Increase},
            UpdateMarker,
        },
    },
    quantities::DerivedPosition,
    state::bitmap::Bitmap,
};

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

    pub fn update<OM: OccupancyMarker, UM: UpdateMarker>(&mut self, resting_order_closed: bool) {
        match (OM::VARIANT, UM::VARIANT) {
            (Vacant, Increase) => {
                // illegal, unreachable
            }
            (Vacant, Decrease) => {
                self.activate();
            }
            (Occupied, Increase) => {
                // Do nothing, already active
            }
            (Occupied, Decrease) => {
                if resting_order_closed {
                    self.deactivate();
                }
            }
        }
    }
}
