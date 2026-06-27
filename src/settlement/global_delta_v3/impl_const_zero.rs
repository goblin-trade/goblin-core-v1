use crate::settlement::{
    global_delta_v3::{CounterpartyTriple, GlobalDeltaV3, GlobalSender},
    ConstZero,
};

impl ConstZero for GlobalDeltaV3 {
    const ZEROED: Self = Self {
        sender: GlobalSender::ZEROED,
        counterparties: CounterpartyTriple::ZEROED,
    };
}
