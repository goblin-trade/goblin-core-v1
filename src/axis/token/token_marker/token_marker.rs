use core::ops::{Index, IndexMut};

use crate::{
    axis::{
        token::{
            token_list::{custom_erc20::CustomERC20List, TokenList},
            token_marker::TokenData,
            token_quantity::TokenQuantity,
        },
        update::UpdateMarker,
    },
    goblin_error::GoblinError,
    input_processor::MsgTransfers,
    quantities::{UnsidedAtoms, UnsidedDeltaAtomsPerLot},
    settlement::{
        global_delta::{CounterpartyMap, CounterpartyTriple, GlobalSender, TokenDelta},
        local_delta::DepositTriple,
    },
    types::{Address, StoreReader},
};

pub trait TokenMarker:
    Clone
    + Copy
    + PartialEq
    + 'static
    + TokenQuantity
    + TokenList
    + StoreReader<DepositTriple, Result = Self::LocalDeposit>
    + StoreReader<CounterpartyTriple, Result = CounterpartyMap<Self>>
    + StoreReader<MsgTransfers, Result = Self::TokenMsgTransfer>
    + StoreReader<GlobalSender, Result = Self::SenderDeltaList>
{
    fn get_global_deposit(
        local_deposit: Self::LocalDeposit,
        atoms_per_lot: UnsidedDeltaAtomsPerLot,
    ) -> Self::GlobalDeposit;

    // this gives a clean implementation for ETH
    //
    // However we don't want to duplicate decimal matching for hardcoded and custom ERC20
    fn update<UM: UpdateMarker>(
        deposit: UnsidedAtoms,
        trader: &Address,
        token_address: &Self::TokenAddress,
        decimals: Self::StoredDecimals,
    ) -> Result<(), GoblinError>;

    ////////////////////////

    /// Try to obtain stored decimals
    ///
    /// * ETH: Stub value
    /// * Hardcoded: Use the hardcoded decimals
    /// * Custom: Read from Hostio
    ///
    fn get_stored_decimals(
        token_data: &TokenData<Self>,
    ) -> Result<Self::StoredDecimals, GoblinError>;

    fn get_data_list<'a>(custom_erc20_list: CustomERC20List<'a>) -> Self::DataList<'a>;
}
