use crate::settlement::{
    local_delta::{DeltaLotsPair, LocalMake},
    ConstZero,
};

impl ConstZero for LocalMake {
    const ZEROED: Self = Self {
        inner: DeltaLotsPair::ZEROED,
    };
}
