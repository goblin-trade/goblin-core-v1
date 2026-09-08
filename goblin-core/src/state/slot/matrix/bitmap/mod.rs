pub mod alias;
pub mod bitmap_reader;
pub mod bitmap_updater;

// TODO trait for conditional_read and conditional_write
pub mod bitmap_preimage;
pub mod conditional_read;
pub mod conditional_write;

mod impl_slot_state;

pub use alias::*;
pub use bitmap_preimage::*;
pub use bitmap_reader::*;
pub use bitmap_updater::*;

use crate::quantities::DerivedPosition;

const CLOSED_SENTINEL: [u8; 32] = [0xFF; 32];

#[derive(Default, Clone, Copy, PartialEq)]
pub struct Bitmap<const BITS: u16, const INNER_BITS: u16> {
    pub inner: [u8; 32],
}

impl<const BITS: u16, const INNER_BITS: u16> Bitmap<BITS, INNER_BITS> {
    pub fn is_closed(&self) -> bool {
        self.inner == CLOSED_SENTINEL
    }

    pub fn is_empty(&self) -> bool {
        *self == Self::default()
    }

    pub fn is_active(&self) -> bool {
        !self.is_empty() && !self.is_closed()
    }

    pub fn close_with_sentinel(&mut self) {
        self.inner = CLOSED_SENTINEL
    }

    pub fn index_active(&self, pos: DerivedPosition<u8, INNER_BITS>) -> bool {
        let byte = self.inner[pos.byte_index()];
        let mask = 1 << pos.bit_index();

        (byte & mask) != 0
    }

    pub fn deactivate(&mut self, pos: DerivedPosition<u8, INNER_BITS>) {
        // mask with 0 at target bit, 1 elsewhere
        let mask = !(1u8 << pos.bit_index());
        self.inner[pos.byte_index()] &= mask;
    }

    pub fn activate(&mut self, pos: DerivedPosition<u8, INNER_BITS>) {
        // OR with 1 to turn on the bit
        let mask = 1u8 << pos.bit_index();
        self.inner[pos.byte_index()] |= mask;
    }
}
