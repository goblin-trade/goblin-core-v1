use goblin_macros::ConstDefault;

use crate::{
    axis::leg::SamePair, quantities::UnsidedDeltaLots, settlement::local_delta::LocalMake,
};

#[derive(ConstDefault, Clone, Copy)]
pub struct LocalSender {
    pub take: SamePair<UnsidedDeltaLots>,
    pub make: LocalMake,
}
