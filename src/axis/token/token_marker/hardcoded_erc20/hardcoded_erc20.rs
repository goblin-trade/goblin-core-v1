use crate::{
    axis::{
        token::{
            token_list::{custom_erc20::CustomERC20List, hardcoded_erc20::HARDCODED_ERC20_LIST},
            token_marker::{TokenData, TokenMarker},
            HardcodedERC20,
        },
        update::UpdateMarker,
    },
    goblin_error::GoblinError,
    quantities::{UnsidedAtoms, UnsidedDeltaAtomsPerLot},
    settlement::global_delta::TransferERC20,
    types::Address,
};

impl TokenMarker for HardcodedERC20 {
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

    ///////////

    fn get_stored_decimals(
        token_data: &TokenData<Self>,
    ) -> Result<Self::StoredDecimals, GoblinError> {
        Ok(token_data.decimals)
    }

    fn get_data_list<'a>(_custom_erc20_list: CustomERC20List<'a>) -> Self::DataList<'a> {
        &HARDCODED_ERC20_LIST
    }
}
