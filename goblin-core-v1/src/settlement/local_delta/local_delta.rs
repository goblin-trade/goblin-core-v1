use crate::settlement::{
    local_delta::{local_take::LocalTake, LocalDeposits, LocalMake},
    ConstZero,
};

pub struct LocalDelta {
    pub deposits: LocalDeposits,
    pub take: LocalTake,
    pub make: LocalMake,
}

impl ConstZero for LocalDelta {
    const ZEROED: Self = Self {
        deposits: LocalDeposits::ZEROED,
        take: LocalTake::ZEROED,
        make: LocalMake::ZEROED,
    };
}
