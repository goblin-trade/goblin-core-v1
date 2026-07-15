use core::ops::{Index, IndexMut};

use crate::{
    axis::{
        token::{
            token_marker::CustomERC20List, token_marker::TokenData,
            token_msg_transfer::TokenMsgTransfer,
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
    + StoreReader<DepositTriple, Result = Self::LocalDeposit>
    + StoreReader<CounterpartyTriple, Result = CounterpartyMap<Self>>
    + StoreReader<MsgTransfers, Result = Self::TokenMsgTransfer>
    + StoreReader<GlobalSender, Result = Self::SenderDeltaList>
{
    const DISCRIMINATOR: u8;

    /// Index to lookup token address
    type TokenIndex: Clone + Copy + Decodable + ConstZero + PartialEq;

    /// Pending deposit amount in local namespace
    type LocalDeposit: Clone + Copy + Default + Decodable + ConstZero + CheckedOps;

    type TokenMsgTransfer: TokenMsgTransfer;

    /// Pending deposit amount in global namespace
    type GlobalDeposit: Clone
        + Copy
        + Default
        + Decodable
        + ConstZero
        + CheckedOps
        + Into<UnsidedDeltaAtoms>;

    type SenderDeltaList: Clone
        + Copy
        + Index<Self::TokenIndex, Output = TokenDelta<Self>>
        + IndexMut<Self::TokenIndex>
        + IntoIterator<Item = TokenDelta<Self>>;

    type DataList<'a>: Sized + Index<Self::TokenIndex, Output = TokenData<Self>>;

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

    type TokenAddress: Clone + Copy + Sized + Default;

    /// Decimals hardcoded in the smart contract
    type HardcodedDecimals: Clone + Copy;

    /// Decimals stored in `Store`
    /// Decimals are stored as u8 for ERC20 tokens but not for ETH
    type StoredDecimals: Clone + Copy + Into<u8>;

    /// Try to obtain stored decimals
    ///
    /// * ETH: Stub value
    /// * Hardcoded: Use the hardcoded decimals
    /// * Custom: Read from Hostio
    ///
    fn get_stored_decimals(
        token_data: &TokenData<Self>,
    ) -> Result<Self::StoredDecimals, GoblinError>;

    /// Padding to pad `Store` to 32 bytes
    /// ERC20 store has less padding to accomodate `decimals: u8`
    type StoredPadding: Clone + Copy;

    fn get_data_list<'a>(custom_erc20_list: CustomERC20List<'a>) -> Self::DataList<'a>;

    fn token_index_data_iter(
        custom_erc20_list: CustomERC20List,
    ) -> impl Iterator<Item = (Self::TokenIndex, TokenData<Self>)>;
}
