use core::ops::{Index, IndexMut};

use crate::{
    axis::{
        token::{
            token_index::{CustomERC20List, TokenData, TokenIndex},
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

pub trait TokenDeltas:
    Clone
    + Copy
    + PartialEq
    + StoreReader<DepositTriple, Result = Self::LocalDeposit>
    + StoreReader<CounterpartyTriple, Result = CounterpartyMap<Self>>
    + StoreReader<MsgTransfers, Result = Self::TokenMsgTransfer>
    + StoreReader<GlobalSender, Result = Self::SenderDelta>
{
    /// Index to lookup token address
    type TokenIndex: TokenIndex;

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

    type SenderDelta: Clone
        + Copy
        + Index<Self::TokenIndex, Output = TokenDelta<Self>>
        + IndexMut<Self::TokenIndex>;

    fn token_index_data_iter(
        custom_erc20_list: CustomERC20List,
    ) -> impl Iterator<Item = (Self::TokenIndex, TokenData<Self>)>;

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
        token_address: &<Self::TokenIndex as TokenIndex>::TokenAddress,
        decimals: <Self::TokenIndex as TokenIndex>::StoredDecimals,
    ) -> Result<(), GoblinError>;
}
