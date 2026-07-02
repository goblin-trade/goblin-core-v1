use crate::{
    axis::token::{token_deltas::TokenDeltas, token_index::ETHStub, ETH},
    quantities::UnsidedDeltaAtomsPerLot,
    settlement::global_delta::{GlobalSender, TokenDelta},
    types::StoreReader,
};

impl TokenDeltas for ETH {
    type TokenIndex = ETHStub;
    type LocalDeposit = ETHStub;
    type GlobalDeposit = ETHStub;

    fn get_global_deposit(
        _local_deposit: Self::LocalDeposit,
        _atoms_per_lot: UnsidedDeltaAtomsPerLot,
    ) -> Self::GlobalDeposit {
        ETHStub
    }

    fn get_global_token_delta(
        _token_index: Self::TokenIndex,
        global_sender: &mut GlobalSender,
    ) -> &mut TokenDelta<Self> {
        Self::get_leg_mut(global_sender)
    }
}
