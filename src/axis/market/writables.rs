use crate::{settlement::local_delta_v3::LocalDeltaV3, state::MarketState};

pub struct Writables<'a> {
    pub local_delta: &'a mut LocalDeltaV3,
    pub market_state: &'a mut MarketState,
}
