use crate::settlement::{global_delta::GlobalDelta, local_delta::LocalDelta};

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
}
