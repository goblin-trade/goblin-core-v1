use crate::settlement::local_delta_v3::{DeltaLotsPair, TakeCounterparty};

pub struct LocalDeltaV3 {
    pub take: DeltaLotsPair,
    pub take_counterparty: TakeCounterparty,
    pub make: DeltaLotsPair,
}
