use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base, Quote},
        market::{LotSizePair, TokenIndexPair},
        token::token_delta_manager::TokenDeltaManager,
    },
    goblin_error::GoblinError,
    settlement::{
        global_delta_v3::{Counterparties, GlobalSender},
        local_delta_v3::LocalDeltaV3,
        ConstZero,
    },
    types::StoreReader,
};

pub struct GlobalDeltaV3 {
    pub sender: GlobalSender,
    pub counterparties: Counterparties,
}

impl ConstZero for GlobalDeltaV3 {
    const ZEROED: Self = Self {
        sender: GlobalSender::ZEROED,
        counterparties: Counterparties::ZEROED,
    };
}

impl GlobalDeltaV3 {
    pub fn commit_local_delta<B, Q>(
        &mut self,
        token_index_pair: &TokenIndexPair<B, Q>,
        lot_size_pair: &LotSizePair,
        local_delta: &LocalDeltaV3,
    ) -> Result<(), GoblinError>
    where
        B: TokenDeltaManager,
        Q: TokenDeltaManager,
    {
        let base_token_index = Base::get(token_index_pair);
        let quote_token_index = Quote::get(token_index_pair);

        self.sender
            .commit_side::<Base, B>(base_token_index, lot_size_pair, local_delta)?;
        self.sender
            .commit_side::<Quote, Q>(quote_token_index, lot_size_pair, local_delta)?;

        Ok(())
    }
}
