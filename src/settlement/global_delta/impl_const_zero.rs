use crate::settlement::{
    global_delta::{CounterpartyTriple, GlobalDelta, GlobalSender},
    ConstZero,
};

impl ConstZero for GlobalDelta {
    const ZEROED: Self = Self {
        sender: GlobalSender::ZEROED,
        counterparties: CounterpartyTriple::ZEROED,
    };
}
