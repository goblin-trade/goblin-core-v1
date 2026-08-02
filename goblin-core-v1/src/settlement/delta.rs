use crate::{
    axis::market::{token_pair::TokenPair, LotSizePair, TokenIndexPair},
    goblin_error::GoblinError,
    settlement::{global_delta::GlobalDelta, local_delta::LocalDelta, ConstZero},
};

static mut DELTA: Delta = Delta::ZEROED;

pub struct Delta {
    pub global: GlobalDelta,
    pub local: LocalDelta,
}

impl ConstZero for Delta {
    const ZEROED: Self = Self {
        global: GlobalDelta::ZEROED,
        local: LocalDelta::ZEROED,
    };
}

impl Delta {
    pub fn get_static() -> &'static mut Self {
        unsafe { &mut DELTA }
    }

    pub fn commit_local_delta<TP: TokenPair>(
        &mut self,
        token_index_pair: &TokenIndexPair<TP>,
        lot_size_pair: &LotSizePair,
    ) -> Result<(), GoblinError> {
        self.global
            .commit_local_delta::<TP>(token_index_pair, lot_size_pair, &self.local)
    }
}
