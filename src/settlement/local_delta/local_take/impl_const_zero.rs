use crate::settlement::{
    local_delta::{
        local_take::{LocalTake, TakeCounterparties},
        DeltaLotsPair,
    },
    ConstZero,
};

impl ConstZero for LocalTake {
    const ZEROED: Self = Self {
        sender: DeltaLotsPair::ZEROED,
        counterparties: TakeCounterparties::ZEROED,
    };
}
