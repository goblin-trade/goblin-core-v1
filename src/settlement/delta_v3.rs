use crate::{
    axis::{
        market::{LotSizePair, TokenIndexPair},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    settlement::{global_delta_v3::GlobalDeltaV3, local_delta_v3::LocalDeltaV3, ConstZero},
};

static mut DELTA_V3: DeltaV3 = DeltaV3::ZEROED;

pub struct DeltaV3 {
    pub global: GlobalDeltaV3,
    pub local: LocalDeltaV3,
}

impl ConstZero for DeltaV3 {
    const ZEROED: Self = Self {
        global: GlobalDeltaV3::ZEROED,
        local: LocalDeltaV3::ZEROED,
    };
}

impl DeltaV3 {
    pub fn get_static() -> &'static mut Self {
        unsafe { &mut DELTA_V3 }
    }

    pub fn commit_local_delta<B, Q>(
        &mut self,
        token_index_pair: &TokenIndexPair<B, Q>,
        lot_size_pair: &LotSizePair,
    ) -> Result<(), GoblinError>
    where
        B: TokenMarker,
        Q: TokenMarker,
    {
        self.global
            .commit_local_delta::<B, Q>(token_index_pair, lot_size_pair, &self.local)
    }
}
