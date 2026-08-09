use crate::settlement::{
    local_delta::{
        local_take::{LocalTake, TakeCounterparties},
        LocalDeposits, LocalMake,
    },
    ConstDefault,
};

pub struct LocalDelta<'a> {
    pub deposits: LocalDeposits,
    pub take: LocalTake<'a>,
    pub make: LocalMake,
}

impl<'a> LocalDelta<'a> {
    pub const fn new(counterparties: &'a mut TakeCounterparties) -> Self {
        Self {
            deposits: LocalDeposits::DEFAULT,
            take: LocalTake::new(counterparties),
            make: LocalMake::DEFAULT,
        }
    }
}
