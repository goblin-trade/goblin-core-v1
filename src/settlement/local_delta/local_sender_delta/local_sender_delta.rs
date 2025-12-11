use crate::settlement::local_delta::TakerDeltaPair;

/// The sender delta of local namespace
///
/// Values are denominated in MatchingLots. Convert it to TakerTokenUpdate
/// so it can be added to the global delta
pub struct LocalSenderDelta {
    /// The results of matching take orders
    pub taker_delta_pair: TakerDeltaPair,
}

impl LocalSenderDelta {
    pub const fn zero() -> Self {
        Self {
            taker_delta_pair: TakerDeltaPair::zero(),
        }
    }
}
