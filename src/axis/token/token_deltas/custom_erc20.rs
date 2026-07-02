use crate::{
    axis::token::{token_deltas::TokenDeltas, token_index::CustomERC20Index, CustomERC20},
    quantities::{UnsidedDeltaAtoms, UnsidedDeltaAtomsPerLot, UnsidedDeltaLots},
    settlement::global_delta::{GlobalSender, TokenDelta},
    types::StoreReader,
};

impl TokenDeltas for CustomERC20 {
    type TokenIndex = CustomERC20Index;

    type LocalDeposit = UnsidedDeltaLots;
    type GlobalDeposit = UnsidedDeltaAtoms;

    fn get_global_deposit(
        local_deposit: Self::LocalDeposit,
        atoms_per_lot: UnsidedDeltaAtomsPerLot,
    ) -> Self::GlobalDeposit {
        local_deposit * atoms_per_lot
    }

    fn get_global_token_delta(
        token_index: Self::TokenIndex,
        global_sender: &mut GlobalSender,
    ) -> &mut TokenDelta<Self> {
        let list = Self::get_leg_mut(global_sender);
        let token_delta = &mut list[token_index.0];
        token_delta
    }
}
