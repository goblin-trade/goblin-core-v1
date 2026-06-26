use crate::settlement::{
    local_delta_v3::{local_take::LocalTake, LocalDepositsV3, LocalMake},
    ConstZero,
};

pub struct LocalDeltaV3 {
    pub deposits: LocalDepositsV3,
    pub take: LocalTake,
    pub make: LocalMake,
}

impl ConstZero for LocalDeltaV3 {
    const ZEROED: Self = Self {
        deposits: LocalDepositsV3::ZEROED,
        take: LocalTake::ZEROED,
        make: LocalMake::ZEROED,
    };
}
