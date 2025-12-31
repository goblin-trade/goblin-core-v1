use crate::settlement::{global_delta::GlobalDelta, local_delta::LocalDelta};

/// Global static mut Delta, initially zero filled.
///
/// `static mut` allows us to take advantage of the fact that lienar memory is zero filled.
/// We get an empty starting buffer without the cost of zeroing.
static mut DELTA: Delta = Delta::zero();

pub struct Delta {
    pub global: GlobalDelta,
    pub local: LocalDelta,
}

impl Delta {
    pub const fn zero() -> Self {
        Self {
            global: GlobalDelta::zero(),
            local: LocalDelta::zero(),
        }
    }

    pub fn get_static() -> &'static mut Self {
        unsafe { &mut DELTA }
    }
}
