use crate::settlement::{
    global_delta_v3::{Counterparties, GlobalSender},
    ConstZero,
};

pub struct GlobalDeltaV3 {
    pub sender: GlobalSender,
    pub counterparties: Counterparties,
}

impl ConstZero for GlobalDeltaV3 {
    const ZEROED: Self = Self {
        sender: GlobalSender::ZEROED,
        counterparties: Counterparties::ZEROED,
    };
}
