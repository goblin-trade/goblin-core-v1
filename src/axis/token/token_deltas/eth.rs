use crate::{
    axis::{
        token::{
            token_deltas::TokenDeltas, token_global_transfer::ETHTransfers,
            token_index::TokenIndex, ETHStub, ETH,
        },
        update::UpdateMarker,
    },
    goblin_error::GoblinError,
    quantities::{ETHAtoms, UnsidedAtoms, UnsidedDeltaAtomsPerLot},
    settlement::global_delta::{GlobalSender, TokenDelta},
    types::{Address, StoreReader},
};

impl TokenDeltas for ETH {
    type TokenIndex = ETHStub;
    type LocalDeposit = ETHStub;
    type GlobalDeposit = ETHStub;

    type TokenGlobalTransfer = ETHTransfers;

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

    fn update<UM: UpdateMarker>(
        deposit: UnsidedAtoms,
        trader: &Address,
        _token_address: &<Self::TokenIndex as TokenIndex>::TokenAddress,
        _decimals: <Self::TokenIndex as TokenIndex>::StoredDecimals,
    ) -> Result<(), GoblinError> {
        let amount = ETHAtoms::try_from(deposit)?;
        UM::update_eth(trader, &amount)
    }
}
