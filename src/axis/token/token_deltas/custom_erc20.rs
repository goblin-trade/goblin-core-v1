use crate::{
    axis::{
        token::{
            token_deltas::TokenDeltas,
            token_index::{CustomERC20Index, TokenIndex},
            CustomERC20, CustomERC20Stub,
        },
        update::UpdateMarker,
    },
    goblin_error::GoblinError,
    quantities::{UnsidedAtoms, UnsidedDeltaAtoms, UnsidedDeltaAtomsPerLot, UnsidedDeltaLots},
    settlement::global_delta::{GlobalSender, TokenDelta, TransferERC20},
    types::{Address, StoreReader},
};

impl TokenDeltas for CustomERC20 {
    type TokenIndex = CustomERC20Index;

    type LocalDeposit = UnsidedDeltaLots;
    type GlobalDeposit = UnsidedDeltaAtoms;

    type TokenMsgTransfer = CustomERC20Stub;

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

    fn update<UM: UpdateMarker>(
        deposit: UnsidedAtoms,
        trader: &Address,
        token_address: &<Self::TokenIndex as TokenIndex>::TokenAddress,
        decimals: <Self::TokenIndex as TokenIndex>::StoredDecimals,
    ) -> Result<(), GoblinError> {
        TransferERC20::<UM>::new(deposit, trader, token_address, decimals).dispatch()
    }
}
