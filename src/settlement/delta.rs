use crate::settlement::{global_delta::GlobalDelta, local_delta::LocalDelta};

pub struct Delta {
    pub global: GlobalDelta,
    pub local: LocalDelta,
}

impl Delta {
    pub const fn new() -> Self {
        Self {
            global: GlobalDelta::new(),
            local: LocalDelta::new(),
        }
    }
}
