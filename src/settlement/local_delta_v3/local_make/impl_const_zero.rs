use crate::settlement::{
    local_delta_v3::{DeltaLotsPair, LocalMake},
    ConstZero,
};

impl ConstZero for LocalMake {
    const ZEROED: Self = Self {
        inner: DeltaLotsPair::ZEROED,
    };
}
