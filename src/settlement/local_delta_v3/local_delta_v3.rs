use crate::settlement::local_delta_v3::{DeltaLotsPair, LocalDepositsV3, TakeCounterparty};

pub struct LocalDeltaV3 {
    pub deposits: LocalDepositsV3,
    pub take: DeltaLotsPair,
    pub take_counterparty: TakeCounterparty,
    pub make: DeltaLotsPair,
}
