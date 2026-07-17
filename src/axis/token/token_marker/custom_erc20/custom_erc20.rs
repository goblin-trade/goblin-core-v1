use crate::{
    axis::{
        token::{
            token_marker::TokenData,
            token_marker::{custom_erc20::CustomERC20Deltas, CustomERC20List, TokenMarker},
            CustomERC20,
        },
        update::UpdateMarker,
    },
    goblin_error::GoblinError,
    hostio::erc20_hostio,
    quantities::{UnsidedAtoms, UnsidedDeltaAtomsPerLot},
    settlement::global_delta::TransferERC20,
    types::Address,
};

impl TokenMarker for CustomERC20 {
    type SenderDeltaList = CustomERC20Deltas;
    type DataList<'a> = CustomERC20List<'a>;

    fn get_global_deposit(
        local_deposit: Self::LocalDeposit,
        atoms_per_lot: UnsidedDeltaAtomsPerLot,
    ) -> Self::GlobalDeposit {
        local_deposit * atoms_per_lot
    }

    fn update<UM: UpdateMarker>(
        deposit: UnsidedAtoms,
        trader: &Address,
        token_address: &Self::TokenAddress,
        decimals: Self::StoredDecimals,
    ) -> Result<(), GoblinError> {
        TransferERC20::<UM>::new(deposit, trader, token_address, decimals).dispatch()
    }

    ///////

    fn get_stored_decimals(
        token_data: &TokenData<Self>,
    ) -> Result<Self::StoredDecimals, GoblinError> {
        erc20_hostio::decimals(&token_data.address)
    }

    fn get_data_list<'a>(custom_erc20_list: CustomERC20List<'a>) -> Self::DataList<'a> {
        custom_erc20_list
    }
}
