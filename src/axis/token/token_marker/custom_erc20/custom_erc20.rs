use crate::{
    axis::{
        token::{
            token_index::{CustomERC20Index, CustomERC20List, TokenData, TokenIndex},
            token_marker::{custom_erc20::CustomERC20Deltas, TokenMarker},
            CustomERC20, CustomERC20Stub,
        },
        update::UpdateMarker,
    },
    goblin_error::GoblinError,
    quantities::{UnsidedAtoms, UnsidedDeltaAtoms, UnsidedDeltaAtomsPerLot, UnsidedDeltaLots},
    settlement::global_delta::TransferERC20,
    types::Address,
};

impl TokenMarker for CustomERC20 {
    const DISCRIMINATOR: u8 = 2;

    type TokenIndex = CustomERC20Index;

    type LocalDeposit = UnsidedDeltaLots;
    type GlobalDeposit = UnsidedDeltaAtoms;

    type TokenMsgTransfer = CustomERC20Stub;

    type SenderDelta = CustomERC20Deltas;
    type DataList = CustomERC20List<'static>;

    fn token_index_data_iter(
        custom_erc20_list: CustomERC20List,
    ) -> impl Iterator<Item = (Self::TokenIndex, TokenData<Self>)> {
        custom_erc20_list
            .inner
            .iter()
            .enumerate()
            .map(|(i, data)| (CustomERC20Index::from(i), *data))
    }

    fn get_global_deposit(
        local_deposit: Self::LocalDeposit,
        atoms_per_lot: UnsidedDeltaAtomsPerLot,
    ) -> Self::GlobalDeposit {
        local_deposit * atoms_per_lot
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
