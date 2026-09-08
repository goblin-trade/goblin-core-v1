mod impl_local_delta_store;

use goblin_macros::ConstDefault;

use crate::{axis::leg::SamePair, quantities::UnsidedDeltaLots};

#[derive(ConstDefault, Clone, Copy)]
pub struct LocalTake {
    pub inner: SamePair<UnsidedDeltaLots>,
}
