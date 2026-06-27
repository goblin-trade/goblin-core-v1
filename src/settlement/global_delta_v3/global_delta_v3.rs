use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base, Quote, SamePair},
        market::{LotSizePair, TokenIndexPair},
        token::token_delta_manager::TokenDeltaManager,
    },
    goblin_error::GoblinError,
    quantities::UnsidedDeltaAtomsPerLot,
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

        let atoms_per_lot_pair = SamePair::<UnsidedDeltaAtomsPerLot>::try_from(lot_size_pair)?;
        let base_atoms_per_lot = Base::get(&atoms_per_lot_pair);
        let quote_atoms_per_lot = Quote::get(&atoms_per_lot_pair);

        self.sender
            .commit_side::<B, Base>(base_token_index, base_atoms_per_lot, local_delta)?;
        self.sender
            .commit_side::<Q, Quote>(quote_token_index, quote_atoms_per_lot, local_delta)?;

        // for (maker, delta_pair) in local_delta.take.counterparties.iter() {}

        Ok(())
    }
}
