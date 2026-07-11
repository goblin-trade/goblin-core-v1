use crate::{
    axis::{
        token::{
            token_global_transfer::TokenGlobalTransfer, token_index::TokenIndex,
            token_marker::TokenMarker,
        },
        update::UpdateMarker,
    },
    goblin_error::GoblinError,
    input_processor::Decodable,
    quantities::{RawAtoms, UnsidedAtoms, UnsidedDeltaAtoms, UnsidedDeltaAtomsPerLot},
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
{
    /// Index to lookup token address
    type TokenIndex: TokenIndex;

    /// Pending deposit amount in local namespace
    type LocalDeposit: Clone + Copy + Default + Decodable + ConstZero + CheckedOps;

    type TokenGlobalTransfer: TokenGlobalTransfer;

    /// Pending deposit amount in global namespace
    type GlobalDeposit: Clone
        + Copy
        + Default
        + Decodable
        + ConstZero
        + CheckedOps
        + Into<UnsidedDeltaAtoms>;

    fn get_global_deposit(
        local_deposit: Self::LocalDeposit,
        atoms_per_lot: UnsidedDeltaAtomsPerLot,
    ) -> Self::GlobalDeposit;

    fn get_global_token_delta(
        token_index: Self::TokenIndex,
        global_sender: &mut GlobalSender,
    ) -> &mut TokenDelta<Self>
    where
        Self: TokenMarker;

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
