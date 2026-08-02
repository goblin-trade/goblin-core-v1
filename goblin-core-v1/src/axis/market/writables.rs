use crate::{settlement::local_delta::LocalDelta, state::MarketState};

pub struct Writables<'a> {
    pub local_delta: &'a mut LocalDelta,
    pub market_state: &'a mut MarketState,
}
