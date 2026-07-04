use crate::{
    axis::token::{
        token_global_transfer::TokenGlobalTransfer, token_index::TokenIndex,
        token_marker::TokenMarker,
    },
    input_processor::Decodable,
    quantities::{UnsidedDeltaAtoms, UnsidedDeltaAtomsPerLot},
    settlement::{
        global_delta::{CounterpartyMap, CounterpartyTriple, GlobalSender, TokenDelta},
        local_delta::DepositTriple,
        CheckedOps, ConstZero,
    },
    types::StoreReader,
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
}
