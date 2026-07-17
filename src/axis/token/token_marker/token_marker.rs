use core::ops::{Index, IndexMut};

use crate::{
    axis::{
        token::{
            token_marker::{CustomERC20List, TokenData},
            token_msg_transfer::TokenMsgTransfer,
            token_quantity::TokenQuantity,
        },
        update::UpdateMarker,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, MsgTransfers},
    quantities::{UnsidedAtoms, UnsidedDeltaAtoms, UnsidedDeltaAtomsPerLot},
    settlement::{
        global_delta::{CounterpartyMap, CounterpartyTriple, GlobalSender, TokenDelta},
        local_delta::DepositTriple,
        CheckedOps, ConstZero,
    },
    types::{Address, StoreReader},
};

pub trait TokenMarker:
    Clone
    + Copy
    + PartialEq
    + 'static
    + TokenQuantity
    + StoreReader<DepositTriple, Result = Self::LocalDeposit>
    + StoreReader<CounterpartyTriple, Result = CounterpartyMap<Self>>
    + StoreReader<MsgTransfers, Result = Self::TokenMsgTransfer>
    + StoreReader<GlobalSender, Result = Self::SenderDeltaList>
{
    type SenderDeltaList: Clone
        + Copy
        + Index<Self::TokenIndex, Output = TokenDelta<Self>>
        + IndexMut<Self::TokenIndex>
        + IntoIterator<Item = TokenDelta<Self>>;

    type DataList<'a>: Sized
        + Index<Self::TokenIndex, Output = TokenData<Self>>
        + IntoIterator<Item = TokenData<Self>>;

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
